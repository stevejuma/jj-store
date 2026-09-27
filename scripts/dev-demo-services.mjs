import { createHash, createHmac, randomBytes } from "node:crypto";
import { execFile } from "node:child_process";
import { readFile } from "node:fs/promises";
import { request } from "node:http";
import { join } from "node:path";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

export function credential() {
  return `jst1_${randomBytes(16).toString("hex")}_${randomBytes(32).toString("hex")}`;
}

export async function execute(command, args, options = {}) {
  const { stdout = "", stderr = "" } = await execFileAsync(command, args, {
    cwd: options.cwd,
    env: { ...process.env, ...options.env },
    maxBuffer: 16 * 1024 * 1024,
  });
  return { stdout: stdout.trim(), stderr: stderr.trim() };
}

async function ownedContainerExists({ containerName, owner, cwd }) {
  try {
    const { stdout } = await execute(
      "docker",
      [
        "inspect",
        containerName,
        "--format",
        '{{ index .Config.Labels "org.lapismd.jj-store.dev" }}',
      ],
      { cwd },
    );
    if (stdout !== owner) {
      throw new Error(
        `refusing to remove unowned Docker container ${containerName}`,
      );
    }
    return true;
  } catch (error) {
    if (
      String(error.stderr ?? error.message)
        .toLowerCase()
        .includes("no such object")
    )
      return false;
    throw error;
  }
}

export async function stopPrevious({
  containerName,
  owner,
  runtimeFile,
  runtimeRoot,
}) {
  try {
    const runtime = JSON.parse(await readFile(runtimeFile, "utf8"));
    if (Number.isInteger(runtime.serverPid)) {
      try {
        const { stdout } = await execute(
          "ps",
          ["-p", String(runtime.serverPid), "-o", "command="],
          { cwd: owner },
        );
        if (!stdout.includes(join(runtimeRoot, "server.toml"))) {
          throw new Error(
            `refusing to stop unrelated process ${runtime.serverPid}`,
          );
        }
        process.kill(runtime.serverPid, "SIGTERM");
        for (let attempt = 0; attempt < 50; attempt += 1) {
          try {
            process.kill(runtime.serverPid, 0);
            await new Promise((resolve) => setTimeout(resolve, 100));
          } catch {
            break;
          }
        }
      } catch (error) {
        if (
          !String(error.stderr ?? error.message)
            .toLowerCase()
            .includes("no such process")
        )
          throw error;
      }
    }
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
  }
  if (await ownedContainerExists({ containerName, owner, cwd: owner })) {
    await execute("docker", ["rm", "--force", "--volumes", containerName], {
      cwd: owner,
    });
  }
}

function signedHeaders(url, accessKey, secretKey) {
  const stamp = new Date().toISOString().replace(/[:-]|\.\d{3}/g, "");
  const date = stamp.slice(0, 8);
  const payload = createHash("sha256").update("").digest("hex");
  const headers = {
    host: url.host,
    "x-amz-content-sha256": payload,
    "x-amz-date": stamp,
  };
  const names = Object.keys(headers).sort().join(";");
  const canonicalHeaders = Object.entries(headers)
    .sort(([left], [right]) => left.localeCompare(right))
    .map(([name, value]) => `${name}:${value}\n`)
    .join("");
  const canonical = [
    "PUT",
    url.pathname,
    "",
    canonicalHeaders,
    names,
    payload,
  ].join("\n");
  const scope = `${date}/us-east-1/s3/aws4_request`;
  const toSign = [
    "AWS4-HMAC-SHA256",
    stamp,
    scope,
    createHash("sha256").update(canonical).digest("hex"),
  ].join("\n");
  let key = Buffer.from(`AWS4${secretKey}`);
  for (const value of [date, "us-east-1", "s3", "aws4_request"]) {
    key = createHmac("sha256", key).update(value).digest();
  }
  const signature = createHmac("sha256", key).update(toSign).digest("hex");
  return {
    ...headers,
    authorization: `AWS4-HMAC-SHA256 Credential=${accessKey}/${scope}, SignedHeaders=${names}, Signature=${signature}`,
  };
}

export async function createBucket({
  accessKey,
  bucket,
  secretKey,
  storagePort,
}) {
  const url = new URL(`http://127.0.0.1:${storagePort}/${bucket}`);
  for (let attempt = 0; attempt < 100; attempt += 1) {
    const status = await new Promise((resolveStatus) => {
      const operation = request(
        url,
        { method: "PUT", headers: signedHeaders(url, accessKey, secretKey) },
        (response) => {
          response.resume();
          response.on("end", () => resolveStatus(response.statusCode));
        },
      );
      operation.on("error", () => resolveStatus(0));
      operation.end();
    });
    if (status === 200 || status === 409) return;
    await new Promise((resolveWait) => setTimeout(resolveWait, 200));
  }
  throw new Error("RustFS did not accept the development bucket");
}

export async function waitForServer(origin) {
  for (let attempt = 0; attempt < 100; attempt += 1) {
    try {
      const response = await fetch(origin);
      if (response.ok) return;
    } catch {
      // The owned process is still starting.
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 200));
  }
  throw new Error(`jj-store did not start at ${origin}`);
}
