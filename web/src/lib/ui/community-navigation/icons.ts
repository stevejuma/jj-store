import HashIcon from "@lucide/svelte/icons/hash";
import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
import BotIcon from "@lucide/svelte/icons/bot";
import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
import GitBranchIcon from "@lucide/svelte/icons/git-branch";
import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
import HistoryIcon from "@lucide/svelte/icons/history";
import MessageSquareIcon from "@lucide/svelte/icons/message-square";
import type { Component } from "svelte";
import type { CommunityNavigationItemModel } from "./model.js";

export function navigationItemIcon(
  item: CommunityNavigationItemModel,
): Component {
  if (item.kind === "channel") return HashIcon;
  if (["threads", "more-threads", "retry-threads"].includes(item.kind))
    return MessagesSquareIcon;
  if (item.kind === "projects" || item.kind === "project")
    return FolderKanbanIcon;
  if (item.kind === "branches") return GitBranchIcon;
  if (item.kind === "log") return GitCommitIcon;
  if (item.kind === "oplog") return HistoryIcon;
  if (item.kind === "agents") return BotIcon;
  if (item.kind === "reviews") return MessageSquareIcon;
  return FolderGit2Icon;
}

export function navigationSectionIcon(section?: string): Component {
  if (section === "threads") return MessagesSquareIcon;
  if (section === "channels") return HashIcon;
  if (section === "projects") return FolderKanbanIcon;
  return FolderGit2Icon;
}
