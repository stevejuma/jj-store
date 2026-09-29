import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const integrations = ["codex", "cursor", "opencode"];

export async function representativeAgentEvents(repositoryRoot, workspace) {
  const fixture = async (name) =>
    JSON.parse(
      (
        await readFile(
          join(repositoryRoot, "crates/jj-agent/tests/fixtures", name),
          "utf8",
        )
      ).replaceAll("<workspace>", workspace),
    );
  const codex = await fixture("codex-0.156.1-hooks.json");
  const cursor = (await fixture("cursor-3.22.7-hooks.json")).slice(0, -1);
  cursor.push({
    ...cursor[0],
    hook_event_name: "stop",
  });
  const opencode = (await fixture("opencode-1.18.30-plugin.json")).slice(0, -2);
  return { codex, cursor, opencode };
}

export async function seedAgentActivity({
  repositoryRoot,
  runtimeRoot,
  repository,
  run,
  runInput,
}) {
  const executable = resolve(repositoryRoot, "target/debug/jj");
  const state = join(runtimeRoot, "agent-state");
  const bin = join(runtimeRoot, "agent-bin");
  await mkdir(bin, { recursive: true, mode: 0o700 });
  const versions = {
    codex: "codex-cli 0.156.1",
    "cursor-agent": "3.22.7",
    opencode: "1.18.30",
  };
  await Promise.all(
    Object.entries(versions).map(([name, version]) =>
      writeFile(join(bin, name), `#!/bin/sh\nprintf '%s\\n' '${version}'\n`, {
        mode: 0o700,
      }),
    ),
  );
  const env = { PATH: `${bin}:${process.env.PATH ?? ""}` };
  const args = (...values) => [
    "-R",
    repository,
    "--no-pager",
    "--color=never",
    "agent",
    "--state-dir",
    state,
    "--format=json",
    ...values,
  ];
  await run(
    executable,
    args(
      "enable",
      "--non-interactive",
      "--agent",
      "codex",
      "--agent",
      "cursor",
      "--agent",
      "opencode",
    ),
    { env },
  );
  const installed = [];
  try {
    for (const integration of integrations) {
      await run(executable, args("setup", integration), { env });
      installed.push(integration);
    }
    const events = await representativeAgentEvents(repositoryRoot, repository);
    for (const integration of integrations) {
      for (const event of events[integration]) {
        await runInput(
          executable,
          args("hook", integration),
          `${JSON.stringify(event)}\n`,
          { env },
        );
      }
    }
  } finally {
    for (const integration of installed.reverse()) {
      await run(executable, args("remove", integration), { env });
    }
  }
}
