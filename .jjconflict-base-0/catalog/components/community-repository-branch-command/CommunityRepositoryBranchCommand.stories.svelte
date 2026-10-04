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
    const root = canvasElement.querySelector<HTMLElement>(
      ".community-bookmark-cards",
    )!;
    await expect(
      getComputedStyle(root)
        .getPropertyValue("--ui-command-view-item-radius")
        .trim(),
    ).toBe("0px");
    await expect(getComputedStyle(root).borderBottomLeftRadius).toBe("0px");
    await expect(getComputedStyle(root).borderBottomRightRadius).toBe("0px");
    const unselectedBranch = canvas.getByRole("option", {
      name: "feature/native-objects",
    });
    const restingBackground =
      getComputedStyle(unselectedBranch).backgroundColor;
    const keyboardSelection = unselectedBranch.getAttribute("aria-selected");
    // Synthetic pointer events verify command state; native hover is checked in the browser.
    await userEvent.hover(unselectedBranch);
    await expect(unselectedBranch.getAttribute("aria-selected")).toBe(
      keyboardSelection,
    );
    await userEvent.unhover(unselectedBranch);
    await expect(getComputedStyle(unselectedBranch).backgroundColor).toBe(
      restingBackground,
    );
    const selectedBranch = canvas.getByRole("option", {
      name: "feature/publication",
    });
    const selectedBackground = getComputedStyle(selectedBranch).backgroundColor;
    await userEvent.hover(selectedBranch);
    await expect(getComputedStyle(selectedBranch).backgroundColor).toBe(
      selectedBackground,
    );
    await userEvent.unhover(selectedBranch);
    await userEvent.click(canvas.getByPlaceholderText("Search branches"));
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(args.onSelect).toHaveBeenLastCalledWith(
      "feature/native-objects",
    );
    const workingCopyTitle = workingCopy.querySelector("strong")!;
    const branchTitle = canvas
      .getByRole("option", { name: "main" })
      .querySelector('[data-ui-part="item-label"]')!;
    await expect(getComputedStyle(workingCopyTitle).fontSize).toBe(
      getComputedStyle(branchTitle).fontSize,
    );
    await userEvent.click(workingCopy);
    await expect(args.onSelectWorkingCopy).toHaveBeenLastCalledWith(
      workingCopies[0]!.commitId,
      workingCopies[0]!.workspace,
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

<Story
  name="Empty filter states"
  args={{
    bookmarks: bookmarks.filter(
      (bookmark) => bookmark.name === "feature/native-objects",
    ),
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);

    await userEvent.click(canvas.getByRole("tab", { name: "Workspace" }));
    const workspaceEmpty = canvas.getByRole("status");
    await expect(workspaceEmpty).toHaveTextContent("No workspace bookmarks");
    await expect(
      workspaceEmpty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();

    await userEvent.click(canvas.getByRole("tab", { name: "Conflicted" }));
    const conflictedEmpty = canvas.getByRole("status");
    await expect(conflictedEmpty).toHaveTextContent("No conflicted bookmarks");
    await expect(
      conflictedEmpty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
  }}
/>
