/** Cancel this physical-read waiter without closing a shared repository owner. */
export async function awaitPhysicalRead<T>(
  read: Promise<T>,
  signal: AbortSignal,
): Promise<T> {
  let cancel: (() => void) | undefined;
  try {
    const value = await Promise.race([
      new Promise<never>((_, reject) => {
        cancel = () =>
          reject(signal.reason ?? new DOMException("Aborted", "AbortError"));
        signal.addEventListener("abort", cancel, { once: true });
        if (signal.aborted) cancel();
      }),
      read,
    ]);
    signal.throwIfAborted();
    return value;
  } finally {
    if (cancel) signal.removeEventListener("abort", cancel);
  }
}
