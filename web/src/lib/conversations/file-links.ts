import type { AgentSession } from "../controller/agent-execution-types.js";
import {
  workspaceBindings,
  type WorkspaceBinding,
} from "./workspace-bindings.js";

export type ConversationFileLink = {
  binding: WorkspaceBinding;
  path: string;
  line?: number;
};

/** Captured destinations may name only this participation's verified code bindings. */
export function conversationFileLink(
  destination: string,
  session: AgentSession,
): ConversationFileLink | undefined {
  let value: string;
  try {
    value = decodeURIComponent(destination);
  } catch {
    return;
  }
  if (
    !value ||
    /[\x00-\x1f\\?#]/.test(value) ||
    /^[a-z][a-z\d+.-]*:/i.test(value)
  )
    return;
  const suffix = /:(\d+)(?::\d+)?$/.exec(value);
  const line = suffix ? Number(suffix[1]) : undefined;
  if (line !== undefined && (!Number.isSafeInteger(line) || line < 1)) return;
  if (suffix) value = value.slice(0, suffix.index);
  const bindings = workspaceBindings([session], new Map());
  const workspace = session.workspace.replace(/\/$/, "");
  const home = /^(\/Users\/[^/]+|\/home\/[^/]+)(?:\/|$)/.exec(workspace)?.[1];
  if (value.startsWith("$WORKSPACE/")) value = workspace + value.slice(10);
  if (home && value.startsWith("$HOME/")) value = home + value.slice(5);
  value = value.replace(/^\.\//, "");
  if (
    value
      .split("/")
      .some(
        (part) =>
          part === ".." ||
          part === "." ||
          (part === "" && !value.startsWith("/")),
      )
  )
    return;
  let binding: WorkspaceBinding | undefined;
  let path: string;
  if (value.startsWith("/")) {
    binding = bindings.find(
      (b) =>
        b.location && value.startsWith(b.location.replace(/\/$/, "") + "/"),
    );
    if (!binding?.location) return;
    path = value.slice(binding.location.replace(/\/$/, "").length + 1);
  } else {
    const attached = /^repositories\/([^/]+)\/(.+)$/.exec(value);
    binding = attached
      ? bindings.find((b) => b.repository === attached[1])
      : bindings.length === 1
        ? bindings[0]
        : undefined;
    if (!binding) return;
    path = attached?.[2] ?? value;
  }
  if (
    !path ||
    path.split("/").some((part) => !part || part === ".." || part === ".")
  )
    return;
  return { binding, path, line };
}

export function conversationPathLabels(
  session: AgentSession,
): readonly (readonly [string, string])[] {
  const workspace = session.workspace.replace(/\/$/, "");
  const home = /^(\/Users\/[^/]+|\/home\/[^/]+)(?:\/|$)/.exec(workspace)?.[1];
  return [
    [workspace, "$WORKSPACE"],
    ...(home ? [[home, "$HOME"] as const] : []),
  ];
}
