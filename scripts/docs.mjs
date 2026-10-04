import { execFileSync } from "node:child_process";
import { mkdirSync, cpSync, rmSync } from "node:fs";
import { references } from "./docs/references.mjs";
import { review } from "./docs/review.mjs";
import { diagrams } from "./docs/diagrams.mjs";
const mode = process.argv[2] ?? "check";
const run = (cmd, args) => execFileSync(cmd, args, { stdio: "inherit" });
function check() {
  references();
  review();
}
async function build() {
  check();
  await diagrams();
  run("uv", ["run", "--locked", "mkdocs", "build", "--strict"]);
  run("uv", ["run", "--locked", "python", "scripts/docs/site.py"]);
}
if (mode === "generate") references(true);
else if (mode === "review") review(process.argv[3], process.argv[4]);
else if (mode === "check") await build();
else if (mode === "build" || mode === "serve") {
  await build();
  if (mode === "serve")
    run("uv", [
      "run",
      "--locked",
      "python",
      "-m",
      "http.server",
      "7480",
      "--bind",
      "127.0.0.1",
      "--directory",
      "docs/build",
    ]);
} else if (mode === "stage") {
  await build();
  mkdirSync("web/dist", { recursive: true });
  rmSync("web/dist/help", { recursive: true, force: true });
  cpSync("docs/build", "web/dist/help", { recursive: true });
} else throw Error("Unknown documentation operation");
