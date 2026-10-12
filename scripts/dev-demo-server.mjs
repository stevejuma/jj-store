import { spawn } from "node:child_process";
import { resolve } from "node:path";
export function startDevelopmentServer(
  repositoryRoot,
  developmentEnvironment,
  serverConfig,
  logHandle,
) {
  const server = spawn(
    resolve(repositoryRoot, "target/debug/jj-store"),
    ["run", "--config", serverConfig, "--format=json"],
    {
      cwd: repositoryRoot,
      detached: true,
      env: { ...process.env, ...developmentEnvironment },
      stdio: ["ignore", logHandle, logHandle],
    },
  );
  server.unref();
  return server;
}

export async function stopDevelopmentServer(server) {
  if (!server.pid) return;
  process.kill(server.pid, "SIGTERM");
  for (let attempt = 0; attempt < 100; attempt += 1) {
    try {
      process.kill(server.pid, 0);
      await new Promise((resolveWait) => setTimeout(resolveWait, 100));
    } catch {
      return;
    }
  }
  throw new Error("temporary native server did not stop");
}
