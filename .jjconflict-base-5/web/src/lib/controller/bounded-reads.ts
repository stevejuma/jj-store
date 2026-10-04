/** Bound asynchronous reads, including queued navigation work. */
export function boundedReads(limit: number) {
  let active = 0;
  const waiting: (() => void)[] = [];
  function drain() {
    while (active < limit && waiting.length) waiting.shift()!();
  }
  return <T>(read: () => Promise<T>): Promise<T> =>
    new Promise((resolve, reject) => {
      waiting.push(() => {
        active++;
        let result: Promise<T>;
        try {
          result = read();
        } catch (cause) {
          result = Promise.reject(cause);
        }
        result.then(resolve, reject).finally(() => {
          active--;
          drain();
        });
      });
      drain();
    });
}
