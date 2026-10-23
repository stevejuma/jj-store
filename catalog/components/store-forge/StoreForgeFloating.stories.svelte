<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Floating panels",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    args: { panelOpen: false },
    beforeEach: async () => {
      await views.stacks();
    },
  });
</script>

<Story
  name="Composer window preserves its editor through window controls"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/workspace",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Create PR for feature/workspace",
      }),
    );
    const form = await within(stack).findByRole("region", {
      name: "Create PR for feature/workspace",
    });
    const editor = within(form).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(editor);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(editor, "Retained floating draft");
    const float = within(form).getByRole("button", {
      name: "Float PR composer",
    });
    await userEvent.click(float);
    let window = await page.findByRole("dialog", {
      name: "Create PR · feature/workspace",
    });
    const background = getComputedStyle(window).backgroundColor;
    await expect(background).not.toBe("rgba(0, 0, 0, 0)");
    await expect(background).not.toBe("transparent");
    await userEvent.click(
      within(window).getByRole("button", { name: "Minimize floating pane" }),
    );
    await userEvent.click(
      await page.findByRole("button", {
        name: "Restore floating pane Create PR · feature/workspace",
      }),
    );
    window = await page.findByRole("dialog", {
      name: "Create PR · feature/workspace",
    });
    await expect(
      within(window).getByRole("textbox", { name: "Markdown editor" }),
    ).toBe(editor);
    await expect(editor).toHaveTextContent("Retained floating draft");
    await userEvent.click(
      within(window).getByRole("button", { name: "Publication status" }),
    );
    await page.findByRole("menuitemradio", { name: "Ready for review" });
    await userEvent.keyboard("{Escape}");
    await expect(window).toBeInTheDocument();
    await expect(
      within(window).getByRole("button", { name: "Publication status" }),
    ).toHaveFocus();
    if (import.meta.env.MODE === "test") {
      const { userEvent: pointer, page: browser } =
        await import("vitest/browser");
      const before = window.getBoundingClientRect();
      await pointer.dragAndDrop(
        window.querySelector<HTMLElement>("[data-floating-pane-header]")!,
        canvasElement,
        {
          sourcePosition: { x: 24, y: 16 },
          targetPosition: { x: before.left + 100, y: before.top + 80 },
        },
      );
      await waitFor(() =>
        expect(window.getBoundingClientRect().left).toBeGreaterThan(
          before.left + 20,
        ),
      );
      const moved = window.getBoundingClientRect();
      await pointer.dragAndDrop(
        within(window).getByRole("button", { name: "Resize floating pane se" }),
        canvasElement,
        {
          targetPosition: { x: moved.right - 70, y: moved.bottom - 60 },
        },
      );
      await waitFor(() =>
        expect(window.getBoundingClientRect().width).toBeLessThan(
          moved.width - 20,
        ),
      );
      try {
        await browser.viewport(640, 640);
        await waitFor(() => {
          const bounds = window.getBoundingClientRect();
          expect(bounds.left).toBeGreaterThanOrEqual(0);
          expect(bounds.top).toBeGreaterThanOrEqual(0);
          expect(bounds.right).toBeLessThanOrEqual(640);
          expect(bounds.bottom).toBeLessThanOrEqual(640);
        });
      } finally {
        await browser.viewport(1280, 800);
      }
    }
    await userEvent.click(
      within(window).getByRole("button", { name: "Publication status" }),
    );
    await userEvent.keyboard("{Escape}");
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Create PR · feature/workspace" }),
      ).not.toBeInTheDocument(),
    );
    await expect(float).toHaveFocus();
    await expect(
      within(form).getByRole("textbox", { name: "Markdown editor" }),
    ).toBe(editor);
    await expect(editor).toHaveTextContent("Retained floating draft");
  }}
/>
