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
      await canvas.findByRole("region", { name: "Repository files" }),
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
      // JST-UI-015: Overview is flush, retaining space below the workspace header.
      await expect(inset).toBe("0px");
    }
    await expect(style.borderTopWidth).toBe("0px");
    await expect(style.marginTop).toBe("8px");
    await expect(body.scrollWidth).toBeLessThanOrEqual(body.clientWidth);
    for (const selector of [
      ".store-application__tabs",
      ".store-application__section-header",
    ]) {
      const header = canvasElement.querySelector<HTMLElement>(selector)!;
      await expect(
        header.firstElementChild!.getBoundingClientRect().left -
          header.getBoundingClientRect().left,
      ).toBeLessThanOrEqual(8);
    }
    const breadcrumb = canvasElement.querySelector(
      ".store-application__body-breadcrumb",
    )!;
    const card = canvasElement.querySelector(
      ".community-projects__file-browser-card",
    )!;
    await expect(
      breadcrumb.firstElementChild!.getBoundingClientRect().left,
    ).toBeCloseTo(card.getBoundingClientRect().left + 4, 0);
  }
</script>

<Story name="Overview body inset" play={overviewInset} />
<Story
  name="Overview mobile body inset"
  args={{ displayMode: "mobile" }}
  parameters={{ viewport: { value: "mobile1" } }}
  play={overviewInset}
/>
