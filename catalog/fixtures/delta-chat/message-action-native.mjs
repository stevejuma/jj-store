import { expect } from "@playwright/test";
export async function messageActionAcceptance(page, base) {
  await page.goto(
    `${base}/iframe.html?id=jj-store-commentable-agent-chat-body--interaction-playground&viewMode=story&embed=true`,
  );
  const source = page.getByRole("textbox", {
    name: "Assistant explanation",
    exact: true,
  });
  await expect(source).toBeVisible({ timeout: 60000 });
  await source.hover();
  const actions = page.getByRole("group", {
    name: "Actions for Assistant explanation",
  });
  await expect(actions).toHaveCSS("opacity", "1");
  await expect(actions).toHaveCSS("border-top-width", "1px");
  const background = await actions.evaluate(
    (node) => getComputedStyle(node).backgroundColor,
  );
  expect(background).not.toBe("rgba(0, 0, 0, 0)");
  const comment = page.getByRole("button", {
    name: "Comment on Assistant explanation",
  });
  await comment.hover();
  expect(
    await comment.evaluate((node) => getComputedStyle(node).backgroundColor),
  ).not.toBe(background);
  await page
    .getByRole("button", { name: "Add reaction to Assistant explanation" })
    .click();
  await page.getByRole("button", { name: "React 👍" }).click();
  await expect(
    page.getByRole("button", { name: "1 reaction with 👍" }),
  ).toHaveAttribute("aria-pressed", "true");
  await expect(
    page.getByRole("form", { name: "Inline comment composer" }),
  ).toHaveCount(0);
  const outer = page.getByRole("button", {
    name: "Agent response · Synthetic review",
  });
  await outer.hover();
  await expect(outer).toHaveCSS("background-color", "rgba(0, 0, 0, 0)");
  await outer.click();
  await page.getByRole("button", { name: /<subagent-completion/ }).click();
  await expect(
    page.getByRole("group", {
      name: "Returned synthetic response",
      exact: true,
    }),
  ).toContainText("No regressions found.");
}
