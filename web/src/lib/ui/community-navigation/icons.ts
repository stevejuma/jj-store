import FolderRootIcon from "@lucide/svelte/icons/folder-root";
import GitMergeIcon from "@lucide/svelte/icons/git-merge";
import BriefcasePlusIcon from "@lucide/svelte/icons/briefcase-plus";
import GitBranchPlusIcon from "@lucide/svelte/icons/git-branch-plus";
import ServerIcon from "@lucide/svelte/icons/server";
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
  if (item.kind === "add-project") return BriefcasePlusIcon;
  if (item.kind === "add-repository") return GitBranchPlusIcon;
  if (item.kind === "create-server-repository") return ServerIcon;
  if (item.kind === "add-local-repository") return FolderGit2Icon;
  if (item.kind === "channel") return HashIcon;
  if (["threads", "more-threads", "retry-threads"].includes(item.kind))
    return MessagesSquareIcon;
  if (item.kind === "projects") return FolderRootIcon;
  if (item.kind === "project") return FolderKanbanIcon;
  if (item.kind === "stacks") return GitMergeIcon;
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
