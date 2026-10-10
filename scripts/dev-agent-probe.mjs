import { spawn } from "node:child_process";

/** Validate parsing without warming a worker, opening a session or sending a turn. */
export async function probePreparedContext(executable, cwd, env) {
  const ids = ["preflight-text-context", "preflight-context-only"];
  const frames = ids.map((requestId, index) => ({
    version: 3,
    type: "prompt",
    requestId,
    sessionId: "preflight-unopened",
    text: index ? "" : "Preflight",
    context: "Prepared preflight context",
    maxDurationMs: 1,
  }));
  const stdout = await new Promise((resolve, reject) => {
    const child = spawn(
      executable,
      ["serve", "--control-version", "3", "--runner", "codex", "--cwd", cwd],
      { cwd, env, stdio: ["pipe", "pipe", "ignore"] },
    );
    let output = "";
    const timeout = setTimeout(() => {
      child.kill("SIGKILL");
      reject(Error("probe timeout"));
    }, 5000);
    child.once("error", reject);
    child.stdout.on("data", (data) => {
      output += data;
      if (output.length > 64 * 1024) {
        child.kill("SIGKILL");
        reject(Error("probe output limit"));
      }
    });
    child.stdin.on("error", () => {});
    child.once("close", (code) => {
      clearTimeout(timeout);
      code === 0 ? resolve(output) : reject(Error("probe failed"));
    });
    child.stdin.end(
      frames.map((frame) => JSON.stringify(frame)).join("\n") + "\n",
    );
  });
  const responses = stdout
    .trim()
    .split("\n")
    .map((line) => JSON.parse(line));
  return ids.every((id) =>
    responses.some(
      (frame) =>
        frame.version === 3 &&
        frame.type === "error" &&
        frame.requestId === id &&
        frame.code === "session_not_open",
    ),
  );
}
