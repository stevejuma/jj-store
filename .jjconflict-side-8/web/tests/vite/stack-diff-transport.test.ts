import { expect, test } from "vitest";
import { HttpStoreDataSource } from "../../src/lib/controller/http-store-data-source.js";
import { HttpLocalStoreDataSource } from "../../src/lib/controller/http-local-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
const repository = "1".repeat(32),
  operation = "2".repeat(128);
const json = (data: unknown) =>
  new Response(JSON.stringify(data), {
    headers: { "content-type": "application/json" },
  });
for (const profile of ["native", "local"] as const) {
  for (const kind of ["files", "patch"] as const) {
    test(`${profile} ${kind} cancellation releases only its observer and permits immediate retry`, async () => {
      let requests = 0;
      const signals: AbortSignal[] = [];
      const fetcher: typeof fetch = async (url, init) => {
        if (String(url).endsWith("snapshot"))
          return json({
            version: 1,
            profile: "git_sha1_jj045",
            repository,
            generation: 1,
            heads: [operation],
            metadata_commit: null,
            actor: null,
          });
        requests++;
        signals.push(init!.signal!);
        return new Promise<Response>((_resolve, reject) => {
          init!.signal!.addEventListener(
            "abort",
            () => reject(new DOMException("Aborted", "AbortError")),
            { once: true },
          );
        });
      };
      const source =
        profile === "native"
          ? new HttpStoreDataSource(
              repository,
              null,
              createStoreQueryClient(),
              "test",
              () => {},
              fetcher,
            )
          : new HttpLocalStoreDataSource(repository, fetcher);
      if (profile === "local") await source.snapshot();
      const from = "3".repeat(profile === "native" ? 128 : 40),
        to = "4".repeat(profile === "native" ? 128 : 40);
      const read = (signal: AbortSignal) =>
        kind === "files"
          ? source.diffPaths(operation, from, to, undefined, signal)
          : source.diff(operation, from, to, "src/main.ts", signal);
      const first = new AbortController(),
        shared = new AbortController();
      const a = read(first.signal),
        b = read(shared.signal);
      const rejectedA = expect(a).rejects.toMatchObject({ name: "AbortError" });
      const rejectedB = expect(b).rejects.toMatchObject({ name: "AbortError" });
      await new Promise((resolve) => setTimeout(resolve, 0));
      first.abort();
      await rejectedA;
      expect(requests).toBe(1);
      expect(signals[0]!.aborted).toBe(false);
      shared.abort();
      const retry = new AbortController();
      const c = read(retry.signal);
      const rejectedC = expect(c).rejects.toMatchObject({ name: "AbortError" });
      await rejectedB;
      await new Promise((resolve) => setTimeout(resolve, 0));
      expect(requests).toBe(2);
      expect(signals[0]!.aborted).toBe(true);
      expect(signals[1]!.aborted).toBe(false);
      retry.abort();
      await rejectedC;
      source.close();
    });
  }
}
