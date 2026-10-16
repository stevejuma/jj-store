import { expect, userEvent, within } from "storybook/test";
export const verifyUnsupportedReviews = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await userEvent.click(await c.findByRole("tab", { name: "Reviews" }));
  const home = await c.findByRole("region", {
    name: "Reviews in UI repository",
  });
  const empty = within(home)
    .getByText("Reviews unavailable")
    .closest('[data-slot="empty"]');
  await expect(empty).toBeVisible();
  await expect(
    empty?.querySelector('[data-ui-part="empty-media"] svg'),
  ).toBeVisible();
  await expect(empty).toHaveTextContent(
    "Reviews are not supported for local JJ repositories.",
  );
  await expect(within(home).queryByRole("alert")).toBeNull();
  await expect(
    within(home).queryByRole("button", { name: "Retry reviews" }),
  ).toBeNull();
  await expect(
    within(
      c.getByRole("region", { name: "Reviews in Parser repository" }),
    ).getByRole("list", { name: "Reviews" }).firstChild,
  ).not.toBeNull();
};
