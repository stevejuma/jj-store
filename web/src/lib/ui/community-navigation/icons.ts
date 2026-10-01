import BotIcon from "@lucide/svelte/icons/bot";
import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
import GitBranchIcon from "@lucide/svelte/icons/git-branch";
import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
import HistoryIcon from "@lucide/svelte/icons/history";
import MessageSquareIcon from "@lucide/svelte/icons/message-square";
import type { Component } from "svelte";
import type { CommunityNavigationItemModel } from "./model.js";

export function navigationItemIcon(
  item: CommunityNavigationItemModel,
): Component {
  if (item.kind === "projects" || item.kind === "project")
    return FolderGit2Icon;
  if (item.kind === "branches") return GitBranchIcon;
  if (item.kind === "log") return GitCommitIcon;
  if (item.kind === "oplog") return HistoryIcon;
  if (item.kind === "agents") return BotIcon;
  if (item.kind === "reviews") return MessageSquareIcon;
  return FolderGit2Icon;
}

export const navigationSectionIcon = (_section?: string): Component =>
  FolderGit2Icon;
