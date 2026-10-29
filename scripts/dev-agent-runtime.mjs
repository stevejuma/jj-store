import { execFile } from "node:child_process";
import { constants } from "node:fs";
import { access, lstat } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import { selectedAiBundle, verifyAiBundle } from "./lapis-ai.mjs";
import { probePreparedContext } from "./dev-agent-probe.mjs";

const execute = promisify(execFile);
const guidance =
  "Run pnpm ai:prepare using the local lapis-ai checkout, or select a verified bundle with JJ_STORE_AI_BUNDLE_DIR.";

/** Select trusted local resources on every start, without launching an agent. */
export async function developmentAgentEnvironment(
  repositoryRoot,
  environment = process.env,
  server = join(repositoryRoot, "target/debug/jj-store"),
) {
  const bundle =
    !environment.JJ_STORE_ACP_EXECUTABLE || environment.JJ_STORE_AI_BUNDLE_DIR
      ? await verifyAiBundle(
          await selectedAiBundle(repositoryRoot, environment),
        ).catch(() => {
          throw Error(`Invalid Lapis AI bundle. ${guidance}`);
        })
      : undefined;
  const executable = resolve(
    repositoryRoot,
    environment.JJ_STORE_ACP_EXECUTABLE ?? bundle.helpers["lapis-acp"],
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
    throw Error(`Cannot inspect lapis-acp at ${executable}. ${guidance}`);
  }
  const versions = /--control-version\s+<([^>\n]+)>/u
    .exec(help)?.[1]
    .split("|");
  if (!versions?.includes("3"))
    throw Error(
      `lapis-acp at ${executable} does not support control version 3. ${guidance}`,
    );
  try {
    if (!(await probePreparedContext(executable, repositoryRoot, environment)))
      throw Error();
  } catch {
    throw Error(
      `lapis-acp at ${executable} does not support prepared context. ${guidance}`,
    );
  }
  return {
    JJ_STORE_ACP_EXECUTABLE: executable,
    JJ_STORE_AGENT_JJ_EXECUTABLE: jj,
    ...(bundle
      ? {
          JJ_STORE_AI_BUNDLE_DIR: bundle.root,
          JJ_STORE_LOCAL_AGENT_EXECUTABLE: bundle.helpers["lapis-agent"],
          JJ_STORE_AGENT_TOOLS_EXECUTABLE: bundle.helpers["lapis-agent-tools"],
          JJ_STORE_COMPUTE_EXECUTABLE: bundle.helpers["lapis-compute"],
          ...(bundle.nativeRuntime
            ? { JJ_STORE_COMPUTE_NATIVE_RUNTIME_DIR: bundle.nativeRuntime }
            : {}),
        }
      : {}),
  };
}
