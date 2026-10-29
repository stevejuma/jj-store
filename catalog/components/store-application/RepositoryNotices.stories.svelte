<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import DetailFixture from "../../fixtures/RepositoryUnavailableDetailFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Repository unavailable banners",
    component: Fixture,
    tags: ["visual-pending", "application"],
    args: { collaborationState: "failed", initiallyCollapsed: false },
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });

  async function expectReviewBanner(canvasElement: HTMLElement) {
    const region = await within(canvasElement).findByRole("region", {
      name: "Repository reviews",
    });
    const banner = within(region).getByRole("alert");
    const header = region.querySelector(".community-projects__surface-header")!;
    const bounds = banner.getBoundingClientRect();
    const style = getComputedStyle(banner);
    await expect(bounds.left).toBeCloseTo(
      header.getBoundingClientRect().left,
      0,
    );
    await expect(bounds.right).toBeCloseTo(
      header.getBoundingClientRect().right,
      0,
    );
    await expect(style.borderRadius).toBe(
      getComputedStyle(header).borderRadius,
    );
    await expect(style.borderTopWidth).toBe("1px");
    await expect(style.paddingLeft).toBe("16px");
    await expect(bounds.top - header.getBoundingClientRect().bottom).toBe(8);
    await expect(banner.scrollWidth).toBeLessThanOrEqual(banner.clientWidth);
    await expect(banner).toHaveTextContent("Reviews unavailable");
  }

  async function expectDetailBanner(canvasElement: HTMLElement, title: string) {
    if (title === "Review unavailable") {
      const unavailable = await within(canvasElement).findByRole("status", {
        name: title,
      });
      await expect(unavailable).toBeVisible();
      await expect(unavailable).toHaveTextContent(
        "The collaboration snapshot could not be loaded.",
      );
      const bounds = unavailable.getBoundingClientRect(),
        outer = canvasElement.getBoundingClientRect();
      await expect(bounds.left).toBeGreaterThanOrEqual(outer.left);
      await expect(bounds.right).toBeLessThanOrEqual(outer.right);
      await expect(unavailable.scrollWidth).toBeLessThanOrEqual(
        unavailable.clientWidth,
      );
      return;
    }
    const banner = (await within(canvasElement).findByText(title)).closest(
      '[role="alert"]',
    )!;
    await expect(banner).toBeVisible();
    const style = getComputedStyle(banner);
    await expect(style.borderRadius).toBe("7.2px");
    await expect(style.borderTopWidth).toBe("1px");
    await expect(style.marginLeft).toBe("16px");
    await expect(style.marginRight).toBe("16px");
    const bounds = banner.getBoundingClientRect();
    const parent = banner.parentElement!.getBoundingClientRect();
    await expect(bounds.left).toBeGreaterThanOrEqual(parent.left + 16);
    await expect(bounds.right).toBeLessThanOrEqual(parent.right - 16);
    await expect(bounds.height).toBeLessThan(160);
  }
</script>

<Story
  name="Reviews banner matches its header"
  args={{ initialPage: "reviews" }}
  play={async ({ canvasElement }) => expectReviewBanner(canvasElement)}
/>
<Story
  name="Unavailable review detail keeps its banner contained"
  play={async ({ canvasElement }) =>
    expectDetailBanner(canvasElement, "Review unavailable")}
>
  {#snippet template()}<DetailFixture />{/snippet}
</Story>
<Story
  name="Unavailable agent response keeps its banner contained"
  play={async ({ canvasElement }) =>
    expectDetailBanner(canvasElement, "Agent response unavailable")}
>
  {#snippet template()}<DetailFixture area="agents" />{/snippet}
</Story>
<Story
  name="Narrow Reviews banner stays inside its header"
  args={{ initialPage: "reviews" }}
  play={async ({ canvasElement }) => {
    canvasElement.querySelector<HTMLElement>(
      ".store-application__body-regions",
    )!.style.width = "320px";
    await expectReviewBanner(canvasElement);
  }}
/>
<Story
  name="Agents banner keeps the collection inset"
  args={{ initialPage: "agents" }}
  play={async ({ canvasElement }) => {
    const region = await within(canvasElement).findByRole("region", {
      name: "Agent activity",
    });
    const banner = within(region).getByRole("alert");
    const style = getComputedStyle(banner);
    await expect(style.borderRadius).toBe("7.2px");
    await expect(style.borderTopWidth).toBe("1px");
    await expect(style.marginLeft).toBe("16px");
    await expect(style.marginRight).toBe("16px");
    await expect(style.marginTop).toBe("8px");
    await expect(banner.scrollWidth).toBeLessThanOrEqual(banner.clientWidth);
  }}
/>
