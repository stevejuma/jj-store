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
import { boundedReads } from "./bounded-reads.js";
import { createReactionLibrary } from "./reaction-library.svelte.js";
import { collabRequestId } from "./collab-actions.svelte.js";
export function createMessageReactions(options: {
  profiles?: import("./directory.svelte.js").DirectoryManagement;
  source: () => ThreadTransport;
  principal: () => string;
  generation: () => number;
  valid: (ticket: number, transport: ThreadTransport) => boolean;
  execute: (
    kind: "threads" | "reactions",
    request: ThreadRequest | ReactionRequest,
  ) => Promise<void>;
}) {
  let profiles = options.profiles;
  let reactions = $state<Record<string, ReactionItems>>({});
  let errors = $state<Record<string, string>>({});
  let revision = $state(0),
    epoch = 0;
  const library = createReactionLibrary(options);
  const queue = boundedReads(4);
  const loads = new Map<string, Promise<void>>();
  const targetEpoch = new Map<string, number>();
  async function readReactions(
    target: ReactionTarget,
    transport: ThreadTransport,
    ticket: number,
    stamp: number,
    targetStamp: number,
  ) {
    const key = targetKey(target);
    const valid = () =>
      options.valid(ticket, transport) &&
      stamp === epoch &&
      targetStamp === (targetEpoch.get(key) ?? 0);
    if (!valid()) return;
    let cursor: string | undefined,
      combined: ReactionItems | undefined,
      pin: string | undefined;
    for (let page = 0; page < 32; page++) {
      const p = await transport.reactions(target, cursor);
      if (!valid()) return;
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
        if (profiles?.available())
          await profiles.loadProfiles(combined.actors.map((a) => a.actor));
        return;
      }
      cursor = p.next_cursor;
    }
    throw new ApiError("reaction_actor_limit");
  }
  function loadReactions(target: ReactionTarget, force = false) {
    const key = targetKey(target);
    if (reactions[key] && !force) return Promise.resolve();
    if (force) {
      targetEpoch.set(key, (targetEpoch.get(key) ?? 0) + 1);
      errors = { ...errors, [key]: "" };
    }
    const ticket = options.generation(),
      stamp = epoch,
      targetStamp = targetEpoch.get(key) ?? 0,
      transport = options.source();
    const loadKey = `${epoch}:${ticket}:${key}:${targetStamp}`;
    if (loads.has(loadKey)) return loads.get(loadKey)!;
    const promise = queue(() =>
      readReactions(target, transport, ticket, stamp, targetStamp),
    ).finally(() => {
      loads.delete(loadKey);
    });
    loads.set(loadKey, promise);
    return promise;
  }
  const custom = (
    value: Extract<Model<"ReactionValueV1">, { type: "custom" }>,
  ) => {
    const d = library.definitions.find(
      (d) =>
        d.emoji_id === value.emoji_id &&
        library.versions[d.emoji_id]?.id === value.version.id,
    );
    return d
      ? {
          emoji: d.source ? d.label : d.shortcode,
          url: library.urls[d.emoji_id],
          label: d.label,
        }
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
      ? library.definitions.find(
          (d) =>
            d.shortcode === customEmoji.shortcode &&
            library.urls[d.emoji_id] === customEmoji.url &&
            !d.retired,
        )
      : undefined;
    const selection =
      customEmoji && !d && !fromBar && profiles?.available()
        ? await profiles.selection(customEmoji)
        : undefined;
    if (customEmoji && !d && !fromBar && !selection)
      throw new ApiError("custom_emoji_unavailable");
    if (!options.valid(ticket, transport)) return;
    const barValue = fromBar
      ? actor?.value && glyph(actor.value) === emoji
        ? actor.value
        : items.reactions.find((r) => glyph(r.value) === emoji)?.value
      : undefined;
    const value: Model<"ReactionValueV1"> =
      selection?.value ??
      barValue ??
      (d
        ? {
            type: "custom",
            emoji_id: d.emoji_id,
            version: library.versions[d.emoji_id]!,
          }
        : emoji === "👍"
          ? { type: "like" }
          : emoji === "👎"
            ? { type: "dislike" }
            : { type: "emoji", sequence: emoji });
    const selected = actor?.value && pretty(actor.value) === pretty(value);
    const command: ReactionRequest["command"] = selected
      ? { type: "clear", target, observed: actor.active }
      : {
          type: "set",
          target,
          value,
          observed: actor?.active ?? [],
          ...(selection ? { emoji: selection.snapshot } : {}),
        };
    await options.execute("reactions", {
      version: 1,
      request_id: collabRequestId(),
      command,
    });
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
    targetEpoch.clear();
    library.invalidate();
  }
  return {
    setProfiles(value: typeof profiles) {
      profiles = value;
    },
    get revision() {
      return revision;
    },
    get emojis() {
      const global = profiles?.emojis ?? [];
      return [
        ...global,
        ...library.emojis.filter(
          (e) => !global.some((g) => g.shortcode === e.shortcode),
        ),
      ];
    },
    getError(target: ReactionTarget) {
      return errors[targetKey(target)] ?? "";
    },
    getReaction(target: ReactionTarget) {
      return reactions[targetKey(target)];
    },
    reactionModels(target: ReactionTarget) {
      const items = reactions[targetKey(target)];
      const models = items ? reactionModels(items, custom) : [];
      return models.map((m) => ({
        ...m,
        actors: m.actors?.map((a) => profiles?.member(a.id, a) ?? a),
      }));
    },
    loadEmojis: library.load,
    refresh: (target: ReactionTarget) => loadReactions(target, true),
    loadReactions,
    toggle,
    invalidate,
    clear() {
      invalidate();
      library.clear();
    },
  };
}
