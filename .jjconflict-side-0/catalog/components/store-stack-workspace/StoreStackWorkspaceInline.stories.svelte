<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    dropWorkspaceChange,
    workspaceCanvas,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace/Inline",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Branch files stay inside their own card"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/api",
    });
    for (const name of ["feature/model", "feature/api"]) {
      const title = within(stack).getByRole("button", {
        name: `Show all changes in ${name}`,
      });
      const card = title.closest<HTMLElement>(
        ".stack-workspace__bookmark-card",
      )!;
      const header = card.querySelector<HTMLElement>(
        ".stack-workspace__bookmark-header",
      )!;
      await userEvent.click(header);
      expect(title).toHaveAttribute("aria-pressed", "true");
      const files = within(card).getByRole("region", {
        name: `Changed files in branch ${name}`,
      });
      const own = name === "feature/model" ? "src/model.ts" : "src/api.ts";
      const other = name === "feature/model" ? "src/api.ts" : "src/model.ts";
      expect(within(files).getByRole("button", { name: own })).toBeVisible();
      expect(within(files).queryByRole("button", { name: other })).toBeNull();
      expect(within(header).getByText("Selected")).toBeVisible();
      expect(files.previousElementSibling).toBe(header);
      expect(files.getBoundingClientRect().top).toBeGreaterThanOrEqual(
        header.getBoundingClientRect().bottom - 1,
      );
      expect(
        stack
          .querySelector(".stack-workspace__stage-card")!
          .compareDocumentPosition(files) & Node.DOCUMENT_POSITION_FOLLOWING,
      ).toBeTruthy();
      await userEvent.click(within(files).getByRole("button", { name: own }));
      expect(
        await canvas.findByRole("region", {
          name: "Selected changes from stack-api",
        }),
      ).toBeVisible();
      expect(title).toHaveAttribute("aria-pressed", "true");
    }
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in stack feature/api",
      }),
    );
    const top = within(stack)
      .getByRole("button", { name: "Show all changes in feature/api" })
      .closest(".stack-workspace__bookmark-card")!;
    expect(
      within(top as HTMLElement).getByRole("region", {
        name: "Changed files in branch feature/api",
      }),
    ).toBeVisible();
    await userEvent.click(
      within(stack).getByRole("button", { name: "Expose repository API" }),
    );
    expect(
      within(stack).queryByRole("region", {
        name: "Changed files in branch feature/api",
      }),
    ).toBeNull();
    expect(
      within(stack).getByRole("region", {
        name: "Changed files in feature/api",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Start commit becomes a staged stack card"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const stack = within(
      await canvas.findByRole("region", { name: "Stack feature/api" }),
    );
    const composer = stack.getByRole("group", {
      name: "Commit composer for feature/api",
    });
    const drop = composer.querySelector<HTMLElement>(
      ".stack-workspace__stage-drop",
    )!;
    const start = composer.querySelector<HTMLElement>(
      ".stack-workspace__stage-start",
    )!;
    expect(getComputedStyle(start).borderTopWidth).toBe("1px");
    expect(
      Math.abs(
        start.getBoundingClientRect().top - drop.getBoundingClientRect().bottom,
      ),
    ).toBeLessThanOrEqual(1);
    await userEvent.click(
      stack.getByRole("button", { name: "Create commit in feature/api" }),
    );
    let form = stack.getByRole("form", {
      name: "Create commit in feature/api",
    });
    let staged = within(form).getByRole("group", {
      name: "Staged files for feature/api",
    });
    expect(within(staged).getByText("Staged", { exact: true })).toBeVisible();
    expect(
      within(staged).getByRole("radio", { name: "Show files as list" }),
    ).toBeChecked();
    expect(form.querySelector(".community-chat-composer")).not.toBeNull();
    expect(composer.querySelector(".stack-workspace__stage-start")).toBeNull();
    await userEvent.click(within(form).getByRole("button", { name: "Cancel" }));
    const unstaged = within(
      canvas.getByRole("region", { name: "Unstaged files" }),
    );
    await dropWorkspaceChange(
      unstaged.getByRole("button", { name: "README.md" }),
      composer,
    );
    form = stack.getByRole("form", { name: "Create commit in feature/api" });
    staged = within(form).getByRole("group", {
      name: "Staged files for feature/api",
    });
    expect(
      within(staged).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    const header = staged.querySelector<HTMLElement>(
      ".stack-workspace__staged-header",
    )!;
    const list = staged.querySelector<HTMLElement>(
      ".stack-workspace__staged-list",
    )!;
    const fields = form.querySelector<HTMLElement>(
      ".stack-workspace__stage-form",
    )!;
    expect(within(header).getByText("1", { exact: true })).toBeVisible();
    expect(header.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      list.getBoundingClientRect().top + 1,
    );
    expect(list.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      fields.getBoundingClientRect().top + 1,
    );
    expect(getComputedStyle(fields).borderTopWidth).toBe("1px");
    await userEvent.click(
      within(staged).getByRole("radio", { name: "Show files as folders" }),
    );
    expect(
      within(staged).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    expect(unstaged.getByRole("button", { name: "README.md" })).toBeVisible();
  }}
/>
