import { expect, userEvent, within, waitFor } from "storybook/test";

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
  const overview = within(navigation).getByRole("tab", { name: "Overview" });
  await expect(overview.querySelector("svg")).toBeVisible();
  await expect(overview.closest("[data-presentation]")).toHaveAttribute(
    "data-presentation",
    "line",
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
  await userEvent.click(canvas.getByRole("button", { name: "Local service" }));
  await userEvent.click(canvas.getByRole("tab", { name: "Launch" }));
  await expect(
    await canvas.findByRole("heading", { name: "Local service", level: 1 }),
  ).toBeVisible();
  await expect(canvas.getByRole("tab", { name: "Launch" })).toHaveAttribute(
    "aria-selected",
    "true",
  );
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

/** Follow the public More menu when a retained conversation constrains tabs. */
export async function selectSettingsSubtab(
  canvasElement: HTMLElement,
  name: string,
) {
  const c = within(canvasElement);
  let tab: HTMLElement | null = null;
  let more: HTMLElement | null = null;
  await waitFor(() => {
    tab = c.queryByRole("tab", { name });
    more = c.queryByRole("button", { name: /^More .*settings sections$/ });
    expect(tab ?? more).not.toBeNull();
  });
  if (tab) await userEvent.click(tab);
  else {
    await userEvent.click(more!);
    await userEvent.click(
      await within(document.body).findByRole("menuitem", { name }),
    );
  }
}
