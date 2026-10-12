import { getContext, setContext, untrack } from "svelte";
import type { DirectoryManagement } from "../controller/directory.svelte.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
const key = Symbol("installation profiles");
type Context = { get directory(): DirectoryManagement | undefined };
export function provideProfiles(
  directory: () => DirectoryManagement | undefined,
) {
  setContext<Context>(key, {
    get directory() {
      return directory();
    },
  });
}
export function useProfileDirectory() {
  return getContext<Context | undefined>(key);
}
export function useProfileMembers(
  members: () => readonly CommunityMemberModel[],
) {
  const context = getContext<Context | undefined>(key);
  $effect(() => {
    const directory = context?.directory,
      ids = members()
        .filter((m) => m.kind !== "agent")
        .map((m) => m.id);
    const available = directory?.available();
    untrack(() => {
      if (available) void directory?.loadProfiles(ids).catch(() => {});
    });
  });
  return {
    get values() {
      const directory = context?.directory;
      return members().map((m) =>
        m.kind === "agent" ? m : (directory?.member(m.id, m) ?? m),
      );
    },
  };
}

/** Project saved human profiles while retaining each authored item's identity. */
export function useProfileAuthors<T extends { author?: CommunityMemberModel }>(
  items: () => readonly T[],
) {
  const members = useProfileMembers(() =>
    items().flatMap((item) => (item.author ? [item.author] : [])),
  );
  return {
    get values() {
      const current = members.values;
      return items().map((item) => ({
        ...item,
        author: item.author
          ? current.find((member) => member.id === item.author?.id)
          : undefined,
      }));
    },
  };
}
