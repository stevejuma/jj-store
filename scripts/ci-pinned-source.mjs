import { execFileSync } from "node:child_process";
import {
  access,
  mkdir,
  readFile,
  rename,
  rm,
  writeFile,
} from "node:fs/promises";
import { join, resolve } from "node:path";
async function exists(path) {
  try {
    await access(path);
    return true;
  } catch {
    return false;
  }
}

/** Trusted callers fix the repository and package names before resolving a pin. */
export async function ensurePinnedSource(
  pin,
  { root, environment, name, packageName },
) {
  const destination = resolve(root, `../${name}`);
  const marker = join(destination, ".jj-store-source.json");
  if (await exists(destination)) {
    if (environment.CI !== "true") return destination;
    if (await exists(marker)) {
      const selected = JSON.parse(await readFile(marker, "utf8"));
      if (selected.revision === pin.revision) return destination;
    }
    throw Error(`Refusing to replace an existing ${name} source checkout`);
  }
  if (environment.CI !== "true")
    throw Error(`Create the sibling ${name} checkout before local builds`);
  const temporary = destination + `.pending-${process.pid}`;
  const archive = join(root, `.local/${name}-source.tar.gz`);
  await mkdir(join(root, ".local"), { recursive: true });
  const headers = { Accept: "application/vnd.github+json" };
  if (environment.GH_TOKEN)
    headers.Authorization = `Bearer ${environment.GH_TOKEN}`;
  const response = await fetch(
    `https://api.github.com/repos/${pin.repository}/tarball/${pin.revision}`,
    { headers, signal: AbortSignal.timeout(120000) },
  );
  if (!response.ok)
    throw Error(`${name} source fetch failed: HTTP ${response.status}`);
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
    if (pkg.name !== packageName || pkg.version !== pin.version)
      throw Error(`${name} source version mismatch`);
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
