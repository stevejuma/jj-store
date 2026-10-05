<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Trailing spacer",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function spacer(element: HTMLElement) {
    const prompt = await within(element).findByRole("group", {
      name: "Create branch spacer",
    });
    const reserve = prompt.closest<HTMLElement>(
      '[data-ui-part="trailing-spacer"]',
    )!;
    expect(reserve).not.toBeNull();
    expect(reserve).not.toHaveAttribute("aria-hidden", "true");
    expect(reserve.closest('[data-ui-part="column"]')).toBeNull();
    expect(reserve.getBoundingClientRect().width).toBe(390);
    expect(getComputedStyle(prompt).backgroundImage).toBe("none");
    expect(getComputedStyle(prompt).borderInlineStartWidth).toBe("0px");
    const empty = prompt.querySelector(
      '[data-ui-component="empty"][data-ui-part="empty"]',
    )!;
    const header = prompt.querySelector('[data-ui-part="empty-header"]')!;
    const content = prompt.querySelector('[data-ui-part="empty-content"]')!;
    expect(empty).not.toBeNull();
    const bounds = prompt.getBoundingClientRect();
    expect(
      Math.abs(
        (header.getBoundingClientRect().top +
          content.getBoundingClientRect().bottom) /
          2 -
          (bounds.top + bounds.bottom) / 2,
      ),
    ).toBeLessThan(1);
    expect(
      prompt.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    return prompt;
  }
</script>

<Story
  name="Centered branch prompt occupies the existing spacer"
  play={async ({ canvasElement }) => {
    const prompt = await spacer(canvasElement);
    const canvas = within(canvasElement);
    expect(canvas.getAllByRole("region", { name: /^Stack / })).toHaveLength(2);
    await userEvent.click(
      within(prompt).getByRole("button", { name: "Create a new branch" }),
    );
    const dialog = await workspacePortal(canvasElement).findByRole("dialog", {
      name: "Create stack",
    });
    expect(
      within(dialog).getByRole("group", { name: "Branch relationship" }),
    ).toBeVisible();
    await userEvent.type(
      within(dialog).getByLabelText("Name"),
      "feature/from-spacer",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("region", { name: "Stack feature/from-spacer" }),
    ).toBeVisible();
    await spacer(canvasElement);
  }}
/>

<Story
  name="Drop files into the spacer retains exact staged changes"
  play={async ({ canvasElement }) => {
    const prompt = await spacer(canvasElement);
    const canvas = within(canvasElement);
    const source = canvas.getByRole("button", { name: "README.md" });
    const transfer = new DataTransfer();
    await fireEvent(
      source,
      new DragEvent("dragstart", { bubbles: true, dataTransfer: transfer }),
    );
    await fireEvent(
      prompt,
      new DragEvent("dragover", {
        bubbles: true,
        cancelable: true,
        dataTransfer: transfer,
      }),
    );
    expect(prompt).toHaveAttribute("data-drop-target-state", "valid");
    expect(
      within(prompt).getByRole("status", { name: "Drop guide" }),
    ).toHaveTextContent("Start a branch with these changes");
    await fireEvent(
      prompt,
      new DragEvent("drop", {
        bubbles: true,
        cancelable: true,
        dataTransfer: transfer,
      }),
    );
    const dialog = await workspacePortal(canvasElement).findByRole("dialog", {
      name: "Create stack",
    });
    expect(
      within(dialog).getByRole("list", { name: "Selected changes" }),
    ).toHaveTextContent("README.md · 1 hunk(s)");
    await userEvent.type(
      within(dialog).getByLabelText("Name"),
      "feature/dropped",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    const stack = await canvas.findByRole("region", {
      name: "Stack feature/dropped",
    });
    expect(
      within(stack).getByRole("group", {
        name: "Staged files for feature/dropped",
      }),
    ).toHaveTextContent("README.md");
    expect(within(stack).getByLabelText("Commit title")).toHaveValue("");
    expect(
      within(canvas.getByRole("region", { name: "Unstaged files" })).getByRole(
        "button",
        { name: "README.md" },
      ),
    ).toBeVisible();
    await userEvent.type(
      within(stack).getByLabelText("Commit title"),
      "Commit dropped readme",
    );
    await userEvent.click(
      within(stack).getByRole("button", { name: "Create commit" }),
    );
    await applyPreview(canvasElement);
    expect(
      within(stack).getByRole("button", { name: "Commit dropped readme" }),
    ).toBeVisible();
    expect(
      within(
        canvas.getByRole("region", { name: "Unstaged files" }),
      ).queryByRole("button", { name: "README.md" }),
    ).toBeNull();
  }}
/>

<Story
  name="Empty workspace keeps its branch prompt centered"
  args={{ empty: true, narrow: true }}
  play={async ({ canvasElement }) => {
    await spacer(canvasElement);
  }}
/>
<Story
  name="Dark workspace branch prompt"
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    await spacer(canvasElement);
  }}
/>
<Story
  name="Branches page keeps a blank trailing spacer"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByRole("region", { name: "Branches" });
    expect(
      within(canvasElement).queryByRole("group", {
        name: "Create branch spacer",
      }),
    ).toBeNull();
    const reserve = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="trailing-spacer"]',
    )!;
    expect(reserve).toHaveAttribute("aria-hidden", "true");
    expect(reserve.getBoundingClientRect().width).toBe(240);
  }}
/>
