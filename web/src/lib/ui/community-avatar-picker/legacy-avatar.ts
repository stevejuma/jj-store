import { isCommunityAvatarImage } from "../../community/avatar-content.js";
/** Preview legacy emoji with Community's template; unchanged saves keep the original value. */
export function avatarPreview(value: string): string {
  if (!value || isCommunityAvatarImage(value)) return value;
  const safe = value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
  return `data:image/svg+xml,${encodeURIComponent(`<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" rx="64" fill="#6e56cf"/><text x="128" y="166" font-size="128" text-anchor="middle">${safe}</text></svg>`)}`;
}
