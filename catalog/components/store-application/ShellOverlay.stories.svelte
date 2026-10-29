<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/ShellOverlayFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Shell overlay scroll ownership",
    component: Fixture,
    tags: ["visual-pending", "prototype"],
    parameters: {
      jjStoreStory: {
        kind: "prototype",
        requirement: "JST-UI-010",
        reason:
          "Explicit structural or conversation design prototype; does not qualify application parity.",
      },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
  async function bounded({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const opener = c.getByRole("button", { name: "Open overlay page" });
    await userEvent.click(opener);
    const sheet = await c.findByRole("region", { name: "Overlay page" });
    const content = within(sheet).getByRole("region", { name: "Page content" });
    const footer = sheet.querySelector<HTMLElement>(
      '[data-ui-part="surface-layer-footer"]',
    )!;
    const body = sheet.querySelector('[data-ui-part="surface-layer-body"]')!;
    await expect(body.querySelector('[data-ui-part="scroll-area"]')).toBeNull();
    await expect(footer).toBeVisible();
    await userEvent.type(
      within(sheet).getByRole("textbox", { name: "Overlay draft" }),
      "Retained draft",
    );
    await waitFor(() =>
      expect(content.scrollHeight).toBeGreaterThan(content.clientHeight),
    );
    const before = footer.getBoundingClientRect().top;
    content.scrollTop = content.scrollHeight;
    await expect(footer.getBoundingClientRect().top).toBeCloseTo(before, 1);
    await userEvent.click(
      within(sheet).getByRole("button", { name: "Close Overlay page" }),
    );
    await waitFor(() => expect(opener).toHaveFocus());
    await userEvent.click(opener);
    await expect(c.getByRole("textbox", { name: "Overlay draft" })).toHaveValue(
      "Retained draft",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Keep draft and close" }),
    );
    await waitFor(() => expect(opener).toHaveFocus());
  }
</script>

<Story name="Fixed footer and child-owned scrolling" play={bounded} />
<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Mobile fixed footer and retained draft"
  args={{ mobile: true }}
  play={bounded}
/>
