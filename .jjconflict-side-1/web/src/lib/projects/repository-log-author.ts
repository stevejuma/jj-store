import { md5 } from "@noble/hashes/legacy.js";
import { bytesToHex, utf8ToBytes } from "@noble/hashes/utils.js";

/** Match Community's normalized Gravatar identicon fallback for Git authors. */
export function repositoryLogGravatarUrl(email: string): string | undefined {
  const normalized = email.trim().toLowerCase();
  if (/^npub1[a-z0-9]+@nostr\.invalid$/u.test(normalized)) return;
  if (!normalized.includes("@") || normalized.startsWith("@")) return;
  return `https://www.gravatar.com/avatar/${bytesToHex(md5(utf8ToBytes(normalized)))}?d=identicon&s=80`;
}
