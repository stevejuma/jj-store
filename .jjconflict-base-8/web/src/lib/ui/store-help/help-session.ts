/** Documentation position is presentation-only and never persisted to browser storage. */
export const helpSession = { path: "/help/", scrollY: 0 };
export function admittedHelpMessage(
  event: MessageEvent,
  frame: Window | null,
  origin: string,
) {
  return (
    frame !== null &&
    event.source === frame &&
    event.origin === origin &&
    (event.data?.type === "jj-store-help-ready" ||
      event.data?.type === "jj-store-help-close" ||
      event.data?.type === "jj-store-help-focus-close")
  );
}
