<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositoryFileSkeletonFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Repository file loading",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function frame({ canvasElement }: { canvasElement: HTMLElement }) {
    const canvas = within(canvasElement);
    const status = await canvas.findByRole("status", {
      name: /Loading (README.md|src\/main.rs|repository files)$/,
    });
    await expect(status).toHaveAttribute("aria-busy", "true");
    const card = status.querySelector(
      ".community-projects__file-content-card",
    )!;
    await expect(card).not.toBeNull();
    await expect(
      status.querySelectorAll(".community-projects__file-panel-info"),
    ).toHaveLength(1);
    await expect(
      status.querySelectorAll(".community-projects__file-panel-stats"),
    ).toHaveLength(1);
    await expect(getComputedStyle(card).borderTopWidth).toBe("1px");
    await expect(
      parseFloat(getComputedStyle(card).borderTopLeftRadius),
    ).toBeGreaterThan(0);
    await expect(
      card.querySelector(".community-projects__file-content-header"),
    ).not.toBeNull();
    await expect(status.querySelectorAll('[role="status"]')).toHaveLength(0);
    await expect(status.scrollWidth).toBeLessThanOrEqual(status.clientWidth);
    await expect(within(status).queryByRole("button")).toBeNull();
  }
</script>

<Story
  name="Markdown detail loading retains metadata and viewer frame"
  play={frame}
/>
<Story
  name="Source detail loading retains numbered content and viewer frame"
  args={{ path: "src/main.rs" }}
  play={frame}
/>
<Story
  name="Initial source snapshot uses the same viewer frame"
  args={{ mode: "initial", path: "src/main.rs" }}
  play={frame}
/>
<Story
  name="Module loading uses the same Markdown viewer frame and breadcrumb inset"
  args={{ mode: "module" }}
  play={async (context) => {
    await frame(context);
    const crumb = context.canvasElement.querySelector(
      ".store-application__body-breadcrumb",
    )!;
    const card = context.canvasElement.querySelector(
      ".community-projects__file-content-card",
    )!;
    await expect(getComputedStyle(crumb).borderBottomWidth).toBe("0px");
    await expect(
      crumb.firstElementChild!.getBoundingClientRect().left,
    ).toBeCloseTo(card.getBoundingClientRect().left + 4, 0);
  }}
/>
<Story
  name="Narrow Markdown viewer skeleton does not overflow"
  args={{ width: 320 }}
  play={frame}
/>
<Story
  name="Narrow source viewer skeleton does not overflow"
  args={{ width: 320, path: "src/main.rs" }}
  play={frame}
/>
