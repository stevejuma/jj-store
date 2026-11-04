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
