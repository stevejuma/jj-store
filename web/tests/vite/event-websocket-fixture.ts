import { vi } from "vitest";
export class EventSocket {
  static instances: EventSocket[] = [];
  onopen: (() => void) | null = null;
  onmessage: ((message: { data: unknown }) => void) | null = null;
  onerror: (() => void) | null = null;
  onclose: ((event: { code: number }) => void) | null = null;
  send = vi.fn();
  close = vi.fn();
  url: string;
  constructor(
    url: URL,
    readonly protocol: string,
  ) {
    this.url = url.href;
    EventSocket.instances.push(this);
  }
  receive(data: unknown) {
    this.onmessage?.({ data: JSON.stringify(data) });
  }
}
export function installSockets() {
  EventSocket.instances = [];
  vi.stubGlobal("WebSocket", EventSocket);
  vi.stubGlobal("location", { href: "http://127.0.0.1:7460/" });
}
