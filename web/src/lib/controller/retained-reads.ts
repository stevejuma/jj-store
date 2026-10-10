/** Session-owned successful reads. Events invalidate; elapsed time never does. */
export class RetainedReads {
  private generations = new Map<string, number>();
  private accepted = new Set<string>();
  private pending = new Map<string, Promise<boolean | void>>();
  constructor(private read: (key: string) => Promise<boolean | void>) {}
  invalidate(key?: string) {
    for (const id of key === undefined ? this.generations.keys() : [key]) {
      this.generations.set(id, (this.generations.get(id) ?? 0) + 1);
      this.accepted.delete(id);
    }
  }
  ensure(key: string) {
    const pending = this.pending.get(key);
    if (pending) return pending;
    if (this.accepted.has(key)) return Promise.resolve(true);
    const generation = this.generations.get(key) ?? 0;
    this.generations.set(key, generation);
    const read = this.read(key)
      .then((accepted) => {
        if (accepted !== false && this.generations.get(key) === generation)
          this.accepted.add(key);
        return accepted;
      })
      .finally(() => {
        if (this.pending.get(key) === read) this.pending.delete(key);
      });
    this.pending.set(key, read);
    return read;
  }
  refresh(key: string) {
    this.invalidate(key);
    return this.ensure(key);
  }
  release(key: string) {
    this.invalidate(key);
    this.pending.delete(key);
  }
  clear() {
    this.invalidate();
    this.accepted.clear();
  }
}
