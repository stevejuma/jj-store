<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import CommunityRepositoryBranchCommand from "../../../web/src/lib/ui/community-repository-branch-command/CommunityRepositoryBranchCommand.svelte";
  import { bookmarks, NOW, workingCopies } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Branch cards",
    component: CommunityRepositoryBranchCommand,
    tags: ["visual-pending"],
    args: {
      bookmarks,
      workingCopies,
      now: NOW,
      selected: "feature/publication",
      onSelect: fn(),
      onSelectWorkingCopy: fn(),
    },
  });
</script>

<Story
  name="Bookmark cards"
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement);
    const workingCopy = canvas.getByRole("button", { name: /Working copy/ });
    const filters = canvas.getByRole("tablist", { name: "Bookmark filters" });
    await expect(
      Boolean(
        workingCopy.compareDocumentPosition(filters) &
        Node.DOCUMENT_POSITION_FOLLOWING,
      ),
    ).toBe(true);
    await userEvent.click(workingCopy);
    await expect(args.onSelectWorkingCopy).toHaveBeenLastCalledWith(
      workingCopies[0]!.commitId,
    );
    const expectSeparators = async () => {
      const cards = canvas.getAllByRole("option");
      for (const card of cards) {
        await expect(getComputedStyle(card).borderBottomWidth).toBe("1px");
        await expect(getComputedStyle(card).borderBottomLeftRadius).toBe("0px");
        await expect(getComputedStyle(card).borderBottomRightRadius).toBe(
          "0px",
        );
      }
    };
    await expectSeparators();
    await expect(
      canvas.getByText("Stream immutable native objects"),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Workspace" }));
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await expectSeparators();
    await expect(canvas.getByRole("option", { name: "main" })).toBeVisible();
    await expect(
      canvas.getByRole("option", { name: "main" }),
    ).toHaveTextContent("just now by Maya Chen");
    await userEvent.click(canvas.getByRole("tab", { name: "Conflicted" }));
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await expectSeparators();
    await userEvent.click(canvas.getByRole("option", { name: "conflicted" }));
    await expect(args.onSelect).toHaveBeenLastCalledWith("conflicted");
    await userEvent.click(canvas.getByRole("tab", { name: "All" }));
    await userEvent.type(
      canvas.getByPlaceholderText("Search branches"),
      "native-objects",
    );
    await expect(canvas.getAllByRole("option")).toHaveLength(1);
    await expectSeparators();
    await expect(
      canvas.getByRole("option", {
        name: "feature/native-objects",
      }),
    ).toBeVisible();
  }}
/>
<Story name="Empty bookmarks" args={{ bookmarks: [] }} />
