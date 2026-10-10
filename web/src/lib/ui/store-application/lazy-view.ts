/** Share each view import; a failed download can be retried explicitly. */
export function lazyView<T>(
  load: () => Promise<T>,
  moduleURL?: string,
): () => Promise<T> {
  let pending: Promise<T> | undefined;
  let failures = 0;
  return () => {
    if (!pending) {
      const attempt =
        failures && moduleURL
          ? (import(
              /* @vite-ignore */ `${moduleURL}${moduleURL.includes("?") ? "&" : "?"}view-retry=${failures}`
            ) as Promise<T>)
          : load();
      pending = attempt.catch((error) => {
        pending = undefined;
        failures += 1;
        throw error;
      });
    }
    return pending;
  };
}

/** Replaced by the owning Vite plugin; never resolve entry URLs by guessing. */
export function viewModuleURL(_source: string): string {
  throw Error("View module URL was not prepared by the application build");
}
