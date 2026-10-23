<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceDialogFixture.svelte";
  import { themeNames } from "../../../web/src/lib/appearance/theme-catalog.js";

  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Modal",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-work/StoreWorkField.svelte",
      },
      layout: "fullscreen",
    },
  });

  async function verify(canvasElement: HTMLElement, alert: boolean) {
    const page = canvasElement.querySelector(".appearance-dialog-fixture")!;
    const c = within(document.body);
    const modal = await c.findByRole(alert ? "alertdialog" : "dialog");
    const m = within(modal);
    const heading = m.getByRole("heading", {
      name: alert ? "Confirm action" : "Create task",
    });
    const secondary = m.getByText(
      "Secondary text follows the selected app palette.",
    );
    for (const theme of themeNames) {
      await waitFor(() =>
        expect(document.documentElement.dataset.storeTheme).toBe(theme),
      );
      // Foundation modals animate all properties; check the settled palette.
      await waitFor(() => {
        expect(getComputedStyle(modal).backgroundColor, theme).toBe(
          getComputedStyle(page).backgroundColor,
        );
        expect(getComputedStyle(heading).color, theme).toBe(
          getComputedStyle(page).color,
        );
        expect(getComputedStyle(secondary).color, theme).toBe(
          getComputedStyle(page.querySelector("p")!).color,
        );
      });
      if (!alert) {
        const input = m.getByRole("textbox", { name: "Task title" });
        await expect(input).toHaveValue("Retained task title");
        await waitFor(() =>
          expect(getComputedStyle(input).color, theme).toBe(
            getComputedStyle(page).color,
          ),
        );
        const editor = m.getByRole("textbox", { name: "Markdown editor" });
        await expect(editor).toHaveTextContent("Retained task description");
        await expect(getComputedStyle(editor).color, theme).toBe(
          getComputedStyle(page).color,
        );
      }
      await userEvent.click(m.getByRole("button", { name: "Next palette" }));
    }
  }
</script>

<Story
  name="Task modal surface and text across every palette"
  play={async ({ canvasElement }) => verify(canvasElement, false)}
/>
<Story
  name="Confirmation modal surface and text across every palette"
  args={{ alert: true }}
  play={async ({ canvasElement }) => verify(canvasElement, true)}
/>
