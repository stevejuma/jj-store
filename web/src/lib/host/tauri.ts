import { invoke } from "@tauri-apps/api/core";
import type { HostEnvironment } from "./contracts.js";
import type { NativeMetadata } from "./native-metadata.js";
import { nativeTerminalBridge } from "./tauri-terminal.js";
export function createNativeHost(metadata: NativeMetadata): HostEnvironment {
  return Object.freeze({
    presentation: "macos",
    windowDragging: true,
    ...(metadata.terminal === 1 ? { terminal: nativeTerminalBridge() } : {}),
    setGlassBackground: (enabled: boolean) =>
      invoke("set_glass_background", { enabled }),
    agentPower: (active: boolean, activity: number) =>
      invoke("agent_power", { active, activity }),
  });
}
