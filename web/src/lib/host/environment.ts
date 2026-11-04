import { browserHost, type HostEnvironment } from "./contracts.js";
import {
  nativeMetadata,
  type HostWindow,
  type NativeMetadata,
} from "./native-metadata.js";

export async function resolveHostEnvironment({
  development = false,
  desktopPreview = false,
  target = typeof window === "undefined" ? undefined : (window as HostWindow),
  loadNative = async (metadata) =>
    (await import("./tauri.js")).createNativeHost(metadata),
}: {
  development?: boolean;
  desktopPreview?: boolean;
  target?: HostWindow;
  loadNative?: (metadata: NativeMetadata) => Promise<HostEnvironment>;
} = {}): Promise<HostEnvironment> {
  const metadata = nativeMetadata(target);
  if (metadata) return loadNative(metadata);
  return development && desktopPreview
    ? Object.freeze({ presentation: "macos" })
    : browserHost;
}
