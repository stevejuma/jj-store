import { expect, within } from "storybook/test";

/** Select the primary revision fixture by its complete native identity. */
export async function revisionReviewOpener(canvasElement: HTMLElement) {
  const identifier = await within(canvasElement).findByRole("button", {
    name: "Open review #e1e1e1e1",
  });
  await expect(identifier).toHaveAccessibleDescription("e1".repeat(16));
  return within(identifier.closest("article")!).getByRole("button", {
    name: "Open Preserve the exact operation when this retry races open review with 3 comments",
  });
}
