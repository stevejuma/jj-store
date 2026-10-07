import { mentionedAgentIds } from "../../community/agent-mentions.js";
import { contentMentionPubkeys } from "../../community/content-mentions.js";
import type { ComposerProps } from "./props.js";
export function composerSubmission(
  content: string,
  context: ComposerProps["context"],
) {
  return {
    content: content.trim(),
    mentionPubkeys: contentMentionPubkeys(content),
    agentIds: mentionedAgentIds(content),
    channelIds: [],
    attachments: [],
    ...(context ? { context } : {}),
  };
}
export function editLastComposerMessage(callback?: () => void) {
  if (!callback) return false;
  callback();
  return true;
}
