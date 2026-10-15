import { spawn } from "node:child_process";
import { mkdirSync, createWriteStream, appendFileSync } from "node:fs";
const [label, separator, command, ...args] = process.argv.slice(2);
if (separator !== "--" || !/^[a-z0-9-]+$/.test(label) || !command)
  throw Error("Usage: ci-exec.mjs LABEL -- COMMAND [ARGS]");
mkdirSync(".release/ci-results", { recursive: true });
const log = createWriteStream(`.release/ci-results/${label}.log`);
const start = Date.now();
const child = spawn(command, args, { stdio: ["ignore", "pipe", "pipe"] });
for (const stream of [child.stdout, child.stderr])
  stream.on("data", (data) => {
    process.stdout.write(data);
    log.write(data);
  });
for (const signal of ["SIGINT", "SIGTERM"])
  process.on(signal, () => child.kill(signal));
child.on("error", (error) => {
  log.write(String(error));
  process.stderr.write(String(error));
});
child.on("close", (code, signal) => {
  log.end();
  appendFileSync(
    ".release/ci-results/commands.jsonl",
    JSON.stringify({
      label,
      command,
      args,
      start,
      seconds: (Date.now() - start) / 1000,
      exit_code: code,
      signal,
    }) + "\n",
  );
  process.exitCode = code ?? 1;
});
