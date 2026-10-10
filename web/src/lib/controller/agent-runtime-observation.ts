/** App-owned state notifications; deadlines bound waits and never trigger reads. */
export class AgentRuntimeObservation {
  private listeners = new Set<() => void>();
  private cancellations = new Set<() => void>();
  notify() {
    for (const listener of this.listeners) listener();
  }
  wait(read: () => boolean | undefined, timeoutMs: number) {
    return new Promise<boolean>((resolve) => {
      const finish = (ready: boolean) => {
        clearTimeout(deadline);
        this.listeners.delete(check);
        this.cancellations.delete(cancel);
        resolve(ready);
      };
      const check = () => {
        const ready = read();
        if (ready !== undefined) finish(ready);
      };
      const deadline = setTimeout(() => finish(false), timeoutMs);
      const cancel = () => finish(false);
      this.listeners.add(check);
      this.cancellations.add(cancel);
      check();
    });
  }
  destroy() {
    for (const cancel of this.cancellations) cancel();
  }
}
