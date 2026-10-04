import { expect, userEvent, within } from "storybook/test";

export async function playMobileSettings({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const canvas = within(canvasElement);
  await userEvent.click(
    canvas.getByRole("button", { name: "Open navigation sidebar" }),
  );
  await userEvent.click(
    canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
  );
  await userEvent.click(
    await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
      name: "Settings",
    }),
  );
  await expect(
    await canvas.findByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  const navigation = canvas.getByRole("navigation", {
    name: "Settings page navigation",
  });
  await expect(
    within(navigation).getByRole("heading", {
      name: "Local service",
      level: 1,
    }),
  ).toBeVisible();
  await expect(
    navigation.querySelector("h1 svg[aria-hidden='true']"),
  ).toBeVisible();
  await expect(navigation.firstElementChild).toBe(
    within(navigation).getByRole("heading", { level: 1 }),
  );
  await expect(
    within(navigation).queryByRole("button", { name: "Back to app" }),
  ).toBeNull();
  await expect(
    canvas.queryByText(
      "Connection and storage state for this jj-store installation.",
    ),
  ).not.toBeInTheDocument();
  await userEvent.click(
    canvas.getByRole("button", { name: "Open settings sidebar" }),
  );
  await userEvent.click(canvas.getByRole("button", { name: "Search" }));
  await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
  await expect(
    await canvas.findByRole("heading", { name: "Search", level: 1 }),
  ).toBeVisible();
  await expect(
    within(
      canvas.getByRole("navigation", { name: "Settings page navigation" }),
    ).getByRole("heading", { name: "Search", level: 1 }),
  ).toBeVisible();
  await expect(canvas.getByText("Fresh")).toBeVisible();
  await userEvent.click(
    canvas.getByRole("button", { name: "Open settings sidebar" }),
  );
  await userEvent.click(
    canvas.getByRole("button", { name: "Back to jj-store" }),
  );
  await expect(
    canvas.getByText("jj-store · Branches", { exact: true }),
  ).toBeVisible();
}
