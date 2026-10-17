import { execFileSync } from "node:child_process";
import { mkdir, readFile, rename, rm, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { exists, repositoryRoot } from "./lapis-ai.mjs";

export function validateAiSourcePin(pin) {
  if (
    pin.repository !== "lapismd/lapis-ai" ||
    !/^[a-f0-9]{40}$/.test(pin.revision) ||
    !/^\d+\.\d+\.\d+$/.test(pin.version)
  )
    throw Error("Invalid Lapis AI source pin");
  return pin;
}
export async function ensureAiSource(
  root = repositoryRoot,
  environment = process.env,
) {
  const pin = validateAiSourcePin(
    JSON.parse(
      await readFile(join(root, "scripts/lapis-ai-source.json"), "utf8"),
    ),
  );
  const destination = resolve(root, "../lapis-ai");
  const marker = join(destination, ".jj-store-source.json");
  if (await exists(destination)) {
    if (environment.CI !== "true") return destination;
    if (await exists(marker)) {
      const selected = JSON.parse(await readFile(marker, "utf8"));
      if (selected.revision === pin.revision) return destination;
    }
    throw Error("Refusing to replace an existing Lapis AI source checkout");
  }
  if (environment.CI !== "true")
    throw Error("Create the sibling lapis-ai checkout before local builds");
  const temporary = destination + `.pending-${process.pid}`;
  const archive = join(root, ".local/lapis-ai-source.tar.gz");
  await mkdir(join(root, ".local"), { recursive: true });
  const headers = { Accept: "application/vnd.github+json" };
  if (environment.GH_TOKEN)
    headers.Authorization = `Bearer ${environment.GH_TOKEN}`;
  const response = await fetch(
    `https://api.github.com/repos/${pin.repository}/tarball/${pin.revision}`,
    { headers, signal: AbortSignal.timeout(120000) },
  );
  if (!response.ok)
    throw Error(`Lapis AI source fetch failed: HTTP ${response.status}`);
  await writeFile(archive, Buffer.from(await response.arrayBuffer()));
  await mkdir(temporary);
  try {
    execFileSync(
      "tar",
      ["-xzf", archive, "-C", temporary, "--strip-components=1"],
      { timeout: 60000, stdio: "pipe" },
    );
    const pkg = JSON.parse(
      await readFile(join(temporary, "package.json"), "utf8"),
    );
    if (pkg.name !== "@lapismd/lapis-ai" || pkg.version !== pin.version)
      throw Error("Lapis AI source version mismatch");
    await writeFile(
      join(temporary, ".jj-store-source.json"),
      JSON.stringify(pin),
    );
    await rename(temporary, destination);
  } finally {
    await rm(archive, { force: true });
    await rm(temporary, { recursive: true, force: true });
  }
  return destination;
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await ensureAiSource();
