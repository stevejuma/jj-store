import BotIcon from "@lucide/svelte/icons/bot";
import CameraIcon from "@lucide/svelte/icons/camera";
import CornerUpLeftIcon from "@lucide/svelte/icons/corner-up-left";
import DatabaseIcon from "@lucide/svelte/icons/database";
import FilesIcon from "@lucide/svelte/icons/files";
import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
import GitCommitHorizontalIcon from "@lucide/svelte/icons/git-commit-horizontal";
import HashIcon from "@lucide/svelte/icons/hash";
import HistoryIcon from "@lucide/svelte/icons/history";
import IterationCwIcon from "@lucide/svelte/icons/iteration-cw";
import ListOrderedIcon from "@lucide/svelte/icons/list-ordered";
import ListTodoIcon from "@lucide/svelte/icons/list-todo";
import LogInIcon from "@lucide/svelte/icons/log-in";
import LogOutIcon from "@lucide/svelte/icons/log-out";
import MessageCircleIcon from "@lucide/svelte/icons/message-circle";
import MessageSquareTextIcon from "@lucide/svelte/icons/message-square-text";
import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
import ScrollTextIcon from "@lucide/svelte/icons/scroll-text";
import UserRoundIcon from "@lucide/svelte/icons/user-round";
import WrenchIcon from "@lucide/svelte/icons/wrench";

const icons: Record<string, typeof HashIcon> = {
  message: MessageSquareTextIcon,
  "message ID": MessageSquareTextIcon,
  "comment ID": MessageCircleIcon,
  "parent comment ID": CornerUpLeftIcon,
  "revision ID": GitCommitHorizontalIcon,
  "author ID": UserRoundIcon,
  "thread ID": MessagesSquareIcon,
  "task ID": ListTodoIcon,
  "session ID": BotIcon,
  "parent session ID": CornerUpLeftIcon,
  "snapshot ID": CameraIcon,
  "agent root": DatabaseIcon,
  "repository ID": FolderGit2Icon,
  "epoch ID": HistoryIcon,
  "transcript ID": ScrollTextIcon,
  sequence: ListOrderedIcon,
  "turn ID": IterationCwIcon,
  "tool call ID": WrenchIcon,
  "tool name": WrenchIcon,
  input: LogInIcon,
  output: LogOutIcon,
  paths: FilesIcon,
};

export function recordCopyIcon(label: string) {
  return icons[label.replace(/^Copy /, "")] ?? HashIcon;
}
