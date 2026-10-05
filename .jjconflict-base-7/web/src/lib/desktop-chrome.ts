/** Presentation metadata only; this marker never authorizes repository access. */
declare global {
  interface Window {
    __JJ_STORE_DESKTOP__?: {
      version: number;
      platform: string;
      terminal?: number;
    };
  }
}

export function isMacDesktopWindow(): boolean {
  return (
    typeof window !== "undefined" &&
    window.__JJ_STORE_DESKTOP__?.version === 1 &&
    window.__JJ_STORE_DESKTOP__.platform === "macos"
  );
}
