import type { ChatBodyComments } from "./comments.svelte.js";
import type { Snippet } from "svelte";
import type { CommunityReactionModel } from "../../community/workspace-model.js";
import type { CommunityCustomEmoji } from "../../community/contracts.js";
import type { CommunityMemberModel } from "../community-workspace/model.js";

/** Immutable captured content; IDs and revisions belong to the caller. */
export interface ChatBodyBlock {
  id: string;
  revision: string;
  kind: "markdown" | "code" | "patch" | "custom" | "context";
  label: string;
  text: string;
  path?: string;
  /** Captured read output includes line-number/tab prefixes in its exact text. */
  lineNumbered?: boolean;
  /** UTF-16 start within the input supplied to markdownBlocks, before trimming. */
  sourceFrom?: number;
}
export interface ChatBodyTool {
  id: string;
  revision: string;
  title: string;
  status: "complete" | "error" | "running" | "unknown";
  blocks: readonly ChatBodyBlock[];
  raw: unknown;
  detailPlacement?: "inline" | "sidebar";
}
export interface ChatBodyTurn {
  id: string;
  author: string;
  prompt: string;
  summary: string;
  agentName?: string;
  modelLabel?: string;
  createdAt?: number;
}
export interface ChatBodyItem {
  id: string;
  kind: "message" | "tools" | "context" | "reasoning" | "handoff";
  label: string;
  role?: "user" | "assistant";
  turn?: ChatBodyTurn;
  author?: CommunityMemberModel;
  blocks: readonly ChatBodyBlock[];
  tools?: readonly ChatBodyTool[];
  defaultExpanded?: boolean;
  commentBatchId?: string;
  replyContext?: { threadId: string; commentIds: readonly string[] };
}
/** UTF-16 offsets into one exact block; custom blocks use the entire range. */
export interface ChatCommentAnchor {
  blockId: string;
  revision: string;
  from: number;
  to: number;
  quote: string;
}
export interface ChatComment {
  id: string;
  author: CommunityMemberModel;
  text: string;
  status: "draft" | "sending" | "sent" | "replied";
  batchId?: string;
  replyItemId?: string;
  replyTo?: string;
  createdAt?: number;
}
export interface ChatCommentThread {
  id: string;
  anchor: ChatCommentAnchor;
  label: string;
  sourceKind?: ChatBodyBlock["kind"];
  comments: ChatComment[];
}
export interface ChatPromptComment {
  threadId: string;
  commentId: string;
  anchor: ChatCommentAnchor;
  author: CommunityMemberModel;
  text: string;
}
export interface ChatPromptSubmission {
  id: string;
  text: string;
  comments: readonly ChatPromptComment[];
}
export interface ChatCommentHost {
  items: readonly ChatBodyItem[];
  author?: CommunityMemberModel;
  onSend?: (submission: ChatPromptSubmission) => void | Promise<void>;
  onInspectTool?: (
    tool: ChatBodyTool,
    opener: HTMLElement,
  ) => void | Promise<void>;
}
export interface ChatBodyProps extends ChatCommentHost {
  reactions?: Readonly<Record<string, readonly CommunityReactionModel[]>>;
  reactionOptions?: readonly string[];
  onReact?: (
    messageId: string,
    emoji: string,
    custom?: CommunityCustomEmoji,
  ) => void;
  composer?: Snippet;
  footer?: Snippet;
  label?: string;
  customBlock?: Snippet<[ChatBodyBlock]>;
  commentState?: ChatBodyComments;
}
