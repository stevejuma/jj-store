import { untrack } from "svelte";
import { DirectoryClient } from "../api/directory-client.js";
import { ApiError, type Model } from "../api/client.js";
import {
  initialDirectoryState,
  emptyProfile as empty,
} from "./directory-model.js";
import { createDirectoryActions } from "./directory-actions.svelte.js";
import { directoryMedia } from "./directory-media.js";
import type { ApplicationFreshness } from "./application-freshness.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
type Profile = Model<"UserProfile">;
type Emoji = Model<"LibraryEmoji">;
const copy = <T>(value: T): T => structuredClone($state.snapshot(value)) as T;
export function createDirectory(options: {
  available(): boolean;
  token(): string | null;
  denied?(): void;
  fetcher?: typeof fetch;
  freshness?: ApplicationFreshness;
}) {
  const client = new DirectoryClient(options.token, options.fetcher);
  const state = $state(initialDirectoryState());
  let epoch = 0,
    observing: Promise<void> | undefined;
  const loaded = new Set<string>();
  const dirty = () =>
    !!state.profile &&
    JSON.stringify(state.draft) !== JSON.stringify(state.profile.details);
  const image = directoryMedia(
    client,
    () => epoch,
    () => state.urls,
    (id, url) => {
      state.urls = { ...state.urls, [id]: url };
    },
  );
  async function loadProfiles(ids: readonly string[], force = false) {
    const ticket = epoch;
    const missing = [...new Set(ids)].filter(
      (id) => /^[0-9a-f]{32}$/.test(id) && (force || !loaded.has(id)),
    );
    missing.forEach((id) => loaded.add(id));
    for (let i = 0; i < missing.length; i += 100) {
      const values = await client.request<Model<"PublicProfile">[]>(
        `profiles?ids=${missing.slice(i, i + 100).join(",")}`,
      );
      if (ticket !== epoch) return;
      for (const value of values) {
        if (value.avatar) await image(value.avatar);
        if (ticket !== epoch) return;
        state.profiles = { ...state.profiles, [value.principal]: value };
      }
    }
  }
  async function observe() {
    if (!options.available() || state.busy) return;
    if (observing) return observing;
    const ticket = epoch;
    state.loading = true;
    observing = (async () => {
      try {
        const [profile, emojis] = await Promise.all([
          client.request<Profile>("profile"),
          client.request<Emoji[]>("emojis"),
        ]);
        if (ticket !== epoch || !options.available()) return;
        state.conflict = dirty() && state.baseRevision !== profile.revision;
        if (!dirty() && !state.pending) {
          state.draft = copy(profile.details);
          state.baseRevision = profile.revision;
        }
        state.profile = profile;
        state.emojis = emojis;
        state.error = "";
        await loadProfiles([profile.principal, ...loaded], true);
        await Promise.all(
          emojis
            .filter((e) => !e.retired)
            .map((e) => image(`blob:${e.preview}`)),
        );
      } catch (cause) {
        if (ticket === epoch)
          state.error =
            "Profile settings could not be loaded. Your drafts are retained.";
      } finally {
        if (ticket === epoch) {
          state.loading = false;
          observing = undefined;
        }
      }
    })();
    return observing;
  }
  const management = {
    state,
    available: options.available,
    get dirty() {
      return dirty();
    },
    observe,
    image,
    loadProfiles,
    ...createDirectoryActions({
      state,
      client,
      image,
      observe,
      epoch: () => epoch,
    }),
    async selection(emoji: { shortcode: string; url: string }) {
      const item = state.emojis.find(
        (e) =>
          !e.retired &&
          e.shortcode === emoji.shortcode &&
          state.urls[e.preview] === emoji.url,
      );
      if (!item) throw new ApiError("custom_emoji_unavailable");
      return client.request<Model<"EmojiSelection">>(
        `emoji-snapshot/${item.id}?revision=${item.revision}`,
      );
    },
    async snapshots(body: string) {
      const values = management.emojis.filter((e) =>
        body.includes(`:${e.shortcode}:`),
      );
      if (values.length > 32) throw new ApiError("message_emoji_limit");
      const selections = [];
      for (const emoji of values)
        selections.push((await management.selection(emoji)).snapshot);
      return selections;
    },
    member(id: string, fallback: CommunityMemberModel): CommunityMemberModel {
      const profile = state.profiles[id];
      if (!profile) return fallback;
      return {
        ...fallback,
        id,
        name: profile.display_name || fallback.name,
        about: profile.description,
        initials: (profile.display_name || fallback.name)
          .slice(0, 2)
          .toUpperCase(),
        avatarUrl: profile.avatar?.startsWith("blob:")
          ? state.urls[profile.avatar.slice(5)]
          : (profile.avatar ?? undefined),
      };
    },
    get emojis() {
      return state.emojis
        .filter((e) => !e.retired && state.urls[e.preview])
        .map((e) => ({
          shortcode: e.shortcode,
          url: state.urls[e.preview]!,
          ownerPubkey: e.owner,
          owned: e.owner === state.profile?.principal,
        }));
    },
    clear() {
      epoch++;
      loaded.clear();
      observing = undefined;
      state.loading = false;
      state.error = state.validation = state.message = state.outcome = "";
      state.emojiDraft = {
        shortcode: "",
        asset: "",
        filename: "",
        selected: undefined,
      };
      state.busy = false;
      state.baseRevision = null;
      state.profile = null;
      state.draft = empty();
      state.pending = null;
      state.profiles = {};
      state.emojis = [];
      Object.values(state.urls).forEach(URL.revokeObjectURL);
      state.urls = {};
    },
    destroy() {
      management.clear();
      client.close();
      stop();
    },
  };
  const stop = $effect.root(() => {
    $effect(() => {
      options.token();
      const available = options.available();
      untrack(() => {
        management.clear();
        if (available) void observe();
      });
    });
    const watch = options.freshness?.watch({
      reset: () => {},
      matches: (change) =>
        ["profiles", "emoji_library"].includes(change.domain),
      refresh: async () => {
        await observe();
        return !state.error;
      },
    });
    return () => watch?.stop();
  });
  return management;
}
export type DirectoryManagement = ReturnType<typeof createDirectory>;
