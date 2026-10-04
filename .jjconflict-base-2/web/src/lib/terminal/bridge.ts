import { Channel, invoke } from "@tauri-apps/api/core";
import type { TerminalFrame, TerminalViewport } from "./renderer.js";
export interface TerminalTarget {
  id: string;
  repository: string;
  path: string;
  label: string;
}
export interface TerminalSession {
  id: string;
  target: TerminalTarget;
  title: string;
  exited: boolean;
}
export type FrameMessage = TerminalFrame & {
  subscriptionId: string;
  sequence: number;
  bracketedPaste: boolean;
  focusReporting: boolean;
};
export type TerminalMessage =
  | { type: "frame"; payload: FrameMessage }
  | { type: "title"; payload: string }
  | { type: "exit" | "resetTitle" | "bell" };
export type Attachment = {
  sessionId: string;
  subscriptionId: string;
  viewport: TerminalViewport;
};
export interface TerminalBridge {
  targets(repository: string, profile: string): Promise<TerminalTarget[]>;
  list(): Promise<TerminalSession[]>;
  open(
    requestId: string,
    repository: string,
    profile: string,
    targetId: string,
  ): Promise<TerminalSession>;
  attach(
    sessionId: string,
    receive: (message: TerminalMessage) => void,
  ): Promise<Attachment>;
  detach(sessionId: string, subscriptionId: string): Promise<void>;
  input(sessionId: string, data: string): Promise<void>;
  resize(
    sessionId: string,
    columns: number,
    rows: number,
  ): Promise<TerminalViewport>;
  scroll(sessionId: string, lines: number): Promise<void>;
  focus(sessionId: string, focused: boolean): Promise<void>;
  ack(
    sessionId: string,
    subscriptionId: string,
    sequence: number,
  ): Promise<void>;
  ready(
    sessionId: string,
    subscriptionId: string,
    viewport: TerminalViewport,
  ): Promise<void>;
  close(sessionId: string): Promise<void>;
}
export function nativeTerminalBridge(): TerminalBridge | undefined {
  if (
    typeof window === "undefined" ||
    window.__JJ_STORE_DESKTOP__?.terminal !== 1
  )
    return;
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
