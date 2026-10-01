<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository body layout",
    component: Fixture,
    tags: ["visual-pending"],
    args: { initialPage: "files", initiallyCollapsed: true },
    parameters: { layout: "fullscreen" },
  });
  async function overviewInset({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("region", { name: "Repository files" }),
    ).toBeVisible();
    const body = canvasElement.querySelector<HTMLElement>(
      ".store-application__body-regions",
    )!;
    const style = getComputedStyle(body);
    for (const inset of [
      style.paddingTop,
      style.paddingLeft,
      style.paddingRight,
      style.paddingBottom,
    ]) {
      await expect(Number.parseFloat(inset)).toBeGreaterThanOrEqual(12);
    }
    await expect(body.scrollWidth).toBeLessThanOrEqual(body.clientWidth);
    for (const selector of [
      ".store-application__tabs",
      ".store-application__section-header",
      ".store-application__body-breadcrumb",
    ]) {
      const header = canvasElement.querySelector<HTMLElement>(selector)!;
      await expect(
        header.firstElementChild!.getBoundingClientRect().left -
          header.getBoundingClientRect().left,
      ).toBeLessThanOrEqual(8);
    }
  }
</script>

<Story name="Overview body inset" play={overviewInset} />
<Story
  name="Overview mobile body inset"
  args={{ displayMode: "mobile" }}
  parameters={{ viewport: { value: "mobile1" } }}
  play={overviewInset}
/>
