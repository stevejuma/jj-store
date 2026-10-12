#!/usr/bin/env node
import { copyFile, chmod, mkdir, rename, rm, stat } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { homedir } from "node:os";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { parseArgs } from "node:util";
import { cargoCommand } from "./managed-cargo.mjs";
const tools = [
  ["jj-store", "jj-store"],
  ["jj", "jj-store-jj"],
];
export async function installCli({ source, destination }) {
  for (const [name] of tools)
    if (!(await stat(join(source, name))).isFile())
      throw Error(`missing CLI artifact: ${name}`);
  await mkdir(destination, { recursive: true });
  for (const [name, installedName] of tools) {
    const temporary = join(destination, `.${installedName}.${process.pid}.tmp`);
    try {
      await copyFile(join(source, name), temporary, 1);
      await chmod(temporary, 0o755);
      await rename(temporary, join(destination, installedName));
    } finally {
      await rm(temporary, { force: true });
    }
  }
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  try {
    const { values } = parseArgs({
      options: {
        "bin-dir": { type: "string" },
        debug: { type: "boolean" },
        "no-build": { type: "boolean" },
        help: { type: "boolean" },
      },
    });
    if (values.help)
      console.log(
        "Usage: pnpm cli:install [--bin-dir PATH] [--debug] [--no-build]\nBuild and install jj-store and jj-store-jj into ~/.local/bin by default. Existing jj is unchanged.",
      );
    else {
      const root = resolve(import.meta.dirname, "..");
      if (!values["no-build"]) {
        const built = spawnSync(
          ...cargoCommand([
            "build",
            "-p",
            "jj-store-cli",
            "--bins",
            "--locked",
            ...(values.debug ? [] : ["--release"]),
          ]),
          { cwd: root, stdio: "inherit" },
        );
        if (built.error) throw built.error;
        if (built.status !== 0)
          throw Error("CLI build failed; installed tools unchanged");
      }
      const destination = resolve(
        values["bin-dir"] ?? join(homedir(), ".local/bin"),
      );
      await installCli({
        source: join(root, "target", values.debug ? "debug" : "release"),
        destination,
      });
      console.log(
        `Installed ${join(destination, "jj-store")} and ${join(destination, "jj-store-jj")}.\nAdd ${destination} to PATH. Use jj-store-jj for native and local project commands; jj-store administers the server.`,
      );
    }
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
}
