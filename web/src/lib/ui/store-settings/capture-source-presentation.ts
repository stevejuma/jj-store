import type { Model } from "../../api/client.js";
import { agentAvatarUrl } from "../../collaboration/agent-avatar.js";
import type { CommunityMemberModel } from "../community-workspace/model.js";

type Source = Model<"CaptureSourceCoverage">;
export const captureSourceHarnesses = [
  { value: "all", label: "All harnesses" },
  { value: "codex", label: "Codex" },
  { value: "cursor", label: "Cursor" },
  { value: "open_code", label: "OpenCode" },
];
const integrations = {
  "codex-cli-jsonl-v1": "codex",
  "codex-desktop-jsonl-v1": "codex",
  "cursor-desktop-transcript-v1": "cursor",
  "opencode-read-only-api-v1": "open_code",
  "opencode-export-json-v1": "open_code",
} as const;
export const captureSourceStatuses = [
  { value: "all", label: "All statuses" },
  { value: "awaiting_activity", label: "Awaiting activity" },
  { value: "running", label: "Running" },
  { value: "caught_up", label: "Caught up" },
  { value: "degraded", label: "Degraded" },
  { value: "observed", label: "Observed" },
  { value: "unobserved", label: "Unobserved" },
  { value: "phase:ready", label: "Ready" },
  { value: "phase:running", label: "Reader running" },
  { value: "phase:overdue", label: "Overdue" },
  { value: "phase:draining", label: "Draining" },
  { value: "phase:paused", label: "Paused" },
  { value: "phase:stopped", label: "Stopped" },
];
export function captureSourceMatches(
  source: Source,
  harness: string,
  status: string,
) {
  if (harness !== "all" && integrations[source.profile] !== harness)
    return false;
  if (status === "all") return true;
  if (status.startsWith("phase:")) return source.phase === status.slice(6);
  if (status === "unobserved") return !source.observed;
  return (
    [
      "awaiting_activity",
      "running",
      "caught_up",
      "degraded",
      "observed",
    ].includes(status) &&
    source[
      status as
        "awaiting_activity" | "running" | "caught_up" | "degraded" | "observed"
    ]
  );
}
export function captureSourceMember(source: Source): CommunityMemberModel {
  const id = integrations[source.profile];
  const name = captureSourceHarnesses.find((h) => h.value === id)!.label;
  return {
    id,
    name,
    handle: id,
    initials: name.slice(0, 2),
    kind: "agent",
    avatarUrl: agentAvatarUrl(id),
  };
}
