import type {
  Attachment,
  TerminalBridge,
  TerminalMessage,
  TerminalSession,
  TerminalTarget,
} from "./bridge.js";
import { TerminalGrid } from "./renderer.js";
export interface TerminalView extends TerminalSession {
  grid?: TerminalGrid;
  revision: number;
  paste: boolean;
  closing: boolean;
}
export class TerminalController {
  sessions = $state.raw<TerminalView[]>([]);
  targets = $state.raw<TerminalTarget[]>([]);
  repository = $state("");
  profile = "native";
  targetId = $state("");
  visible = $state(false);
  maximized = $state(false);
  loading = $state(false);
  busy = $state(false);
  activeId = $state("");
  private active = new Map<string, string>();
  private attachments = new Map<string, Attachment>();
  private attaching = new Set<string>();
  private sizes = new Map<string, string>();
  private resizeQueue = Promise.resolve();
  private generation = 0;
  private disposed = false;
  private pending?: {
    request: string;
    repository: string;
    profile: string;
    target: string;
  };
  constructor(
    readonly bridge: TerminalBridge,
    readonly failed: (error: unknown) => void,
  ) {}
  private update(id: string, update: Partial<TerminalView>) {
    this.sessions = this.sessions.map((s) =>
      s.id === id ? { ...s, ...update } : s,
    );
  }
  private live(id: string) {
    return (
      !this.disposed && this.sessions.some((s) => s.id === id && !s.closing)
    );
  }
  private sessionFailed(id: string, error: unknown) {
    if (this.live(id)) this.failed(error);
  }
  async initialize() {
    try {
      const sessions = await this.bridge.list();
      if (this.disposed) return;
      this.sessions = [
        ...this.sessions,
        ...sessions
          .filter(
            (s) => !this.sessions.some((existing) => existing.id === s.id),
          )
          .map((s) => ({ ...s, revision: 0, paste: false, closing: false })),
      ];
      await Promise.all(this.sessions.map((s) => this.attach(s.id)));
      if (!this.activeId)
        this.select(
          this.sessions.find((s) => s.target.repository === this.repository)
            ?.id ?? "",
        );
    } catch (error) {
      if (!this.disposed) this.failed(error);
    }
  }
  async context(repository: string, profile: string) {
    this.repository = repository;
    this.profile = profile;
    this.targets = [];
    this.targetId = "";
    this.activeId =
      this.active.get(repository) ??
      this.sessions.find((s) => s.target.repository === repository)?.id ??
      "";
    const generation = ++this.generation;
    if (!repository) {
      this.loading = false;
      return;
    }
    this.loading = true;
    try {
      const targets = await this.bridge.targets(repository, profile);
      if (this.disposed || generation !== this.generation) return;
      this.targets = targets;
      this.targetId = targets.length === 1 ? targets[0].id : "";
    } catch (error) {
      if (!this.disposed && generation === this.generation) this.failed(error);
    } finally {
      if (generation === this.generation) this.loading = false;
    }
  }
  select(id: string) {
    this.activeId = id;
    this.active.set(this.repository, id);
  }
  async show() {
    this.visible = true;
    if (this.loading) return;
    if (!this.activeId && this.targets.length === 1) await this.create();
  }
  async create() {
    if (this.busy || !this.targetId) return;
    this.busy = true;
    const intent =
      this.pending?.repository === this.repository &&
      this.pending.target === this.targetId
        ? this.pending
        : {
            request: crypto.randomUUID(),
            repository: this.repository,
            profile: this.profile,
            target: this.targetId,
          };
    this.pending = intent;
    try {
      const session = await this.bridge.open(
        intent.request,
        intent.repository,
        intent.profile,
        intent.target,
      );
      if (this.disposed) return;
      this.pending = undefined;
      if (!this.sessions.some((s) => s.id === session.id))
        this.sessions = [
          ...this.sessions,
          { ...session, revision: 0, paste: false, closing: false },
        ];
      this.active.set(intent.repository, session.id);
      if (this.repository === intent.repository) this.select(session.id);
      await this.attach(session.id);
    } catch (error) {
      if (!this.disposed) this.failed(error);
    } finally {
      this.busy = false;
    }
  }
  async attach(id: string) {
    if (this.attaching.has(id) || this.attachments.has(id)) return;
    this.attaching.add(id);
    let attachment: Attachment | undefined;
    const pending: TerminalMessage[] = [];
    const receive = (message: TerminalMessage) => {
      if (!this.live(id)) return;
      if (!attachment) {
        if (pending.length < 8) pending.push(message);
        return;
      }
      const session = this.sessions.find((s) => s.id === id);
      if (!session) return;
      if (message.type === "frame") {
        const frame = message.payload;
        if (frame.subscriptionId !== attachment.subscriptionId) return;
        const grid = session.grid ?? new TerminalGrid(frame.viewport);
        if (
          grid.viewport.generation !== frame.viewport.generation ||
          grid.viewport.columns !== frame.viewport.columns ||
          grid.viewport.screenLines !== frame.viewport.screenLines
        )
          grid.resize(frame.viewport);
        grid.apply(frame);
        this.update(id, {
          grid,
          revision: session.revision + 1,
          paste: frame.bracketedPaste,
        });
        void this.bridge
          .ack(id, attachment.subscriptionId, frame.sequence)
          .catch((error) => this.sessionFailed(id, error));
      } else if (message.type === "title")
        this.update(id, { title: message.payload || "Shell" });
      else if (message.type === "resetTitle")
        this.update(id, { title: "Shell" });
      else if (message.type === "exit") this.update(id, { exited: true });
    };
    try {
      attachment = await this.bridge.attach(id, receive);
      if (this.disposed || !this.sessions.some((s) => s.id === id)) {
        await this.bridge.detach(id, attachment.subscriptionId);
        return;
      }
      this.attachments.set(id, attachment);
      for (const message of pending) receive(message);
    } finally {
      this.attaching.delete(id);
    }
  }
  async close(id: string) {
    this.update(id, { closing: true });
    try {
      await this.bridge.close(id);
      this.attachments.delete(id);
      this.sizes.delete(id);
      this.sessions = this.sessions.filter((s) => s.id !== id);
      if (this.activeId === id)
        this.select(
          this.sessions.find((s) => s.target.repository === this.repository)
            ?.id ?? "",
        );
    } catch (error) {
      this.update(id, { closing: false });
      this.failed(error);
    }
  }
  input(id: string, data: string) {
    if (this.live(id))
      void this.bridge
        .input(id, data)
        .catch((error) => this.sessionFailed(id, error));
  }
  scroll(id: string, lines: number) {
    if (this.live(id))
      void this.bridge
        .scroll(id, lines)
        .catch((error) => this.sessionFailed(id, error));
  }
  focus(id: string, value: boolean) {
    if (this.live(id))
      void this.bridge
        .focus(id, value)
        .catch((error) => this.sessionFailed(id, error));
  }
  resize(id: string, columns: number, rows: number) {
    if (!this.live(id)) return;
    const key = `${columns}:${rows}`;
    if (this.sizes.get(id) === key) return;
    this.sizes.set(id, key);
    this.resizeQueue = this.resizeQueue
      .then(async () => {
        if (!this.live(id) || this.sizes.get(id) !== key) return;
        const attachment = this.attachments.get(id);
        if (!attachment) {
          this.sizes.delete(id);
          return;
        }
        const viewport = await this.bridge.resize(id, columns, rows);
        if (!this.live(id)) return;
        await this.bridge.ready(id, attachment.subscriptionId, viewport);
      })
      .catch((error) => {
        this.sizes.delete(id);
        this.sessionFailed(id, error);
      });
  }
  dispose() {
    this.disposed = true;
    for (const [id, attachment] of this.attachments)
      void this.bridge.detach(id, attachment.subscriptionId).catch(() => {});
    this.attachments.clear();
  }
}
