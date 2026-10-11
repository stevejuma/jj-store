<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositorySettingsLoadingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Loading",
    component: Fixture,
    tags: ["visual-pending"],
  });
  async function loading({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const busy = c.getByRole("status", { name: "Loading repository settings" });
    await expect(busy).toHaveAttribute("aria-busy", "true");
    await expect(c.queryAllByRole("button")).toHaveLength(0);
    await expect(c.queryAllByRole("tab")).toHaveLength(0);
    await expect(c.queryAllByRole("textbox")).toHaveLength(0);
    const frame = canvasElement.querySelector("main")!;
    await expect(frame.scrollWidth).toBeLessThanOrEqual(frame.clientWidth);
    const card = canvasElement.querySelector(".store-settings-skeleton__card")!;
    const row = card.querySelector(".store-settings-skeleton__row")!;
    await expect(
      Math.abs(row.getBoundingClientRect().width - card.clientWidth),
    ).toBeLessThanOrEqual(2);
    const viewport = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(viewport).not.toBeNull();
    const navigation = canvasElement.querySelector(
      ".store-settings-loading__primary",
    )!;
    const y = navigation.getBoundingClientRect().y;
    viewport.scrollTop = 100;
    await expect(navigation.getBoundingClientRect().y).toBe(y);
  }
</script>

<Story name="Native General discovery" play={loading} />
<Story
  name="Local JJ General discovery"
  args={{ local: true }}
  play={loading}
/>
<Story
  name="Local JJ capabilities"
  args={{ local: true, section: "advanced" }}
  play={loading}
/>
<Story
  name="Native capabilities"
  args={{ section: "advanced" }}
  play={loading}
/>
<Story
  name="Search worker limits on narrow screens"
  args={{ section: "search", tab: "advanced", narrow: true }}
  play={loading}
/>
<Story
  name="Search file scopes"
  args={{ section: "search", tab: "scopes" }}
  play={loading}
/>
<Story
  name="Storage overview"
  args={{ section: "storage", tab: "overview" }}
  play={loading}
/>
<Story
  name="Storage maintenance"
  args={{ section: "storage", tab: "maintenance" }}
  play={loading}
/>
<Story
  name="Sync configuration"
  args={{ section: "sync", tab: "configuration" }}
  play={loading}
/>

<Story
  name="Dark Storage overview"
  args={{ section: "storage" }}
  globals={{ colorMode: "dark" }}
  play={loading}
/>
