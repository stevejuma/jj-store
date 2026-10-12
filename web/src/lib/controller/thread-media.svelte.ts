import { boundedReads } from "./bounded-reads.js";
import type { ThreadTransport } from "./thread-types.js";
import type { Model } from "../api/client.js";
export function createThreadMedia() {
  let urls = $state<Record<string, string>>({}),
    epoch = 0;
  const queue = boundedReads(4);
  return {
    get urls() {
      return urls;
    },
    async load(
      events: readonly Model<"ThreadEventItemV1">[],
      transport: ThreadTransport,
      valid: () => boolean,
    ) {
      const ticket = epoch;
      const ids = [
        ...new Set(
          events.flatMap((e) =>
            Object.values(e.emojis ?? {}).map((d) => d.preview.id),
          ),
        ),
      ];
      await Promise.all(
        ids
          .filter((id) => !urls[id])
          .map((id) =>
            queue(async () => {
              if (ticket !== epoch || !valid()) return;
              const asset = await transport.asset(id);
              if (ticket !== epoch || !valid()) return;
              const url = URL.createObjectURL(
                new Blob([new Uint8Array(asset.bytes)], { type: asset.type }),
              );
              if (urls[id]) URL.revokeObjectURL(url);
              else urls = { ...urls, [id]: url };
            }),
          ),
      );
    },
    clear() {
      epoch++;
      Object.values(urls).forEach(URL.revokeObjectURL);
      urls = {};
    },
  };
}
