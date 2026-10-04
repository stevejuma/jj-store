import type { CommunityEntityLinkIcon } from "../../community/community-links.js";

import type { CommunityRoomModel } from "../community-workspace/model.js";

import type { CommunityComposerAutocompleteItem } from "./autocomplete.js";

import {
  COMPOSER_SLASH_KIND_ALIASES,
  parseComposerSlashQuery,
  type ComposerSlashKind,
} from "./composer-slash.js";

import type { ComposerSlashSession } from "./composer-slash-session.js";

export interface ComposerSlashCatalog extends ComposerSlashSession {}

export const COMPOSER_SLASH_CATEGORIES: readonly {
  kind: ComposerSlashKind;
  label: string;
  icon: CommunityEntityLinkIcon;
}[] = [
  { kind: "repositories", label: "Repositories", icon: "folder-git-2" },
  { kind: "projects", label: "Projects", icon: "folder-kanban" },
  { kind: "tasks", label: "Tasks", icon: "circle-dot" },
  { kind: "commits", label: "Commits", icon: "git-commit-horizontal" },
  { kind: "logs", label: "Logs", icon: "history" },
  { kind: "threads", label: "Threads", icon: "hash" },
];

export function mergeComposerSlashCatalog(
  rooms: readonly CommunityRoomModel[],
  session: ComposerSlashSession,
): ComposerSlashCatalog {
  return { ...session, rooms };
}

export function composerSlashAutocompleteItems(
  query: string,
  catalog: ComposerSlashCatalog,
): CommunityComposerAutocompleteItem[] {
  const parsed = parseComposerSlashQuery(query);
  const filter = parsed.filter.trim().toLocaleLowerCase();
  if (parsed.stage === "category")
    return COMPOSER_SLASH_CATEGORIES.filter(
      (c) =>
        !filter ||
        c.label.toLocaleLowerCase().startsWith(filter) ||
        COMPOSER_SLASH_KIND_ALIASES[c.kind].some((a) => a.startsWith(filter)),
    ).map((c) => ({
      kind: "slash",
      id: `slash:category:${c.kind}`,
      label: c.label,
    }));
  return catalog[parsed.kind!]
    .filter((row) =>
      `${row.label} ${row.detail ?? ""}`.toLocaleLowerCase().includes(filter),
    )
    .slice(0, 24)
    .map((row) => ({
      kind: "slash",
      id: `slash:entry:${row.href}`,
      label: row.label,
      detail: row.detail,
    }));
}

export function composerSlashItemKind(
  item: CommunityComposerAutocompleteItem,
): ComposerSlashKind | undefined {
  return /^slash:category:(.+)$/u.exec(item.id)?.[1] as
    ComposerSlashKind | undefined;
}

export function composerSlashItemHref(
  item: CommunityComposerAutocompleteItem,
): string | undefined {
  return item.id.startsWith("slash:entry:")
    ? item.id.slice("slash:entry:".length)
    : undefined;
}
