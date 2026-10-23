<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/GlobalSettingsLoadingFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-settings/StoreGlobalSettingsLoading.svelte",
      },
    },
    title: "JJ Store/Settings/Global loading",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
  async function loading({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement),
      busy = c.getByRole("status");
    await expect(busy).toHaveAttribute("aria-busy", "true");
    await expect(within(busy).queryAllByRole("textbox")).toHaveLength(0);
    await expect(within(busy).queryAllByRole("button")).toHaveLength(0);
    const frame = c.getByTestId("loading-frame");
    await expect(frame.scrollWidth).toBeLessThanOrEqual(frame.clientWidth);
    const nav = c.getByRole("navigation", { name: "Settings page navigation" });
    const viewport = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(viewport).not.toContainElement(nav);
    const y = nav.getBoundingClientRect().y;
    viewport.scrollTop = 150;
    await expect(nav.getBoundingClientRect().y).toBe(y);
    const card = busy.querySelector(".store-settings-skeleton__card")!,
      row = card.querySelector(".store-settings-skeleton__row")!;
    await expect(
      Math.abs(row.getBoundingClientRect().width - card.clientWidth),
    ).toBeLessThanOrEqual(2);
  }
</script>

<Story name="Runtime preferences and actions" play={loading} />
<Story
  name="Launch inventory"
  args={{ section: "service", tab: "launch" }}
  play={loading}
/>
<Story
  name="Storage inventory dark"
  args={{ section: "service", tab: "storage" }}
  globals={{ colorMode: "dark" }}
  play={loading}
/>
<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Compute resources narrow"
  args={{ section: "compute", tab: "resources", narrow: true }}
  play={loading}
/>
<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Agent configuration overflow"
  args={{ section: "agents", tab: "configuration", narrow: true }}
  play={loading}
/>
<Story
  name="Keyboard selects Appearance loading page"
  args={{ section: "appearance", tab: "theme" }}
  play={async (context) => {
    await loading(context);
    const c = within(context.canvasElement),
      theme = await c.findByRole("tab", { name: "Theme" }, { timeout: 10000 });
    theme.focus();
    await userEvent.keyboard("{ArrowRight}{Enter}");
    await expect(c.getByRole("tab", { name: "Preferences" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(c.getByRole("status")).toHaveAttribute(
      "data-tab",
      "preferences",
    );
  }}
/>
