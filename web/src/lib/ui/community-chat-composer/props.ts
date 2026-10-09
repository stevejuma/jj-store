import type { Snippet } from "svelte";
import type { ComposerStatus } from "@lapismd/design-core/ai/chat";
import type { CommunityCustomEmoji } from "../../community/contracts.js";
import type {
  CommunityMemberModel,
  CommunityRoomModel,
} from "../community-workspace/model.js";
import type {
  CommunityComposerContext,
  CommunityComposerSubmission,
} from "./model.js";
export interface ComposerProps {
  presentation?: "document" | "classic";
  member?: CommunityMemberModel;
  hideAvatar?: boolean;
  dockActions?: boolean;
  documentActions?: Snippet;
  value?: string;
  formattingOpen?: boolean;
  placeholder?: string;
  members?: readonly CommunityMemberModel[];
  audienceKey?: string;
  initialAgentIds?: readonly string[];
  rooms?: readonly CommunityRoomModel[];
  communityEmojis?: readonly CommunityCustomEmoji[];
  context?: CommunityComposerContext;
  disabled?: boolean;
  disabledReason?: string;
  composerStatus?: ComposerStatus;
  busy?: boolean;
  ariaLabel?: string;
  submitLabel?: string;
  mode?: "chat" | "form";
  allowEmpty?: boolean;
  enterInsertsNewline?: boolean;
  showSubmitButton?: boolean;
  clearOnSubmit?: boolean;
  additionalActions?: Snippet;
  onSubmit?: (submission: CommunityComposerSubmission) => void | Promise<void>;
  onCancel?: () => void;
  onCancelContext?: () => void;
  onEditLastMessage?: () => void;
  onOpenMember?: (member: CommunityMemberModel) => void | Promise<void>;
  onMessageMember?: (member: CommunityMemberModel) => void | Promise<void>;
  onWaveMember?: (member: CommunityMemberModel) => void | Promise<void>;
  onRetryComposerStatus?: () => void | Promise<void>;
  onDismissComposerStatus?: () => void;
  class?: string;
  ref?: HTMLDivElement | null;
}
