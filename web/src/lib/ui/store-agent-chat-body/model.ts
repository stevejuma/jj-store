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
  author: string;
  text: string;
}
export interface ChatCommentThread {
  id: string;
  anchor: ChatCommentAnchor;
  comments: readonly ChatComment[];
}
export interface ChatBodyProps {
  items: readonly ChatBodyItem[];
  threads?: readonly ChatCommentThread[];
  onAddComment: (anchor: ChatCommentAnchor, text: string) => void;
  onReply: (threadId: string, text: string) => void;
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
  onInspectTool?: (tool: ChatBodyTool, opener: HTMLElement) => void;
}
