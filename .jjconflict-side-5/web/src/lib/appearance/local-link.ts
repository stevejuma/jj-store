/** Only existing HTTP(S) address and text; never resolve metadata or images. */
export function localLink(href: unknown, label: string) {
  if (typeof href !== "string") return undefined;
  try {
    const url = new URL(href);
    if (
      !["http:", "https:"].includes(url.protocol) ||
      url.username ||
      url.password
    )
      return undefined;
    return {
      href: url.href,
      host: url.hostname,
      label: label.trim() || url.href,
    };
  } catch {
    return undefined;
  }
}
