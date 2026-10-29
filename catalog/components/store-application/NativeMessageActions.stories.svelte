<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within, fn } from "storybook/test";
  import Fixture from "../../fixtures/NativeReviewConversationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Native message actions",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-collaboration/StoreReviewConversation.svelte",
      },
      layout: "fullscreen",
    },
  });
  const root = (canvas: HTMLElement) =>
    canvas.querySelector<HTMLElement>(
      '[data-message-id="' + "11".repeat(16) + '"]',
    )!;
  async function openMenu(message: HTMLElement, copyValues = true) {
    await userEvent.hover(message);
    await waitFor(() =>
      expect(message).toHaveAttribute("data-actions-visible"),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: "More message actions" }),
    );
    const menu = within(document.body);
    await expect(
      menu.queryByRole("menuitem", { name: "Copy comment ID" }),
    ).toBeNull();
    const copy = menu.getByRole("menuitem", { name: /^Copy$/ });
    await expect(copy).toHaveAttribute("aria-haspopup", "menu");
    if (copyValues) {
      await waitFor(() =>
        expect(
          copy.closest('[role="menu"]')!.contains(document.activeElement),
        ).toBe(true),
      );
      copy.focus();
      await userEvent.keyboard("{ArrowRight}");
      await menu.findByRole("menuitem", { name: "Copy comment ID" });
    }
    return menu;
  }
</script>

<Story
  name="Native copy reply and own author edit"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      message = root(canvasElement);
    await expect(
      message.querySelector(".store-collaboration__message-identities"),
    ).toBeNull();
    const write = fn();
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      let menu = await openMenu(message);
      const icons = [
        "Copy message",
        "Copy comment ID",
        "Copy revision ID",
        "Copy author ID",
        "Copy thread ID",
      ].map((name) => {
        const icon = menu.getByRole("menuitem", { name }).querySelector("svg")!;
        expect(icon).toHaveAttribute("aria-hidden", "true");
        return icon.getAttribute("class");
      });
      await expect(new Set(icons).size).toBe(5);
      await userEvent.unhover(message);
      await expect(
        message.querySelector('[data-ui-part="message-actions"]'),
      ).toHaveAttribute("data-menu-open");
      await userEvent.click(
        menu.getByRole("menuitem", { name: "Copy comment ID" }),
      );
      await expect(write).toHaveBeenCalledWith("11".repeat(16));
      await expect(within(message).getByRole("status").textContent).toContain(
        "comment ID copied",
      );
      await userEvent.hover(message);
      // Message actions intentionally appear after the shared hover delay.
      await waitFor(() =>
        expect(message).toHaveAttribute("data-actions-visible"),
      );
      await userEvent.click(
        within(message).getByRole("button", { name: "Reply" }),
      );
      await expect(
        c.getByRole("group", { name: /Replying/ }),
      ).toHaveTextContent("Can we preserve the exact operation");
      await userEvent.type(c.getByRole("textbox"), "Nested native reply");
      await userEvent.click(
        within(
          c.getByRole("group", { name: "Reply to review thread" }),
        ).getByRole("button", { name: "Reply" }),
      );
      await expect(
        c.getByText("Nested native reply", { exact: true }),
      ).toBeVisible();
      const reply = c
        .getByText("Nested native reply", { exact: true })
        .closest("[data-thread-depth]")!;
      await expect(reply).toHaveAttribute("data-thread-depth", "1");
      menu = await openMenu(message, false);
      await userEvent.click(
        menu.getByRole("menuitem", { name: "Edit comment" }),
      );
      await expect(
        c.getByRole("group", { name: "Editing message" }),
      ).toHaveTextContent("Can we preserve the exact operation");
      const editor = c.getByRole("textbox");
      await userEvent.click(editor);
      await userEvent.clear(editor);
      await userEvent.type(editor, "Edited native comment");
      await userEvent.click(c.getByRole("button", { name: "Save comment" }));
      await expect(
        c.getByText("Edited native comment", { exact: true }),
      ).toBeVisible();
      await expect(
        c.queryByRole("group", { name: "Editing message" }),
      ).toBeNull();
      await waitFor(async () => {
        await expect(editor.querySelector(".cm-placeholder")).toBeVisible();
      });
      await expect(editor).not.toHaveTextContent("Edited native comment");
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>
<Story
  name="Read only and other author comments expose copy without edit"
  args={{ own: false, writable: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      message = root(canvasElement),
      menu = await openMenu(message);
    await expect(
      menu.queryByRole("menuitem", { name: "Edit comment" }),
    ).toBeNull();
    await expect(
      within(message).queryByRole("button", { name: "Reply" }),
    ).toBeNull();
    await waitFor(async () => {
      await expect(
        menu.getByRole("menuitem", { name: "Copy revision ID" }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await expect(c.queryByRole("textbox")).toBeNull();
  }}
/>
<Story
  name="Unknown edit retains original context and draft"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      menu = await openMenu(root(canvasElement), false);
    await userEvent.click(menu.getByRole("menuitem", { name: "Edit comment" }));
    const editor = c.getByRole("textbox");
    await userEvent.click(editor);
    await userEvent.clear(editor);
    await userEvent.type(editor, "Retained edit draft");
    await userEvent.click(c.getByRole("button", { name: "Save comment" }));
    await expect(editor).toHaveTextContent("Retained edit draft");
    await expect(
      c.getByRole("group", { name: "Editing message" }),
    ).toHaveTextContent("Can we preserve");
    await expect(
      c.getByRole("button", { name: "Save comment" }),
    ).toBeDisabled();
    await expect(c.getByRole("alert")).toHaveTextContent("outcome is unknown");
    await userEvent.click(
      c.getByRole("button", { name: "Check review command" }),
    );
    await waitFor(() =>
      expect(
        within(c.getByRole("list", { name: "Review comments" })).getByText(
          "Retained edit draft",
          { exact: true },
        ),
      ).toBeVisible(),
    );
    await expect(
      c.queryByRole("group", { name: "Editing message" }),
    ).toBeNull();
  }}
/>
