import { selectSettingsSubtab } from "./settings-shell-play.js";
import { expect, userEvent, within } from "storybook/test";
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
    await selectSettingsSubtab(canvasElement, "Preferences");
    await userEvent.click(await c.findByRole("radio", { name: layout }));
    await userEvent.click(c.getByRole("radio", { name: /^Classic$/ }));
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
  };
  await settings("Focus");
  const focused = await body.findByRole("region", { name: "Agent response" });
  const tool = within(focused).getByRole("group", {
    name: "apply_patch tool call",
  });
  await userEvent.hover(within(tool).getByRole("group", { name: /tool row$/ }));
  await userEvent.click(
    within(tool).getByRole("button", {
      name: "Reply to apply_patch tool call",
    }),
  );
  await userEvent.type(
    within(tool).getByRole("textbox", { name: "Markdown editor" }),
    "Unsent tool discussion draft",
  );
  await expect(
    document.querySelectorAll('[data-ui-part="surface-layer-surface"]'),
  ).toHaveLength(1);
  await settings("Split");
  const split = await c.findByRole("complementary", { name: "Agent response" });
  const restored = within(split).getByRole("group", {
    name: "apply_patch tool call",
  });
  await expect(
    within(restored).getByRole("textbox", { name: "Markdown editor" }),
  ).toHaveTextContent("Unsent tool discussion draft");
  await expect(
    document.querySelectorAll('[data-ui-part="surface-layer-surface"]'),
  ).toHaveLength(0);
  await userEvent.click(
    within(restored).getByRole("button", { name: "Cancel" }),
  );
  await expect(within(restored).queryByRole("textbox")).toBeNull();
  await expect(
    within(restored).getByText(
      "Can we preserve the exact operation when this retry races?",
      { exact: true },
    ),
  ).toBeVisible();
};
