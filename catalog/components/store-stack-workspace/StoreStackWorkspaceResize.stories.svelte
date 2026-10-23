<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import { workspaceCanvas } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Column resizing",
    component: StackWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Resizing stack columns keeps natural commit row heights"
  args={{}}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    const rows = [...stack.querySelectorAll<HTMLElement>("[data-revision-id]")];
    const heights = rows.map((row) => row.getBoundingClientRect().height);
    const handle = within(stack).getByRole("separator");
    const originalWidth = stack.getBoundingClientRect().width;
    handle.focus();
    for (let i = 0; i < 10; i++) await userEvent.keyboard("{ArrowRight}");
    await waitFor(() =>
      expect(stack.getBoundingClientRect().width).toBeGreaterThan(440),
    );
    await waitFor(() => {
      for (let i = 0; i < rows.length; i++)
        expect(rows[i]!.getBoundingClientRect().height).toBeLessThanOrEqual(
          heights[i]! + 1,
        );
    });
    for (let i = 0; i < 10; i++) await userEvent.keyboard("{ArrowLeft}");
    await waitFor(() =>
      expect(stack.getBoundingClientRect().width).toBeCloseTo(originalWidth, 0),
    );
    await waitFor(() => {
      for (let i = 0; i < rows.length; i++)
        expect(rows[i]!.getBoundingClientRect().height).toBeCloseTo(
          heights[i]!,
          0,
        );
    });
  }}
/>
