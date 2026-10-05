import type { NotificationToastRecord } from "@lapismd/design-core/workspace/plugins/notifications";

export type NotificationLifetime = "condition" | "dismiss" | "timed";
export interface StoreNotification extends NotificationToastRecord {
  scope: string;
  key: string;
  lifetime: NotificationLifetime;
}
export interface NotificationInput {
  scope: string;
  key: string;
  title: string;
  message: string;
  severity?: StoreNotification["severity"];
  lifetime?: NotificationLifetime;
  action?: NonNullable<NotificationToastRecord["action"]> & { run(): void };
}
export type NotificationEvent = {
  type: "published" | "resolved" | "dismissed";
  notification: Readonly<StoreNotification>;
};
type Timer = {
  handle?: ReturnType<typeof setTimeout>;
  remaining: number;
  started: number;
};

/** Session-only application policy. Transport adapters can subscribe without DOM access. */
export class NotificationService {
  records = $state<readonly StoreNotification[]>([]);
  private episodes = new Map<
    string,
    { fingerprint: string; dismissed: boolean }
  >();
  private actions = new Map<string, { id: string; run(): void }>();
  private timers = new Map<string, Timer>();
  private listeners = new Set<(event: NotificationEvent) => void>();
  private disposed = false;
  private id(scope: string, key: string) {
    return JSON.stringify([scope, key]);
  }

  publish(input: NotificationInput) {
    if (this.disposed) return;
    const id = this.id(input.scope, input.key);
    if (!input.message) {
      this.resolve(input.scope, input.key);
      return;
    }
    const { action, ...data } = input;
    const record: StoreNotification = {
      ...data,
      id,
      severity: input.severity ?? "error",
      lifetime: input.lifetime ?? "condition",
      ...(action
        ? {
            action: {
              id: action.id,
              label: action.label,
              ...(action.variant ? { variant: action.variant } : {}),
            },
          }
        : {}),
    };
    const fingerprint = JSON.stringify(record);
    const episode = this.episodes.get(id);
    if (episode?.fingerprint === fingerprint && episode.dismissed) {
      this.actions.delete(id);
      return;
    }
    if (action) this.actions.set(id, { id: action.id, run: action.run });
    else this.actions.delete(id);
    if (
      episode?.fingerprint === fingerprint &&
      this.records.some((item) => item.id === id)
    )
      return;
    this.episodes.set(id, { fingerprint, dismissed: false });
    this.stopTimer(id);
    this.records = [...this.records.filter((item) => item.id !== id), record];
    if (record.lifetime === "timed") {
      this.timers.set(id, { remaining: 4000, started: Date.now() });
      this.resume(id);
    }
    this.emit("published", record);
  }
  resolve(scope: string, key: string) {
    const id = this.id(scope, key);
    this.remove(id, "resolved");
    this.episodes.delete(id);
  }
  /** Unmount hides an obsolete condition but retains its dismissal episode. */
  withdraw(scope: string, key: string) {
    this.remove(this.id(scope, key), "resolved");
  }
  dismiss = (id: string) => {
    const episode = this.episodes.get(id);
    if (episode) episode.dismissed = true;
    this.remove(id, "dismissed");
  };
  invoke = (id: string, actionId: string) => {
    const action = this.actions.get(id);
    if (action?.id === actionId && this.records.some((item) => item.id === id))
      action.run();
  };
  pause = (id: string) => {
    const timer = this.timers.get(id);
    if (!timer?.handle) return;
    clearTimeout(timer.handle);
    timer.handle = undefined;
    timer.remaining = Math.max(
      0,
      timer.remaining - (Date.now() - timer.started),
    );
  };
  resume = (id: string) => {
    const timer = this.timers.get(id);
    if (!timer || timer.handle) return;
    timer.started = Date.now();
    timer.handle = setTimeout(() => this.dismiss(id), timer.remaining);
  };
  /** Page changes retire obsolete conditions, while acknowledgement outcomes survive. */
  leaveScope(scope: string) {
    for (const record of this.records)
      if (record.scope === scope && record.lifetime === "condition")
        this.remove(record.id, "resolved");
  }
  reset() {
    for (const record of this.records) this.remove(record.id, "resolved");
    this.episodes.clear();
    this.actions.clear();
  }
  subscribe(listener: (event: NotificationEvent) => void) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }
  dispose() {
    this.reset();
    this.listeners.clear();
    this.disposed = true;
  }
  private stopTimer(id: string) {
    clearTimeout(this.timers.get(id)?.handle);
    this.timers.delete(id);
  }
  private remove(id: string, type: "resolved" | "dismissed") {
    this.stopTimer(id);
    this.actions.delete(id);
    const record = this.records.find((item) => item.id === id);
    if (!record) return;
    this.records = this.records.filter((item) => item.id !== id);
    this.emit(type, record);
  }
  private emit(type: NotificationEvent["type"], record: StoreNotification) {
    const notification = Object.freeze({
      ...record,
      ...(record.action ? { action: Object.freeze({ ...record.action }) } : {}),
    });
    for (const listener of this.listeners) listener({ type, notification });
  }
}
