import { Channel, invoke } from "@tauri-apps/api/core";
import type { TerminalBridge, TerminalMessage } from "../terminal/bridge.js";
export function nativeTerminalBridge(): TerminalBridge {
  return {
    targets: (repository, profile) =>
      invoke("terminal_targets", { repository, profile }),
    list: () => invoke("terminal_list"),
    open: (requestId, repository, profile, targetId) =>
      invoke("terminal_open", { requestId, repository, profile, targetId }),
    attach: (sessionId, receive) =>
      invoke("terminal_attach", {
        sessionId,
        onFrame: new Channel<TerminalMessage>(receive),
      }),
    detach: (sessionId, subscriptionId) =>
      invoke("terminal_detach", { sessionId, subscriptionId }),
    input: (sessionId, data) => invoke("terminal_input", { sessionId, data }),
    resize: (sessionId, columns, rows) =>
      invoke("terminal_resize", { sessionId, columns, rows }),
    scroll: (sessionId, lines) =>
      invoke("terminal_scroll", { sessionId, lines }),
    focus: (sessionId, focused) =>
      invoke("terminal_focus", { sessionId, focused }),
    ack: (sessionId, subscriptionId, sequence) =>
      invoke("terminal_ack", { sessionId, subscriptionId, sequence }),
    ready: (sessionId, subscriptionId, viewport) =>
      invoke("terminal_viewport_ready", {
        sessionId,
        subscriptionId,
        viewport,
      }),
    close: (sessionId) => invoke("terminal_close", { sessionId }),
  };
}
