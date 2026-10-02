import type { Model } from "../api/client.js";
import type { StoreControllerState } from "../controller/contracts.js";
import type { RepositoryLogRevision } from "../projects/repository-log.js";
import type { CommunityMemberModel } from "../ui/community-workspace/model.js";
import {
  agentMember,
  anchorAgentMember,
  anchorMatchesOperation,
  principalMember,
} from "./presentation.js";

export interface RevisionCollaboration {
  members: CommunityMemberModel[];
  replyCount: number;
}

function associationMatches(
  revision: RepositoryLogRevision,
  association: Model<"AgentCodeAssociation">,
): boolean {
  return (
    association.before_commits.includes(revision.eventId) ||
    association.after_commits.includes(revision.eventId) ||
    association.before_operations.includes(revision.eventId) ||
    association.after_operations.includes(revision.eventId) ||
    association.changes.includes(revision.changeId)
  );
}

function anchorMatches(
  revision: RepositoryLogRevision,
  anchor: Model<"ArtifactAnchorV1">,
  relatedSessions: ReadonlySet<string>,
): boolean {
  const target = anchor.target;
  if (
    target.target === "revision" ||
    target.target === "code_text" ||
    target.target === "code_artifact"
  )
    return (
      target.value.commit_id === revision.eventId ||
      target.value.change_id === revision.changeId ||
      anchorMatchesOperation(anchor, revision.eventId)
    );
  if (target.target === "operation")
    return anchorMatchesOperation(anchor, revision.eventId);
  if (
    target.target === "agent_text" ||
    target.target === "agent_artifact" ||
    target.target === "agent_session"
  )
    return relatedSessions.has(target.value.session_id);
  return false;
}

export function revisionCollaboration(
  revision: RepositoryLogRevision,
  state: StoreControllerState,
): RevisionCollaboration | undefined {
  const sessions = [
    ...state.agentSessions,
    ...(state.agentSession ? [state.agentSession.session] : []),
  ];
  const associations = [
    ...state.agentRelations.flatMap((relation) =>
      relation.association ? [relation.association] : [],
    ),
    ...(state.agentSession?.code ?? []),
  ].filter((association) => associationMatches(revision, association));
  const relatedSessions = new Set(
    associations.map((association) => association.session_id),
  );
  const threads = state.reviewThreads.filter((thread) =>
    anchorMatches(revision, thread.anchor, relatedSessions),
  );
  const members: CommunityMemberModel[] = [];
  for (const sessionId of relatedSessions) {
    const session = sessions.find(
      (candidate) => candidate.session_id === sessionId,
    );
    if (session) members.push(agentMember(session));
  }
  for (const thread of threads) {
    const agent = anchorAgentMember(thread.anchor, sessions);
    if (agent) members.push(agent);
  }
  if (
    state.reviewThread &&
    threads.some(
      (thread) => thread.thread_id === state.reviewThread?.thread.thread_id,
    )
  )
    for (const comment of state.reviewThread.comments)
      members.push(principalMember(comment.author));
  const uniqueMembers = [
    ...new Map(members.map((member) => [member.id, member])).values(),
  ];
  const replyCount = threads.reduce(
    (total, thread) => total + Number(thread.comment_count),
    0,
  );
  return uniqueMembers.length || replyCount
    ? { members: uniqueMembers, replyCount }
    : undefined;
}
