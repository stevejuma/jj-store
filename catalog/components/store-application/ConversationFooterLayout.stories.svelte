<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/WorkspaceFilesFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Conversation footer layout",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      a11y: { test: "error" },
    },
  });
</script>

<Story
  name="Formatting actions retain space and expanded tools own a row"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const repository = await c.findByRole("button", { name: /Workspace:/ });
    const footer = repository.closest<HTMLElement>(".chat-message-footer")!;
    const toolbar = within(footer).getByRole("toolbar", {
      name: "Message formatting and actions",
    });
    const send = within(footer).getByRole("button", { name: "Send message" });
    const originalWidth = footer.style.width;
    try {
      for (const width of [548, 390, 280, 760]) {
        footer.style.width = `${width}px`;
        const toggle = within(toolbar).getByRole("button", {
          name: "Show formatting",
        });
        await waitFor(() => {
          const tools = toolbar.getBoundingClientRect();
          expect(toggle.getBoundingClientRect().right).toBeLessThanOrEqual(
            tools.right + 1,
          );
          const repo = repository.getBoundingClientRect();
          expect(repo.top >= tools.bottom || repo.left >= tools.right).toBe(
            true,
          );
          if (repo.top >= tools.bottom)
            expect(Math.abs(repo.left - tools.left)).toBeLessThanOrEqual(1);
          // Both fixture runtime labels are "Harness default". At 760px the
          // controls fit only if the repository yields some of its 18rem cap.
          if (width === 760) {
            const sendBox = send.getBoundingClientRect();
            expect(
              Math.abs(sendBox.top - tools.top),
              `Collapsed at ${width}px`,
            ).toBeLessThanOrEqual(1);
            expect(repo.top).toBeLessThan(tools.bottom);
            expect(repo.width).toBeLessThan(288);
          }
        });
        await userEvent.click(toggle);
        const hide = within(toolbar).getByRole("button", {
          name: "Hide formatting",
        });
        await waitFor(() => {
          const tools = toolbar.getBoundingClientRect();
          expect(repository.getBoundingClientRect().top).toBeGreaterThanOrEqual(
            tools.bottom,
          );
          expect(
            Math.abs(repository.getBoundingClientRect().left - tools.left),
          ).toBeLessThanOrEqual(1);
          expect(send.getBoundingClientRect().top).toBeGreaterThanOrEqual(
            tools.bottom,
          );
          const repo = repository.getBoundingClientRect();
          const sendBox = send.getBoundingClientRect();
          expect(
            Math.abs(
              repo.top + repo.height / 2 - (sendBox.top + sendBox.height / 2),
            ),
            `Expanded at ${width}px: ${Array.from(footer.children)
              .map(
                (element) =>
                  `${element.getBoundingClientRect().width}:${element.getBoundingClientRect().top}`,
              )
              .join(", ")}`,
          ).toBeLessThanOrEqual(1);
          expect(footer.scrollWidth).toBeLessThanOrEqual(
            footer.clientWidth + 1,
          );
        });
        await expect(send).toBeVisible();
        // Keyboard access reaches the final action even when the toolbar scrolls.
        hide.focus();
        for (let index = 0; index < 7; index++) await userEvent.tab();
        await expect(
          within(toolbar).getByRole("button", { name: "Quote" }),
        ).toHaveFocus();
        await userEvent.click(hide);
      }
    } finally {
      footer.style.width = originalWidth;
    }
  }}
/>
