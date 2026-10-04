import type {
  Attachment,
  TerminalBridge,
  TerminalMessage,
  TerminalSession,
  TerminalTarget,
} from "../../web/src/lib/terminal/bridge.js";
import type { TerminalViewport } from "../../web/src/lib/terminal/renderer.js";
export class FakeTerminalBridge implements TerminalBridge {
  sessions: TerminalSession[] = [];
  receivers = new Map<string, (message: TerminalMessage) => void>();
  attachments = new Map<string, Attachment>();
  inputs: string[] = [];
  closeCalls: string[] = [];
  acks: number[] = [];
  failOpen = false;
  opens: string[] = [];
  detached: string[] = [];
  constructor(readonly folders = 1) {}
  async targets(repository: string): Promise<TerminalTarget[]> {
    return Array.from({ length: this.folders }, (_, i) => ({
      id: `${repository}:${i}`,
      repository,
      label: `Folder ${i + 1}`,
      path: `/workspace/${repository}/${i + 1}`,
    }));
  }
  async list() {
    return [...this.sessions];
  }
  async open(
    request: string,
    repository: string,
    _profile: string,
    targetId: string,
  ) {
    this.opens.push(request);
    let session = this.sessions.find((s) => s.id === request);
    if (!session) {
      const target = (await this.targets(repository)).find(
        (t) => t.id === targetId,
      )!;
      session = { id: request, target, title: "Shell", exited: false };
      this.sessions.push(session);
    }
    if (this.failOpen) {
      this.failOpen = false;
      throw Error("Open acknowledgement lost");
    }
    return session;
  }
  emit(id: string, text = "Ready") {
    const attachment = this.attachments.get(id)!;
    this.receivers.get(id)?.({
      type: "frame",
      payload: {
        subscriptionId: attachment.subscriptionId,
        sequence: this.acks.length + 1,
        bracketedPaste: true,
        focusReporting: false,
        viewport: attachment.viewport,
        full: true,
        cursor: { line: 0, column: text.length, visible: true },
        rows: [
          {
            line: 0,
            wrapped: false,
            spans: [
              {
                style: { fg: 0x01000100, bg: 0x01000101, flags: 0 },
                clusters: [...text].map((text, column) => ({
                  text,
                  column,
                  width: 1,
                })),
              },
            ],
          },
        ],
      },
    });
  }
  async attach(id: string, receive: (message: TerminalMessage) => void) {
    const attachment = {
      sessionId: id,
      subscriptionId: crypto.randomUUID(),
      viewport: { generation: 0, columns: 80, screenLines: 24 },
    };
    this.receivers.set(id, receive);
    this.attachments.set(id, attachment);
    this.emit(id);
    return attachment;
  }
  async detach(id: string, subscriptionId: string) {
    this.detached.push(id);
    if (this.attachments.get(id)?.subscriptionId === subscriptionId)
      this.receivers.delete(id);
  }
  async input(id: string, data: string) {
    this.inputs.push(data);
    this.emit(id, `Input: ${data.replaceAll("\r", "")}`);
  }
  async resize(id: string, columns: number, rows: number) {
    const item = this.attachments.get(id)!;
    return (item.viewport = {
      generation: item.viewport.generation + 1,
      columns,
      screenLines: rows,
    });
  }
  async ready(id: string, _subscription: string, _viewport: TerminalViewport) {
    this.emit(id);
  }
  async scroll() {}
  async focus() {}
  async ack(_id: string, _subscription: string, sequence: number) {
    this.acks.push(sequence);
  }
  async close(id: string) {
    this.closeCalls.push(id);
    this.sessions = this.sessions.filter((s) => s.id !== id);
    this.receivers.delete(id);
  }
}
