<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StackWorkspaceGraphFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Graph scrolling",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function graphScroll(canvasElement: HTMLElement, name: string) {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", { name: `Stack ${name}` });
    await within(stack).findByRole("button", { name: "Graph merge" });
    const row = (id: string) =>
      stack.querySelector<HTMLElement>(`[data-revision-id="${id}"]`)!;
    expect(parseFloat(getComputedStyle(row("tip")).paddingLeft)).toBe(32);
    expect(
      parseFloat(getComputedStyle(row("merge")).paddingLeft),
    ).toBeGreaterThan(32);
    const viewport = stack.querySelector<HTMLElement>(
      '[data-ui-part="column-body"] > [data-ui-part="scroll-area"] > [data-ui-part="scroll-area-viewport"]',
    )!;
    expect(["auto", "scroll"]).toContain(getComputedStyle(viewport).overflowX);
    expect(viewport.scrollWidth).toBeGreaterThan(viewport.clientWidth);
    expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight);
    const left = row("tip").getBoundingClientRect().left;
    viewport.scrollLeft = 80;
    viewport.dispatchEvent(new Event("scroll"));
    await waitFor(() =>
      expect(row("tip").getBoundingClientRect().left).toBeLessThan(left),
    );
    viewport.scrollLeft = 0;
    await userEvent.click(
      within(stack).getByRole("button", { name: "Graph tip" }),
    );
    expect(
      await within(stack).findByRole("region", {
        name: `Changed files in ${name}`,
      }),
    ).toBeVisible();
    viewport.scrollTop = viewport.scrollHeight;
    viewport.dispatchEvent(new Event("scroll"));
    expect(
      await within(stack).findByRole("button", { name: "Graph base" }),
    ).toBeVisible();
  }
</script>

<Story
  name="Workspace graph uses row lanes and scrolls both axes"
  play={async ({ canvasElement }) => graphScroll(canvasElement, "feature/api")}
/>
<Story
  name="Branches graph uses row lanes and scrolls both axes"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    await userEvent.click(
      await within(canvasElement).findByRole("option", { name: "feature/api" }),
    );
    await graphScroll(canvasElement, "feature/api");
  }}
/>
<Story
  name="Target graph uses row lanes and scrolls both axes"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    await userEvent.click(
      await within(canvasElement).findByRole("option", { name: "main" }),
    );
    await graphScroll(canvasElement, "main");
  }}
/>
