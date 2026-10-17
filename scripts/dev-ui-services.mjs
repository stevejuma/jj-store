import { spawn } from "node:child_process";
import { openSync, closeSync } from "node:fs";
import { createServer } from "node:net";

export async function assertPortAvailable(port) {
  const listener = createServer();
  await new Promise((resolveListen, reject) => {
    listener.once("error", reject);
    listener.listen(port, "127.0.0.1", resolveListen);
  });
  await new Promise((resolveClose, reject) =>
    listener.close((error) => (error ? reject(error) : resolveClose())),
  );
}

export function start(
  command,
  args,
  repositoryRoot,
  layout,
  environment,
  logPath,
) {
  const log = openSync(logPath, "a", 0o600);
  const child = spawn(command, args, {
    cwd: repositoryRoot,
    detached: true,
    env: {
      ...environment,
      JJ_STORE_DEVELOPMENT_STATE_ROOT: layout.runtimeRoot,
    },
    stdio: ["ignore", log, log],
  });
  closeSync(log);
  child.unref();
  return new Promise((resolveStarted, reject) => {
    child.once("error", reject);
    child.once("spawn", () => resolveStarted(child.pid));
  });
}

export async function ready(origin) {
  // Native startup verifies the full helper/native bundle before serving.
  const deadline = performance.now() + 60_000;
  while (performance.now() < deadline) {
    try {
      const response = await fetch(`${origin}v1/bootstrap`, {
        headers: {
          "x-jj-store-protocol": "1",
          Origin: new URL(origin).origin,
          "Sec-Fetch-Site": "same-origin",
          "Sec-Fetch-Mode": "cors",
          "Sec-Fetch-Dest": "empty",
        },
        signal: AbortSignal.timeout(1000),
      });
      if (
        response.ok &&
        (await response.json()).access_mode === "local_trusted"
      )
        return;
      await response.body?.cancel();
    } catch {
      /* Both owned processes are still starting. */
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 100));
  }
  throw new Error(
    "UI development did not become ready; inspect ui.log and server.log in the selected runtime root",
  );
}
