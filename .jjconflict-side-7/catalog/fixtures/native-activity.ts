import type {
  NativeActivityCommit,
  NativeReviewContext,
} from "../../web/src/lib/controller/native-activity-types.js";
import type { StoreControllerState } from "../../web/src/lib/controller/contracts.js";
import { agentDetail } from "./collaboration.js";
import { IDS, revisions, sampleDetail } from "./repository.js";
export function activityCommit(repository: string): NativeActivityCommit {
  return {
    repository,
    operation: IDS.opLeft,
    revision: revisions.find((revision) => revision.eventId === IDS.merge)!,
    agents: {
      sessions: [
        {
          ...agentDetail,
          snapshot: { ...agentDetail.snapshot, repo_id: repository },
        },
      ],
      error: "",
    },
  };
}
export function nativeReviewContext(
  anchor: NonNullable<StoreControllerState["reviewThread"]>["anchor"]["value"],
): NativeReviewContext {
  const commit = activityCommit(anchor.repository_id),
    target = anchor.target.target;
  const code = ["revision", "code_text", "code_artifact", "operation"].includes(
    target,
  );
  return {
    commits: code ? [commit] : [],
    files: code ? sampleDetail.files : [],
    patches: code ? { ...sampleDetail.patches } : {},
    agents: {
      sessions:
        target === "agent_text" || target === "agent_artifact"
          ? [agentDetail]
          : [],
      error: "",
    },
    codeError: "",
  };
}
