import assert from "node:assert/strict";
import { mkdtemp, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import test from "node:test";
import {
  representativeAgentEvents,
  seedAgentActivity,
} from "./dev-demo-agents.mjs";
import { normalizedAgentId } from "./dev-demo-agent-reviews.mjs";
import { seedReviews } from "./dev-demo-reviews.mjs";
import { executeWithInput } from "./dev-demo-services.mjs";

const repositoryRoot = resolve(import.meta.dirname, "..");

test("representative agent seed covers integrations, tools, transcripts, and lifecycle states", async () => {
  const events = await representativeAgentEvents(
    repositoryRoot,
    "/sample/workspace",
  );
  assert.deepEqual(Object.keys(events), ["codex", "cursor", "opencode"]);
  assert.equal(events.codex.at(-1).hook_event_name, "SessionEnd");
  assert.equal(events.cursor.at(-1).hook_event_name, "stop");
  assert.equal(events.opencode[0].input.event.type, "session.created");
  assert.ok(
    events.codex.some((event) => event.hook_event_name === "UserPromptSubmit"),
  );
  assert.ok(
    events.cursor.some(
      (event) => event.hook_event_name === "afterAgentResponse",
    ),
  );
  assert.ok(events.opencode.some((event) => event.hook === "chat.message"));
  assert.ok(
    Object.values(events)
      .flat()
      .some((event) => JSON.stringify(event).includes("test failed")),
  );
  assert.ok(
    Object.values(events)
      .flat()
      .some((event) => JSON.stringify(event).includes("success")),
  );
});

test("agent seed installs, publishes, and removes each owned integration", async () => {
  const runtimeRoot = await mkdtemp(`${tmpdir()}/jj-store-dev-agents-`);
  const commands = [];
  const hooks = [];
  try {
    await seedAgentActivity({
      repositoryRoot,
      runtimeRoot,
      repository: "/sample/workspace",
      run: async (_command, args, options) => {
        assert.equal(
          options.cwd,
          "/sample/workspace",
          "setup must target the isolated folder, not the source checkout",
        );
        commands.push(args);
        return { stdout: "", stderr: "" };
      },
      runInput: async (_command, args, input, options) => {
        assert.equal(
          options.cwd,
          "/sample/workspace",
          "hooks must target the isolated folder",
        );
        hooks.push({ args, event: JSON.parse(input) });
        return { stdout: "", stderr: "" };
      },
    });
    assert.equal(commands.filter((args) => args.includes("setup")).length, 3);
    assert.equal(commands.filter((args) => args.includes("remove")).length, 3);
    assert.deepEqual(
      new Set(hooks.map(({ args }) => args.at(-1))),
      new Set(["codex", "cursor", "opencode"]),
    );
    assert.ok(hooks.length > 20);
  } finally {
    await rm(runtimeRoot, { recursive: true, force: true });
  }
});

test("review seed exercises every command and each supported sample anchor category", async (t) => {
  const commands = [];
  const id = (byte, width) => byte.toString(16).padStart(2, "0").repeat(width);
  let resultIndex = 1;
  t.mock.method(globalThis, "fetch", async (input, options = {}) => {
    const url = new URL(input);
    if (url.pathname.endsWith("/reviews/commands")) {
      const request = JSON.parse(options.body);
      commands.push(request.command);
      const byte = resultIndex++;
      return Response.json({
        status: "committed",
        receipt: {
          result: {
            thread_id: id(byte, 16),
            comment_id: id(byte + 30, 16),
            action_id: null,
            thread_snapshot: { kind: "thread_snapshot", id: id(byte + 60, 32) },
            comment_revision: {
              kind: "comment_revision",
              id: id(byte + 90, 32),
            },
            event: { kind: "thread_event", id: id(byte + 120, 32) },
          },
        },
      });
    }
    if (url.pathname.endsWith("/tree")) {
      return Response.json({
        data: {
          entries: [
            {
              name: "lib.rs",
              terms: [{ type: "file", id: id(10, 64) }],
              conflicted: false,
            },
          ],
        },
      });
    }
    if (url.pathname.includes("/files/")) {
      return new Response(
        "pub fn supports_native_history() -> bool { true }\n",
      );
    }
    if (url.pathname.endsWith("/agent/sessions")) {
      return Response.json({
        sessions: [
          { session_id: id(11, 16), integration: "cursor", tool_count: 1 },
        ],
      });
    }
    if (url.pathname.includes("/agent/sessions/")) {
      const sourceInstanceId = id(16, 16);
      return Response.json({
        snapshot: { agent_root: id(12, 32) },
        session: { session_id: id(11, 16), snapshot_id: id(13, 32) },
        transcripts: [
          {
            source_instance_id: sourceInstanceId,
            source_format: "cursor-normalized-messages-v1",
            decoder_version: "cursor@3.22.7/v1",
            sanitizer_version: "mongodb-kingfisher@2.7.0/builtin-medium/v1",
          },
        ],
        tools: [
          {
            tool_call_id: normalizedAgentId(
              sourceInstanceId,
              "cursor",
              "tool",
              "conv_demo/edit_1",
            ),
            snapshot_id: id(15, 32),
          },
        ],
      });
    }
    return new Response("missing fake route", { status: 404 });
  });

  await seedReviews({
    origin: "http://127.0.0.1:7460/",
    repositoryId: id(1, 16),
    references: {
      operationId: id(2, 64),
      commitId: id(3, 64),
      changeId: id(4, 16),
    },
  });
  assert.deepEqual(
    new Set(commands.map((command) => command.command)),
    new Set([
      "create_thread",
      "append_reply",
      "edit_comment",
      "tombstone_comment",
      "hide_comment",
      "resolve_thread",
      "reopen_thread",
    ]),
  );
  assert.deepEqual(
    commands
      .filter((command) => command.command === "create_thread")
      .map((command) => command.anchor.target.target),
    [
      "repository",
      "operation",
      "revision",
      "code_text",
      "code_artifact",
      "agent_artifact",
      "agent_text",
    ],
  );
});

test("bounded process input helper forwards stdin", async () => {
  const result = await executeWithInput("/bin/cat", [], "sample input\n");
  assert.equal(result.stdout, "sample input");
});
