import { DirectoryClient } from "../api/directory-client.js";
import { readReactionAsset } from "../api/reaction-asset.js";
import { ApiError } from "../api/client.js";
import { boundedReads } from "./bounded-reads.js";
export function directoryMedia(
  client: DirectoryClient,
  current: () => number,
  urls: () => Record<string, string>,
  put: (id: string, url: string) => void,
) {
  const queue = boundedReads(4),
    pending = new Map<string, Promise<string>>();
  return async function image(value: string): Promise<string> {
    if (!value.startsWith("blob:")) return value;
    const id = value.slice(5),
      ticket = current(),
      key = `${ticket}:${id}`;
    if (urls()[id]) return urls()[id]!;
    if (pending.has(key)) return pending.get(key)!;
    const promise = queue(async () => {
      if (ticket !== current()) throw new ApiError("directory_session_changed");
      const media = await readReactionAsset(await client.fetch(`media/${id}`));
      if (ticket !== current()) throw new ApiError("directory_session_changed");
      const url = URL.createObjectURL(
        new Blob([new Uint8Array(media.bytes)], { type: media.type }),
      );
      put(id, url);
      return url;
    }).finally(() => {
      pending.delete(key);
    });
    pending.set(key, promise);
    return promise;
  };
}
