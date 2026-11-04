export interface NativeMetadata {
  version: number;
  platform: string;
  terminal?: number;
}
export interface HostWindow {
  __JJ_STORE_DESKTOP__?: NativeMetadata;
  __TAURI_INTERNALS__?: { invoke?: unknown };
}
/** Presentation discovery only; native ACLs admit each command independently. */
export function nativeMetadata(
  target?: HostWindow,
): NativeMetadata | undefined {
  const metadata = target?.__JJ_STORE_DESKTOP__;
  if (
    metadata?.version === 1 &&
    metadata.platform === "macos" &&
    typeof target?.__TAURI_INTERNALS__?.invoke === "function"
  )
    return metadata;
}
