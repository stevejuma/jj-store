import type { TerminalBridge } from "../terminal/bridge.js";

/** Renderer integration only. Native permissions remain enforced by the host. */
export interface HostEnvironment {
  readonly presentation: "browser" | "macos";
  readonly terminal?: TerminalBridge;
  readonly windowDragging?: true;
  readonly setGlassBackground?: (enabled: boolean) => Promise<unknown>;
  readonly agentPower?: (active: boolean, activity: number) => Promise<unknown>;
}

export const browserHost: HostEnvironment = Object.freeze({
  presentation: "browser",
});
