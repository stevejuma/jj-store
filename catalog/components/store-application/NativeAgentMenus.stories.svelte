<script module lang="ts">
  import { userEvent as pointer } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentLayoutFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Native agent actions",
    component: Fixture,
    tags: ["visual-pending"],
    args: { repository: true, focus: false, conversationView: "classic" },
    parameters: { layout: "fullscreen" },
  });
  async function openCopy() {
    const menu = within(document.body);
    await pointer.click(menu.getByRole("menuitem", { name: /^Copy$/ }));
    await menu.findByRole("menu", { name: /^Copy$/ });
  }
</script>

<Story
  name="Native identifier menus preserve selection and exact values"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      select = await c.findByRole("button", { name: /Open Codex ended/ });
    const card = select.closest(".store-collaboration__row")!;
    const duration = card.querySelector(
      ".store-collaboration__agent-duration",
    )!;
    await expect(
      card.getBoundingClientRect().right -
        duration.getBoundingClientRect().right,
    ).toBeLessThan(18);
    const write = fn(),
      descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await pointer.click(
        within(card as HTMLElement).getByRole("button", {
          name: "Agent session actions",
        }),
      );
      await openCopy();
      await pointer.click(
        within(document.body).getByRole("menuitem", {
          name: "Copy session ID",
        }),
      );
      await waitFor(() =>
        expect(write).toHaveBeenLastCalledWith("a1".repeat(16)),
      );
      await expect(select).toHaveAttribute("aria-pressed", "false");
      await pointer.click(select);
      await pointer.click(
        await c.findByRole("button", { name: "Agent details actions" }),
      );
      await openCopy();
      await pointer.click(
        within(document.body).getByRole("menuitem", {
          name: "Copy agent root",
        }),
      );
      await waitFor(() =>
        expect(write).toHaveBeenLastCalledWith("c3".repeat(32)),
      );
      const tool = c.getByRole("group", { name: "apply_patch tool call" });
      const toolRow = within(tool).getByRole("group", { name: /tool row$/ });
      await userEvent.hover(toolRow);
      await waitFor(() =>
        expect(
          within(tool).getByRole("group", {
            name: "Actions for apply_patch tool call",
          }),
        ).toHaveAttribute("data-visible"),
      );
      const actions = within(tool).getByRole("group", {
        name: "Actions for apply_patch tool call",
      });
      await waitFor(() =>
        expect(actions.getBoundingClientRect().top).toBeGreaterThanOrEqual(
          tool.getBoundingClientRect().top,
        ),
      );
      await pointer.click(
        within(tool).getByRole("button", {
          name: "More apply_patch tool actions",
        }),
      );
      await userEvent.unhover(toolRow);
      await expect(
        within(tool).getByRole("group", {
          name: "Actions for apply_patch tool call",
        }),
      ).toHaveAttribute("data-visible");
      await openCopy();
      await pointer.click(
        within(document.body).getByRole("menuitem", {
          name: "Copy tool call ID",
        }),
      );
      await waitFor(() =>
        expect(write).toHaveBeenLastCalledWith("d5".repeat(16)),
      );
      const message = c
        .getByText("Updated the parser and verified the focused contract.")
        .closest(".community-message")! as HTMLElement;
      await userEvent.hover(message);
      await waitFor(() =>
        expect(message).toHaveAttribute("data-actions-visible"),
      );
      await pointer.click(
        within(message).getByRole("button", { name: "More message actions" }),
      );
      await expect(
        within(document.body).queryByRole("menuitem", { name: "Edit comment" }),
      ).toBeNull();
      await openCopy();
      await pointer.click(
        within(document.body).getByRole("menuitem", { name: "Copy message" }),
      );
      await waitFor(() =>
        expect(write).toHaveBeenLastCalledWith(
          "Updated the parser and verified the focused contract.",
        ),
      );
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>
