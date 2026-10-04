import { ApiError, responseBytes } from "./client.js";
export async function readReactionAsset(response: Response) {
  if (!response.ok) {
    await response.body?.cancel();
    throw new ApiError("asset_unavailable");
  }
  const type = response.headers.get("content-type")?.split(";")[0] ?? "";
  if (!["image/png", "image/jpeg", "image/gif", "image/webp"].includes(type)) {
    await response.body?.cancel();
    throw new ApiError("unsupported_reaction_asset_media");
  }
  return { bytes: await responseBytes(response, 2 * 1024 * 1024), type };
}
