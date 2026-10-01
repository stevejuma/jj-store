import { ApiError, pretty, type Model } from "../api/client.js";
import {
  reactionModels,
  targetKey,
} from "../collaboration/thread-presentation.js";
import type {
  ReactionItems,
  ReactionRequest,
  ReactionTarget,
  ThreadRequest,
  ThreadTransport,
} from "./thread-types.js";
import { collabRequestId } from "./collab-actions.svelte.js";
export function createMessageReactions(options: {
  source: () => ThreadTransport;
  principal: () => string;
  generation: () => number;
  valid: (ticket: number, transport: ThreadTransport) => boolean;
  execute: (
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ) => Promise<void>;
}) {
  let reactions = $state<Record<string, ReactionItems>>({});
  let errors = $state<Record<string, string>>({});
  let emojiDefinitions = $state<Model<"CustomEmojiVersionV1">[]>([]);
  let emojiVersions = $state<Record<string, Model<"CollabObjectRef">>>({});
  let emojiUrls = $state<Record<string, string>>({});
  let revision = $state(0),
    epoch = 0;
  let emojiLoad: { ticket: number; promise: Promise<void> } | undefined;
  let loaded = false;
  const loads = new Map<string, Promise<void>>();
  const revoke = (urls: Record<string, string>) =>
    Object.values(urls).forEach(URL.revokeObjectURL);
  async function loadLibrary() {
    const transport = options.source(),
      ticket = options.generation(),
      stamp = epoch;
    const definitions: Model<"CustomEmojiVersionV1">[] = [],
      versions: Record<string, Model<"CollabObjectRef">> = {},
      urls: Record<string, string> = {};
    let cursor: string | undefined,
      pin: string | undefined,
      admitted = false;
    try {
      for (let page = 0; page < 16; page++) {
        const p = await transport.emojis(cursor);
        if (!options.valid(ticket, transport) || stamp !== epoch) return;
        if (p.items.type !== "emojis") throw new ApiError("invalid_emoji_page");
        if (pin && pin !== pretty(p.snapshot))
          throw new ApiError("emoji_snapshot_changed");
        pin = pretty(p.snapshot);
        definitions.push(...p.items.definitions);
        Object.assign(versions, p.items.versions);
        for (const d of p.items.definitions) {
          const asset = await transport.asset(d.preview.id);
          if (!options.valid(ticket, transport) || stamp !== epoch) return;
          urls[d.emoji_id] = URL.createObjectURL(
            new Blob([new Uint8Array(asset.bytes)], { type: asset.type }),
          );
        }
        if (!p.next_cursor) {
          revoke(emojiUrls);
          emojiDefinitions = definitions;
          emojiVersions = versions;
          emojiUrls = urls;
          admitted = true;
          loaded = true;
          return;
        }
        cursor = p.next_cursor;
      }
      throw new ApiError("emoji_library_limit");
    } finally {
      if (!admitted) revoke(urls);
    }
  }
  function loadEmojis() {
    if (loaded) return Promise.resolve();
    const ticket = options.generation();
    if (emojiLoad?.ticket === ticket) return emojiLoad.promise;
    const promise = loadLibrary().finally(() => {
      if (emojiLoad?.promise === promise) emojiLoad = undefined;
    });
    emojiLoad = { ticket, promise };
    return promise;
  }
  async function readReactions(target: ReactionTarget) {
    const key = targetKey(target),
      transport = options.source(),
      ticket = options.generation(),
      stamp = epoch;
    let cursor: string | undefined,
      combined: ReactionItems | undefined,
      pin: string | undefined;
    for (let page = 0; page < 32; page++) {
      const p = await transport.reactions(target, cursor);
      if (!options.valid(ticket, transport) || stamp !== epoch) return;
      if (p.items.type !== "reactions" || targetKey(p.items.target) !== key)
        throw new ApiError("invalid_reaction_page");
      if (pin && pin !== pretty(p.snapshot))
        throw new ApiError("reaction_snapshot_changed");
      pin = pretty(p.snapshot);
      combined = combined
        ? { ...combined, actors: [...combined.actors, ...p.items.actors] }
        : p.items;
      if (!p.next_cursor) {
        reactions = { ...reactions, [key]: combined };
        return;
      }
      cursor = p.next_cursor;
    }
    throw new ApiError("reaction_actor_limit");
  }
  function loadReactions(target: ReactionTarget, force = false) {
    const key = targetKey(target);
    if (reactions[key] && !force) return Promise.resolve();
    const loadKey = `${epoch}:${options.generation()}:${key}`;
    if (loads.has(loadKey)) return loads.get(loadKey)!;
    const promise = readReactions(target).finally(() => {
      loads.delete(loadKey);
    });
    loads.set(loadKey, promise);
    return promise;
  }
  const custom = (
    value: Extract<Model<"ReactionValueV1">, { type: "custom" }>,
  ) => {
    const d = emojiDefinitions.find(
      (d) =>
        d.emoji_id === value.emoji_id &&
        emojiVersions[d.emoji_id]?.id === value.version.id,
    );
    return d
      ? { emoji: d.shortcode, url: emojiUrls[d.emoji_id], label: d.label }
      : { emoji: ":custom:", label: "Retained custom reaction" };
  };
  const glyph = (v: Model<"ReactionValueV1">) =>
    v.type === "custom"
      ? custom(v).emoji
      : v.type === "like"
        ? "👍"
        : v.type === "dislike"
          ? "👎"
          : v.sequence;
  async function performToggle(
    target: ReactionTarget,
    emoji: string,
    customEmoji?: { shortcode: string; url: string },
    fromBar = false,
  ) {
    const ticket = options.generation(),
      transport = options.source();
    await loadReactions(target);
    if (!options.valid(ticket, transport)) return;
    const items = reactions[targetKey(target)];
    if (!items) throw new ApiError("reaction_snapshot_unavailable");
    const actor = items.actors.find((a) => a.actor === options.principal());
    if (actor?.conflict)
      throw new ApiError("reaction_conflict_requires_resolution");
    const d = customEmoji
      ? emojiDefinitions.find(
          (d) =>
            d.shortcode === customEmoji.shortcode &&
            emojiUrls[d.emoji_id] === customEmoji.url &&
            !d.retired,
        )
      : undefined;
    if (customEmoji && !d && !fromBar)
      throw new ApiError("custom_emoji_unavailable");
    const barValue = fromBar
      ? actor?.value && glyph(actor.value) === emoji
        ? actor.value
        : items.reactions.find((r) => glyph(r.value) === emoji)?.value
      : undefined;
    const value: Model<"ReactionValueV1"> =
      barValue ??
      (d
        ? {
            type: "custom",
            emoji_id: d.emoji_id,
            version: emojiVersions[d.emoji_id]!,
          }
        : emoji === "👍"
          ? { type: "like" }
          : emoji === "👎"
            ? { type: "dislike" }
            : { type: "emoji", sequence: emoji });
    const selected = actor?.value && pretty(actor.value) === pretty(value);
    const command: ReactionRequest["command"] = selected
      ? { type: "clear", target, observed: actor.active }
      : { type: "set", target, value, observed: actor?.active ?? [] };
    await options.execute("reactions", {
      version: 1,
      request_id: collabRequestId(),
      command,
    });
    if (options.valid(ticket, transport)) await loadReactions(target, true);
  }
  async function toggle(...args: Parameters<typeof performToggle>) {
    const key = targetKey(args[0]),
      ticket = options.generation(),
      transport = options.source(),
      stamp = epoch;
    errors = { ...errors, [key]: "" };
    try {
      await performToggle(...args);
    } catch (cause) {
      if (stamp === epoch && options.valid(ticket, transport))
        errors = {
          ...errors,
          [key]: cause instanceof Error ? cause.message : String(cause),
        };
      throw cause;
    }
  }
  function invalidate() {
    epoch++;
    revision++;
    reactions = {};
    errors = {};
    loaded = false;
    emojiLoad = undefined;
  }
  return {
    get revision() {
      return revision;
    },
    get emojis() {
      return emojiDefinitions
        .filter((d) => !d.retired && emojiUrls[d.emoji_id])
        .map((d) => ({
          shortcode: d.shortcode,
          url: emojiUrls[d.emoji_id]!,
          name: d.label,
          ownerPubkey: d.actor,
          owned: d.actor === options.principal(),
        }));
    },
    getError(target: ReactionTarget) {
      return errors[targetKey(target)] ?? "";
    },
    getReaction(target: ReactionTarget) {
      return reactions[targetKey(target)];
    },
    reactionModels(target: ReactionTarget) {
      const items = reactions[targetKey(target)];
      return items ? reactionModels(items, custom) : [];
    },
    loadEmojis,
    loadReactions,
    toggle,
    invalidate,
    clear() {
      invalidate();
      emojiDefinitions = [];
      emojiVersions = {};
      revoke(emojiUrls);
      emojiUrls = {};
    },
  };
}
