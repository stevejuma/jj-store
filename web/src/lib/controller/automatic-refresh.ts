// Serial, coalesced updates. A deferred or failed refresh retries automatically.
export function createAutomaticRefresh(options: {
  active(): boolean;
  refresh(): Promise<boolean>;
  report(cause: unknown): void;
  signal: AbortSignal;
}) {
  let pending = false,
    running = false,
    epoch = 0,
    requests = 0;
  const pauses = new Set<string>();
  let timer: ReturnType<typeof setTimeout> | undefined;
  let deadline = Infinity;
  let attempts = 0;
  const visible = () => typeof document === "undefined" || !document.hidden;
  function schedule(delay: number) {
    if (options.signal.aborted) return;
    const next = Date.now() + delay;
    if (timer !== undefined && deadline <= next) return;
    clearTimeout(timer);
    deadline = next;
    timer = setTimeout(() => {
      timer = undefined;
      deadline = Infinity;
      void tick();
    }, delay);
  }
  function reschedule() {
    if (running) return;
    if (!visible() || !options.active() || pauses.size || !pending) {
      clearTimeout(timer);
      timer = undefined;
      deadline = Infinity;
      attempts = 0;
      return;
    }
    if (timer !== undefined) return;
    const delay =
      [500, 1000, 2000, 5000][Math.min(attempts++, 3)]! *
      (0.9 + Math.random() * 0.2);
    schedule(delay);
  }
  async function tick() {
    if (running || options.signal.aborted) return;
    running = true;
    const ticket = epoch;
    try {
      if (visible() && options.active() && !pauses.size) {
        const before = requests;
        if (ticket === epoch && pending && !options.signal.aborted) {
          const accepted = await options.refresh();
          if (accepted) attempts = 0;
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
      reschedule();
    }
  }
  const request = () => {
    requests++;
    pending = true;
    if (!running) schedule(150);
  };
  const wake = () => {
    if (!visible()) {
      clearTimeout(timer);
      timer = undefined;
      deadline = Infinity;
    } else if (pending) schedule(150);
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
      timer = undefined;
      deadline = Infinity;
      if (typeof window !== "undefined") {
        window.removeEventListener("focus", wake);
        document.removeEventListener("visibilitychange", wake);
      }
    },
    { once: true },
  );
  reschedule();
  return {
    request,
    reschedule,
    reset() {
      epoch++;
      pending = false;
      pauses.clear();
      attempts = 0;
      reschedule();
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
