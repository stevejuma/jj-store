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
    child.stdout.setEncoding("utf8");
    let pending = "";
    const answered = new Set();
    const timeout = setTimeout(() => {
      child.kill("SIGKILL");
      reject(Error("probe timeout"));
    }, 5000);
    child.once("error", reject);
    child.stdout.on("data", (data) => {
      output += data;
      if (Buffer.byteLength(output) > 64 * 1024) {
        child.kill("SIGKILL");
        reject(Error("probe output limit"));
        return;
      }
      pending += data;
      while (pending.includes("\n")) {
        const boundary = pending.indexOf("\n");
        const line = pending.slice(0, boundary);
        pending = pending.slice(boundary + 1);
        try {
          const frame = JSON.parse(line);
          if (
            ids.includes(frame.requestId) &&
            ["error", "record"].includes(frame.type)
          )
            answered.add(frame.requestId);
        } catch {
          child.kill("SIGKILL");
          reject(Error("invalid probe response"));
          return;
        }
      }
      // EOF closes the control host and cancels queued work. Wait for both
      // terminal replies before closing so shutdown cannot race the probe.
      if (answered.size === ids.length) child.stdin.end();
    });
    child.stdin.on("error", () => {});
    child.once("close", (code) => {
      clearTimeout(timeout);
      code === 0 ? resolve(output) : reject(Error("probe failed"));
    });
    child.stdin.write(
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
