/** Abort obsolete reads before advancing the controller selection. */
export function createReadScope() {
  let version = 0;
  let scope = new AbortController();
  return {
    version: () => version,
    signal: () => scope.signal,
    invalidate() {
      scope.abort();
      scope = new AbortController();
      return ++version;
    },
  };
}
