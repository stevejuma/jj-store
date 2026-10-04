/** Optional file metadata never consumes every read worker or builds an unbounded queue. */
export function createFileReadQueue() {
  type Job = { key: string; run(): Promise<void>; cancel(): void };
  let pending: Job[] = [];
  const keys = new Set<string>();
  let active = false;
  let running: Job | undefined;
  let generation = 0;
  async function drain() {
    if (active) return;
    active = true;
    try {
      while (pending.length) {
        const job = pending.shift()!;
        running = job;
        const ticket = generation;
        try {
          await job.run();
        } catch {
          job.cancel();
        } finally {
          running = undefined;
          if (ticket === generation) keys.delete(job.key);
        }
      }
    } finally {
      active = false;
    }
  }
  return {
    add(key: string, run: () => Promise<void>, cancel: () => void) {
      if (keys.has(key)) return;
      if (pending.length >= 32) {
        cancel();
        return;
      }
      keys.add(key);
      pending.push({ key, run, cancel });
      void drain();
    },
    clear() {
      generation++;
      running?.cancel();
      for (const job of pending) job.cancel();
      pending = [];
      keys.clear();
    },
  };
}
