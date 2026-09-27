<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import CommunityRepositoryBranchCommand from "./CommunityRepositoryBranchCommand.svelte";
  import { bookmarks, NOW } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Branch cards",
    component: CommunityRepositoryBranchCommand,
    tags: ["visual-pending"],
    args: {
      bookmarks,
      now: NOW,
      selected: "feature/publication",
      onSelect: fn(),
    },
  });
</script>

<Story
  name="Bookmark cards"
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("Stream immutable native objects"),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Workspace" }));
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await expect(canvas.getByRole("option", { name: "main" })).toBeVisible();
    await expect(
      canvas.getByRole("option", { name: "main" }),
    ).toHaveTextContent("just now by Maya Chen");
    await userEvent.click(canvas.getByRole("tab", { name: "Conflicted" }));
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await userEvent.click(canvas.getByRole("option", { name: "conflicted" }));
    await expect(args.onSelect).toHaveBeenLastCalledWith("conflicted");
    await userEvent.click(canvas.getByRole("tab", { name: "All" }));
    await userEvent.type(
      canvas.getByPlaceholderText("Search branches"),
      "native-objects",
    );
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await expect(
      canvas.getByRole("option", {
        name: "feature/native-objects",
      }),
    ).toBeVisible();
  }}
/>
<Story name="Empty bookmarks" args={{ bookmarks: [] }} />
