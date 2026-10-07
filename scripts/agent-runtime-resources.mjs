import { cp, stat, lstat, readdir, writeFile } from "node:fs/promises";
import { constants, createReadStream } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";

async function checksum(path) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(path)) hash.update(chunk);
  return hash.digest("hex");
}
export async function stageAgentResources(destination, helpers, nativeRuntime) {
  const names = [
    "jj-acp",
    "jj-store-local-agent",
    "jj-store-agent-tools",
    "jj-store-compute",
  ];
  const binaries = [];
  for (const name of names) {
    const source = helpers[name];
    if (!source || !(await stat(source)).isFile())
      throw Error(`Missing bundled helper: ${name}`);
    const digest = await checksum(source);
    await cp(source, join(destination, name), {
      mode: constants.COPYFILE_FICLONE,
    });
    binaries.push({ name, sha256: digest });
  }
  const nativeFiles = [];
  async function inventory(source, relative) {
    const info = await lstat(source);
    if (info.isSymbolicLink())
      throw Error("Native runtime symlinks are not supported");
    if (info.isDirectory()) {
      for (const name of (await readdir(source)).sort())
        await inventory(join(source, name), `${relative}/${name}`);
    } else if (info.isFile()) {
      if (nativeFiles.length >= 512) throw Error("Native runtime file limit");
      nativeFiles.push({
        name: relative,
        sha256: await checksum(source),
      });
    } else throw Error("Invalid native runtime resource");
  }
  if (nativeRuntime) {
    if (!(await stat(nativeRuntime)).isDirectory())
      throw Error("Invalid native runtime directory");
    await inventory(nativeRuntime, "native-runtime");
    if (!nativeFiles.length) throw Error("Empty native runtime directory");
    await cp(nativeRuntime, join(destination, "native-runtime"), {
      recursive: true,
      mode: constants.COPYFILE_FICLONE,
    });
  }
  await cp(
    new URL("../runtime/THIRD_PARTY.md", import.meta.url),
    join(destination, "THIRD_PARTY.md"),
  );
  await cp(
    new URL(
      "../runtime/local-agent/vendor/buzz-agent/LICENSE",
      import.meta.url,
    ),
    join(destination, "LICENSE.local-agent"),
  );
  await cp(
    new URL("../runtime/local-agent/UPSTREAM.md", import.meta.url),
    join(destination, "LOCAL_AGENT_UPSTREAM.md"),
  );
  await writeFile(
    join(destination, "agent-resources.json"),
    JSON.stringify(
      {
        version: 1,
        binaries,
        nativeRuntime: !!nativeRuntime,
        nativeFiles,
        meshRevision: "9f192c9d821991ba43d79bb15fd2f2f12e597571",
        localAgentRevision: "ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be",
      },
      null,
      2,
    ) + "\n",
  );
}
