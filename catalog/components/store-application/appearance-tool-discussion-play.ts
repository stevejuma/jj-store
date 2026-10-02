import { expect, userEvent, waitFor, within } from "storybook/test";
export const appearanceToolDiscussionPlay = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement),
    body = within(document.body);
  await userEvent.click(await c.findByRole("button", { name: /Codex ended/u }));
  const settings = async (layout: "Focus" | "Split") => {
    await userEvent.click(
      c.getByRole("button", { name: /Open profile menu for/ }),
    );
    await userEvent.click(body.getByRole("menuitem", { name: "Settings" }));
    await userEvent.click(await c.findByRole("button", { name: "Appearance" }));
    await userEvent.click(c.getByRole("radio", { name: layout }));
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
  };
  const openReply = async () => {
    const tool = c.getByRole("group", { name: "apply_patch tool call" });
    await userEvent.hover(tool);
    await waitFor(() =>
      expect(
        within(tool).getByRole("group", {
          name: "Actions for apply_patch tool call",
        }),
      ).toHaveAttribute("data-visible"),
    );
    await userEvent.click(
      within(tool).getByRole("button", {
        name: "Reply to apply_patch tool call",
      }),
    );
  };
  await settings("Focus");
  await openReply();
  const dialog = await body.findByRole("region", {
    name: "Reply to apply_patch",
  });
  await expect(dialog).toHaveAttribute("data-ui-part", "surface-layer-surface");
  const main = canvasElement.querySelector('[data-ui-part="main"]')!;
  const layer = dialog.closest('[data-ui-part="surface-layer"]')!;
  await waitFor(() => {
    expect(
      Math.abs(
        dialog.getBoundingClientRect().left -
          main.getBoundingClientRect().left -
          112,
      ),
    ).toBeLessThan(1);
    expect(dialog.getBoundingClientRect().right).toBeGreaterThanOrEqual(
      main.getBoundingClientRect().right,
    );
  });
  await expect(
    getComputedStyle(layer)
      .getPropertyValue("--ui-shell-surface-layer-inline-end-inset")
      .trim(),
  ).toBe("0px");
  await userEvent.type(
    within(dialog).getByRole("textbox"),
    "Unsent tool discussion draft",
  );
  await userEvent.click(
    within(dialog).getByRole("button", { name: "Close Reply to apply_patch" }),
  );
  await waitFor(() =>
    expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
  );
  await openReply();
  const restored = await body.findByRole("region", {
    name: "Reply to apply_patch",
  });
  await expect(within(restored).getByRole("textbox")).toHaveTextContent(
    "Unsent tool discussion draft",
  );
  await userEvent.click(
    within(restored).getByRole("button", {
      name: "Close Reply to apply_patch",
    }),
  );
  await waitFor(() =>
    expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
  );
  await settings("Split");
  await openReply();
  const split = await c.findByRole("complementary", {
    name: "Reply to apply_patch",
  });
  await expect(within(split).getByRole("textbox")).toHaveTextContent(
    "Unsent tool discussion draft",
  );
  await userEvent.click(
    within(split).getByRole("button", { name: "Close Reply to apply_patch" }),
  );
};
