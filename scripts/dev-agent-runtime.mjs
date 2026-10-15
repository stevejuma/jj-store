import { execFile } from "node:child_process";
import { constants } from "node:fs";
import { access, lstat } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import { probePreparedContext } from "./dev-agent-probe.mjs";

const execute = promisify(execFile);
const guidance =
  "Stage a current jj-acp with pnpm desktop:prepare -- --acp-executable /absolute/path/to/jj-acp, or set JJ_STORE_ACP_EXECUTABLE explicitly.";

/** Select trusted local resources on every start, without launching an agent. */
export async function developmentAgentEnvironment(
  repositoryRoot,
  environment = process.env,
  server = join(repositoryRoot, "target/debug/jj-store"),
) {
  const executable = resolve(
    repositoryRoot,
    environment.JJ_STORE_ACP_EXECUTABLE ?? "desktop/resources/jj-acp",
  );
  const jj = resolve(
    repositoryRoot,
    environment.JJ_STORE_AGENT_JJ_EXECUTABLE ??
      join(dirname(server), "jj-store-jj"),
  );
  for (const path of [executable, jj]) {
    try {
      const metadata = await lstat(path);
      if (!metadata.isFile() || metadata.isSymbolicLink()) throw Error();
      await access(path, constants.X_OK);
    } catch {
      throw Error(
        `Missing regular agent runtime executable: ${path}. ${guidance}`,
      );
    }
  }
  let help;
  try {
    help = (
      await execute(executable, ["--help"], {
        cwd: repositoryRoot,
        env: environment,
        timeout: 5000,
        maxBuffer: 64 * 1024,
      })
    ).stdout;
  } catch {
    throw Error(`Cannot inspect jj-acp at ${executable}. ${guidance}`);
  }
  const versions = /--control-version\s+<([^>\n]+)>/u
    .exec(help)?.[1]
    .split("|");
  if (!versions?.includes("3"))
    throw Error(
      `jj-acp at ${executable} does not support control version 3. ${guidance}`,
    );
  try {
    if (!(await probePreparedContext(executable, repositoryRoot, environment)))
      throw Error();
  } catch {
    throw Error(
      `jj-acp at ${executable} does not support prepared context. ${guidance}`,
    );
  }
  return {
    JJ_STORE_ACP_EXECUTABLE: executable,
    JJ_STORE_AGENT_JJ_EXECUTABLE: jj,
  };
}
