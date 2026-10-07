import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import { transcriptMessage } from "../src/lib/collaboration/presentation.js";

const summary = {
  session_id: "11".repeat(16),
  integration: "open_code",
} as Model<"AgentSessionSummary">;
const message = (content: string, integration = summary.integration) =>
  transcriptMessage(
    { sequence: 7, media_type: "json", content },
    { ...summary, integration },
  );

test("OpenCode text parts render ordered message text with the recorded message identity", () => {
  const raw = JSON.stringify([
    { id: "part_1", type: "text", text: "Update the **parser**." },
    { id: "part_2", type: "text", text: "Keep the review anchors exact." },
  ]);
  assert.equal(
    message(raw).content,
    "Update the **parser**.\n\nKeep the review anchors exact.",
  );
  assert.equal(message(raw).id, `${summary.session_id}:7`);
});

test("ordinary JSON and other integrations retain their visible JSON representation", () => {
  const raw = JSON.stringify([{ type: "text", text: "A JSON example" }]);
  assert.match(message(raw, "codex").content, /^```json/);
  assert.match(message('{"success":true}').content, /"success": true/);
});

test("mixed, unknown, empty and malformed OpenCode parts remain visible", () => {
  for (const parts of [
    [
      { type: "text", text: "Hello" },
      { type: "file", url: "file:///sample.png" },
    ],
    [{ type: "text", text: 7 }],
    [{ type: "unknown", data: "kept" }],
    [],
  ])
    assert.equal(
      message(JSON.stringify(parts)).content,
      `\`\`\`json\n${JSON.stringify(parts, null, 2)}\n\`\`\``,
    );
  assert.equal(message("{invalid JSON").content, "```text\n{invalid JSON\n```");
});

test("controller context envelope renders human text and preserves its historical snapshot", () => {
  const raw = JSON.stringify({
    schema: "jj-store.sent-context.v1",
    text: "Fix this file",
    context: {
      version: 1,
      blocks: [{ title: "Original context", text: "Pinned revision" }],
      delivery: "sent",
    },
  });
  assert.equal(message(raw, "owned_controller").content, "Fix this file");
  assert.match(message(raw, "codex").content, /Original context/);
});
