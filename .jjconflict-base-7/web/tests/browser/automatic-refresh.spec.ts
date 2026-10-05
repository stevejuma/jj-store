import { expect, test } from "@playwright/test";
import {
  api,
  operation,
  repo,
  sample,
  graph,
  referencesPage,
} from "./native-browser-fixture.js";
for (const historical of [false, true]) {
  test(`new repository data refreshes automatically with ${historical ? "historical" : "current-head"} selection`, async ({
    page,
  }) => {
    await api(page, { access: "local_trusted" });
    const retained = historical ? "aa".repeat(64) : operation,
      next = "bb".repeat(64);
    let changed = false,
      refreshedReferences = 0;
    const head = () => {
      const value = sample("snapshot");
      if (changed) {
        value.heads = [next];
        value.published_seq++;
        value.manifest_revision++;
      }
      return value;
    };
    await page.route("**/snapshot", (route) => route.fulfill({ json: head() }));
    await page.route("**/events", (route) => {
      const ready = sample("events");
      ready.snapshot = head();
      ready.cursor.position = ready.snapshot.published_seq;
      return route.fulfill({
        contentType: "text/event-stream",
        body: `id: v1:${repo}:${ready.cursor.epoch}:${ready.cursor.position}\nevent: ready\ndata: ${JSON.stringify(ready)}\n\n`,
      });
    });
    await page.route("**/references?**", (route) => {
      if (changed) refreshedReferences++;
      const value = referencesPage(null);
      value.snapshot.operation_id = new URL(
        route.request().url(),
      ).searchParams.get("at_op");
      return route.fulfill({ json: value });
    });
    await page.route("**/log?**", (route) => {
      const value = structuredClone(graph);
      value.snapshot.operation_id = new URL(
        route.request().url(),
      ).searchParams.get("at_op");
      return route.fulfill({ json: value });
    });
    const errors: string[] = [];
    page.on("pageerror", (e) => errors.push(e.message));
    await page.goto(`/#/repos/${repo}/branches?at_op=${retained}`);
    await expect(
      page.getByRole("option", { name: "main", exact: true }),
    ).toBeVisible();
    changed = true;
    if (!historical)
      await expect
        .poll(
          () => new URLSearchParams(page.url().split("?")[1]).get("at_op"),
          { timeout: 15000 },
        )
        .toBe(next);
    else {
      await expect
        .poll(() => refreshedReferences, { timeout: 15000 })
        .toBeGreaterThan(0);
      expect(new URLSearchParams(page.url().split("?")[1]).get("at_op")).toBe(
        retained,
      );
    }
    await expect(
      page.getByRole("option", { name: "main", exact: true }),
    ).toBeVisible();
    expect(errors).toEqual([]);
  });
}
