import { ApiError, pretty, type Model } from "../api/client.js";
import { boundedReads } from "./bounded-reads.js";
import type { ThreadTransport } from "./thread-types.js";
type Options = {
  source: () => ThreadTransport;
  principal: () => string;
  generation: () => number;
  valid: (ticket: number, transport: ThreadTransport) => boolean;
};
export function createReactionLibrary(options: Options) {
  let definitions = $state<Model<"CustomEmojiVersionV1">[]>([]);
  let versions = $state<Record<string, Model<"CollabObjectRef">>>({});
  let urls = $state<Record<string, string>>({});
  let epoch = 0,
    loaded = false;
  let load: { ticket: number; promise: Promise<void> } | undefined;
  const queue = boundedReads(4);
  const revoke = (values: string[]) =>
    values.forEach((url) => URL.revokeObjectURL(url));
  async function read() {
    const transport = options.source(),
      ticket = options.generation(),
      stamp = epoch;
    const next: Model<"CustomEmojiVersionV1">[] = [],
      refs: typeof versions = {},
      assets: typeof urls = {},
      created: string[] = [];
    let cursor: string | undefined,
      pin: string | undefined,
      admitted = false;
    const valid = () => stamp === epoch && options.valid(ticket, transport);
    try {
      for (let page = 0; page < 16; page++) {
        const p = await transport.emojis(cursor);
        if (!valid()) return;
        if (p.items.type !== "emojis") throw new ApiError("invalid_emoji_page");
        if (pin && pin !== pretty(p.snapshot))
          throw new ApiError("emoji_snapshot_changed");
        pin = pretty(p.snapshot);
        next.push(...p.items.definitions);
        Object.assign(refs, p.items.versions);
        const previews = await Promise.allSettled(
          p.items.definitions.map((d) =>
            queue(async () => {
              if (!valid()) return;
              const previous = definitions.find(
                (old) => old.emoji_id === d.emoji_id,
              );
              if (previous?.preview.id === d.preview.id && urls[d.emoji_id]) {
                assets[d.emoji_id] = urls[d.emoji_id]!;
              } else {
                const asset = await transport.asset(d.preview.id);
                if (!valid()) return;
                const url = URL.createObjectURL(
                  new Blob([new Uint8Array(asset.bytes)], { type: asset.type }),
                );
                created.push(url);
                assets[d.emoji_id] = url;
              }
            }),
          ),
        );
        const failed = previews.find((result) => result.status === "rejected");
        if (failed?.status === "rejected") throw failed.reason;
        if (!valid()) return;
        if (!p.next_cursor) {
          revoke(
            Object.values(urls).filter(
              (url) => !Object.values(assets).includes(url),
            ),
          );
          definitions = next;
          versions = refs;
          urls = assets;
          loaded = true;
          admitted = true;
          return;
        }
        cursor = p.next_cursor;
      }
      throw new ApiError("emoji_library_limit");
    } finally {
      if (!admitted) revoke(created);
    }
  }
  return {
    get definitions() {
      return definitions;
    },
    get versions() {
      return versions;
    },
    get urls() {
      return urls;
    },
    get emojis() {
      return definitions
        .filter((d) => !d.retired && urls[d.emoji_id])
        .map((d) => ({
          shortcode: d.shortcode,
          url: urls[d.emoji_id]!,
          name: d.label,
          ownerPubkey: d.actor,
          owned: d.actor === options.principal(),
        }));
    },
    load() {
      if (loaded) return Promise.resolve();
      const ticket = options.generation();
      if (load?.ticket === ticket) return load.promise;
      const promise = read().finally(() => {
        if (load?.promise === promise) load = undefined;
      });
      load = { ticket, promise };
      return promise;
    },
    invalidate() {
      epoch++;
      loaded = false;
      load = undefined;
    },
    clear() {
      this.invalidate();
      definitions = [];
      versions = {};
      revoke(Object.values(urls));
      urls = {};
    },
  };
}
