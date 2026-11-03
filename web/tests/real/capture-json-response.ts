import type { Page } from "@playwright/test";

export async function captureJsonResponse<T>(
  page: Page,
  url: string,
  action: () => Promise<void>,
) {
  let resolveCapture!: (value: { ok: boolean; outcome: T }) => void;
  let rejectCapture!: (reason: unknown) => void;
  const captured = new Promise<{ ok: boolean; outcome: T }>(
    (resolve, reject) => {
      resolveCapture = resolve;
      rejectCapture = reject;
    },
  );
  await page.route(
    url,
    async (route) => {
      try {
        const response = await route.fetch();
        const outcome = (await response.json()) as T;
        await route.fulfill({ response });
        resolveCapture({ ok: response.ok(), outcome });
      } catch (error) {
        rejectCapture(error);
        await route.abort("failed").catch(() => {});
      }
    },
    { times: 1 },
  );
  await action();
  return captured;
}
