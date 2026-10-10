import { expect, test } from "vitest";
import {
  conversationFileLink,
  conversationPathLabels,
} from "../../../web/src/lib/conversations/file-links.js";
import type { AgentSession } from "../../../web/src/lib/controller/agent-execution-types.js";
const session = {
  id: "session",
  state: "idle",
  workspace: "/Users/synthetic/workspaces/session",
  repository_bindings: [
    {
      repository: "repo",
      workspace: "/Users/synthetic/workspaces/session/repositories/repo",
      operation: "op",
      commit: "commit",
    },
  ],
} as AgentSession;

test("file links resolve absolute, relative and privacy-substituted destinations to the same binding", () => {
  for (const path of [
    "$WORKSPACE/repositories/repo/LICENSE:3",
    "$HOME/workspaces/session/repositories/repo/LICENSE:3",
    "/Users/synthetic/workspaces/session/repositories/repo/LICENSE:3",
    "repositories/repo/LICENSE:3",
    "./LICENSE:3",
  ]) {
    const link = conversationFileLink(path, session);
    expect(link?.path).toBe("LICENSE");
    expect(link?.line).toBe(3);
    expect(link?.binding.repository).toBe("repo");
  }
  expect(conversationPathLabels(session)).toEqual([
    [session.workspace, "$WORKSPACE"],
    ["/Users/synthetic", "$HOME"],
  ]);
});

test("file links refuse unsafe schemes, traversal, unrelated roots and ambiguous repositories", () => {
  for (const path of [
    "javascript:alert(1)",
    "file:///etc/passwd",
    "/etc/passwd",
    "../LICENSE",
    "$WORKSPACE/repositories/repo/%2e%2e/secret",
    "$WORKSPACE/repositories/other/LICENSE",
    "LICENSE:0",
    "LICENSE?x=1",
  ])
    expect(conversationFileLink(path, session)).toBeUndefined();
  expect(
    conversationFileLink("LICENSE", {
      ...session,
      repository_bindings: [
        ...session.repository_bindings,
        { ...session.repository_bindings[0], repository: "other" },
      ],
    }),
  ).toBeUndefined();
});
