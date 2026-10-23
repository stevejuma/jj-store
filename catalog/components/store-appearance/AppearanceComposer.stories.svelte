<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceComposerFixture.svelte";
  import { themeNames } from "../../../web/src/lib/appearance/theme-catalog.js";

  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Composer",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte",
      },
      layout: "fullscreen",
    },
  });
</script>

<Story
  name="Single composer surface across every palette"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const editor = await c.findByRole("textbox", { name: "Markdown editor" });
    const send = c.getByRole("button", { name: "Send message" });
    await expect(
      Number.parseFloat(getComputedStyle(send).borderRadius),
    ).toBeGreaterThanOrEqual(
      Number.parseFloat(getComputedStyle(send).width) / 2,
    );
    await userEvent.click(editor);
    await userEvent.type(editor, "Retained composer draft");
    const composer = canvasElement.querySelector(".community-chat-composer")!;
    const body = composer.querySelector('[data-ui-part="body"]')!;
    const cm = composer.querySelector(".cm-editor")!;
    const page = canvasElement.querySelector(".appearance-composer-fixture")!;
    const action = c.getByRole("button", { name: "Mention someone" });
    for (const theme of themeNames) {
      await waitFor(() =>
        expect(document.documentElement.dataset.storeTheme).toBe(theme),
      );
      await expect(editor).toHaveTextContent("Retained composer draft");
      await expect(getComputedStyle(body).backgroundColor, theme).toBe(
        getComputedStyle(page).backgroundColor,
      );
      // The frameless editor reveals the owned composer surface.
      await expect(getComputedStyle(cm).backgroundColor, theme).toBe(
        "rgba(0, 0, 0, 0)",
      );
      await expect(getComputedStyle(cm).color, theme).toBe(
        getComputedStyle(body).color,
      );
      // Buttons animate color; verify the settled palette rather than a transition frame.
      await waitFor(() =>
        expect(getComputedStyle(action).color, theme).toBe(
          getComputedStyle(body).color,
        ),
      );
      await userEvent.click(c.getByRole("button", { name: "Next palette" }));
    }
    await userEvent.click(editor);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    const mutedText = c.getByText(
      "Text and placeholders follow the selected app palette.",
    );
    for (const theme of themeNames) {
      await waitFor(() =>
        expect(document.documentElement.dataset.storeTheme).toBe(theme),
      );
      await waitFor(() => {
        const placeholder = composer.querySelector(".cm-placeholder");
        expect(placeholder).toHaveTextContent("Write a message");
        expect(getComputedStyle(placeholder!).color, theme).toBe(
          getComputedStyle(mutedText).color,
        );
      });
      await expect(
        c.getByRole("button", { name: "Send message" }),
      ).toBeDisabled();
      await userEvent.click(c.getByRole("button", { name: "Next palette" }));
    }
  }}
/>

<Story
  name="Pending reply keeps recovery text and context readable"
  args={{ pending: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByText("Retained reply context")).toBeVisible();
    await expect(
      c.getByText(
        "Check the original reply status before saving another reply.",
      ),
    ).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Send message" }),
    ).toBeDisabled();
    const composer = canvasElement.querySelector(
      '[data-ui-component="ai-chat-composer"]',
    )!;
    await expect(getComputedStyle(composer).opacity).toBe("1");
  }}
/>

<Story
  name="Saving reply keeps context readable without a status"
  args={{ busy: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByText("Retained reply context")).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Send message" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveAttribute("contenteditable", "false");
    const composer = canvasElement.querySelector(
      '[data-ui-component="ai-chat-composer"]',
    )!;
    await expect(getComputedStyle(composer).opacity).toBe("1");
  }}
/>
