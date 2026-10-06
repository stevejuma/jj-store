import { expect } from "@playwright/test";
export function validateAgentAcceptanceOrigin(origin, repository) {
  if (
    !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin ?? "") ||
    !/^[a-f0-9]{32}$/.test(repository ?? "")
  )
    throw Error("explicit disposable native origin and repository required");
}

export async function verifyRecordedHistory(context, origin, repository) {
  // The review page deliberately pins historical code. A cold legacy route chooses its own snapshot.
  const historyPage = await context.newPage();
  await historyPage.goto(`${origin}/#/repos/${repository}/agents`);
  await expect(
    historyPage.getByRole("tab", { name: "Agents", exact: true }),
  ).toHaveAttribute("aria-selected", "true");
  await expect(
    historyPage.getByRole("region", { name: "Agent activity", exact: true }),
  ).toBeVisible();
  await expect
    .poll(
      () =>
        historyPage
          .getByRole("list", { name: "Agent sessions", exact: true })
          .getByRole("listitem")
          .count(),
      { timeout: 30000 },
    )
    .toBeGreaterThan(0);
}
