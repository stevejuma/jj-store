<script module lang="ts">
  import { workspaceFileName } from "../../fixtures/stack-workspace/story-actions.js";
  import { views as applicationViews } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  import {
    messagePreview,
    assertCompactColumnToggle,
  } from "../../fixtures/stack-workspace/story-actions.js";
  import { assertCanvasInsets } from "../../fixtures/stack-workspace/canvas-layout.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Application",
    component: StackWorkspaceApplicationFixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    beforeEach: async () => {
      await Promise.all([
        applicationViews.stacks(),
        applicationViews.files(),
        applicationViews.log(),
        applicationViews.stackDetails(),
        applicationViews.stackConflicts(),
      ]);
    },
  });
</script>

<Story
  name="Whole stack files and continuous graph"
  args={{ continuousStack: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in stack feature/api",
      }),
    );
    await expect(
      stack.querySelector(".stack-workspace__bookmark-header"),
    ).toHaveAttribute("data-selected");
    const files = within(stack).getByRole("region", {
      name: "Changed files in branch feature/api",
    });
    await expect(
      within(files).getByRole("button", {
        name: workspaceFileName("src/model.ts"),
      }),
    ).toBeVisible();
    await userEvent.click(
      within(files).getByRole("button", {
        name: workspaceFileName("src/api.ts"),
      }),
    );
    await expect(
      await canvas.findByRole("group", {
        name: "Draggable hunk query endpoint",
      }),
    ).toHaveAttribute("draggable", "true");
    await expect(
      canvas.getByRole("button", { name: "Assign hunk query endpoint" }),
    ).toBeEnabled();
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    await expect(
      stack.querySelector('[aria-current="true"] [data-revision-id]'),
    ).toHaveAttribute("data-revision-id", "api2");
    await waitFor(() => {
      for (const row of stack.querySelectorAll<HTMLElement>(
        "[data-revision-id]",
      )) {
        const svg = row.querySelector<SVGElement>(
          ".community-revision-graph__svg",
        )!;
        const stem = svg.querySelector("line")!;
        expect(stem.getAttribute("y1")).toBe("0");
        expect(Number(stem.getAttribute("y2"))).toBeCloseTo(
          row.parentElement!.getBoundingClientRect().height,
          0,
        );
        expect(svg.getBoundingClientRect().height).toBeCloseTo(
          row.parentElement!.getBoundingClientRect().height,
          0,
        );
      }
      const rows = [
        ...stack.querySelectorAll<HTMLElement>("[data-revision-id]"),
      ];
      expect(
        rows[0]!.querySelector("svg")!.getBoundingClientRect().bottom,
      ).toBeCloseTo(rows[1]!.getBoundingClientRect().top, 0);
    });
  }}
/>
