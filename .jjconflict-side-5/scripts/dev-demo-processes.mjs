import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);

async function commandFor(pid, owner) {
  try {
    return (
      await execFileAsync("ps", ["-p", String(pid), "-o", "command="], {
        cwd: owner,
      })
    ).stdout.trim();
  } catch (error) {
    if (error.code === 1 && !error.stdout?.trim()) return null;
    throw error;
  }
}

export async function inspectRuntimeProcesses(
  runtime,
  { owner, runtimeRoot, ui = true },
) {
  const processes = [];
  for (const kind of ui ? ["ui", "server"] : ["server"]) {
    const pid = runtime[`${kind}Pid`];
    if (pid === undefined) continue;
    if (!Number.isInteger(pid) || pid <= 1)
      throw new Error(`invalid ${kind} process marker`);
    const command = await commandFor(pid, owner);
    if (command === null) continue;
    const owned =
      kind === "ui"
        ? runtime.uiRunner &&
          command.includes(runtime.uiRunner) &&
          command.includes(`--runtime-root ${runtimeRoot}`)
        : command.includes("jj-store") &&
          (runtime.serverConfig
            ? command.includes(runtime.serverConfig)
            : command.includes(runtimeRoot) && command.includes("server.toml"));
    if (!owned)
      throw new Error(`refusing to stop unrelated ${kind} process ${pid}`);
    processes.push({ kind, pid });
  }
  return processes;
}

export async function stopRuntimeProcesses(runtime, options) {
  // Inspect every marker first: a stale/unowned UI PID must not stop the backend.
  const processes = await inspectRuntimeProcesses(runtime, options);
  for (const { pid } of processes) {
    try {
      process.kill(pid, "SIGTERM");
    } catch (error) {
      if (error.code !== "ESRCH") throw error;
    }
  }
  for (const { kind, pid } of processes) {
    let stopped = false;
    for (let attempt = 0; attempt < 100; attempt += 1) {
      if ((await commandFor(pid, options.owner)) === null) {
        stopped = true;
        break;
      }
      await new Promise((resolveWait) => setTimeout(resolveWait, 100));
    }
    if (!stopped) throw new Error(`owned ${kind} process ${pid} did not stop`);
  }
}
