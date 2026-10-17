import { appendFileSync } from "node:fs";
import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { ensureAiSource } from "./ci-lapis-source.mjs";
import { probePreparedContext } from "./dev-agent-probe.mjs";

export async function install() {
  if (!process.env.GITHUB_ENV)
    throw Error("ACP setup requires a runner-owned environment file");
  const source = await ensureAiSource();
  execFileSync(
    "pnpm",
    ["dev:cargo", "--", "build", "-p", "lapis-acp", "--locked"],
    { cwd: source, stdio: "inherit" },
  );
  const executable = join(source, "target/debug/lapis-acp");
  if (!(await probePreparedContext(executable, process.cwd(), process.env)))
    throw Error("Lapis ACP lacks prepared Control V3 context");
  appendFileSync(
    process.env.GITHUB_ENV,
    `JJ_STORE_ACP_EXECUTABLE=${executable}\n`,
  );
  console.log("Installed source-pinned Lapis ACP for cold CI acceptance");
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await install();
