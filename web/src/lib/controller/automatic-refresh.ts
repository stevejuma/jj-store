// Serial, coalesced updates. A deferred or failed refresh retries automatically.
export function createAutomaticRefresh(options: {
  active(): boolean;
  refresh(): Promise<boolean>;
  poll(): Promise<boolean>;
  report(cause: unknown): void;
  signal: AbortSignal;
  interval?: number;
}) {
  let pending = false,
    running = false,
    epoch = 0,
    requests = 0;
  const pauses = new Set<string>();
  let timer: ReturnType<typeof setTimeout> | undefined;
  const visible = () => typeof document === "undefined" || !document.hidden;
  function schedule(delay: number) {
    if (options.signal.aborted) return;
    clearTimeout(timer);
    timer = setTimeout(() => void tick(), delay);
  }
  async function tick() {
    if (running || options.signal.aborted) return;
    running = true;
    const ticket = epoch;
    try {
      if (visible() && options.active() && !pauses.size) {
        if (!pending) {
          const changed = await options.poll();
          if (ticket === epoch) pending ||= changed;
        }
        const before = requests;
        if (ticket === epoch && pending && !options.signal.aborted) {
          const accepted = await options.refresh();
          if (ticket === epoch) pending = !accepted || before !== requests;
        }
      }
    } catch (cause) {
      if (ticket === epoch && !options.signal.aborted) {
        pending = true;
        options.report(cause);
      }
    } finally {
      running = false;
      schedule(options.interval ?? 5000);
    }
  }
  const request = () => {
    requests++;
    pending = true;
    if (!running) schedule(150);
  };
  const wake = () => {
    if (visible()) request();
  };
  if (typeof window !== "undefined") {
    window.addEventListener("focus", wake);
    document.addEventListener("visibilitychange", wake);
  }
  options.signal.addEventListener(
    "abort",
    () => {
      epoch++;
      clearTimeout(timer);
      if (typeof window !== "undefined") {
        window.removeEventListener("focus", wake);
        document.removeEventListener("visibilitychange", wake);
      }
    },
    { once: true },
  );
  schedule(options.interval ?? 5000);
  return {
    request,
    reset() {
      epoch++;
      pending = false;
      pauses.clear();
    },
    pause(key: string, paused: boolean) {
      if (paused) pauses.add(key);
      else {
        pauses.delete(key);
        if (pending) schedule(150);
      }
    },
  };
}
