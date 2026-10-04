<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    dropWorkspaceChange,
    chooseWorkspace,
    commandMenu,
    messagePreview,
    unassignedFile,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Unstaged working files"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("region", { name: "Stack feature/api" });
    expect(canvas.getByText("feature/model", { exact: true })).toBeVisible();
    expect(canvas.queryByRole("radio", { name: "Combined files" })).toBeNull();
    await unassignedFile(canvasElement, "src/settings.ts");
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show split diff" }),
    );
    expect(
      canvasElement.querySelector('[data-view-mode="split"]'),
    ).not.toBeNull();
  }}
/>

<Story
  name="Named combinations retain edits"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await canvas.findByRole("button", { name: "src/settings.ts" });
    await chooseWorkspace(canvasElement, "Experiment");
    await expect(
      canvas.getByRole("button", { name: "src/experiment.ts" }),
    ).toBeVisible();
    await chooseWorkspace(canvasElement, "Daily work");
    await expect(
      canvas.getByRole("button", { name: "src/settings.ts" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Commit one hunk and undo"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await unassignedFile(canvasElement, "src/settings.ts");
    await userEvent.click(
      canvas.getByRole("checkbox", { name: "remember preference" }),
    );
    await dropWorkspaceChange(
      canvas.getByRole("group", {
        name: "Draggable hunk add keyboard shortcut",
      }),
      canvas.getByRole("button", { name: "Assign to feature/workspace" }),
    );
    await messagePreview(canvasElement, "Add keyboard shortcut");
    await expect(
      workspacePortal(canvasElement).getByText(/using only the staged changes/),
    ).toBeVisible();
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("button", {
        name: "Add keyboard shortcut",
      }),
    ).toBeVisible();
    await unassignedFile(canvasElement, "src/settings.ts");
    await expect(
      canvas.queryByRole("checkbox", { name: "add keyboard shortcut" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("checkbox", { name: "remember preference" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Undo" }));
    await applyPreview(canvasElement);
    await unassignedFile(canvasElement, "src/settings.ts");
    await expect(
      canvas.getByRole("checkbox", { name: "add keyboard shortcut" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Branches apply and base-only workspace"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/api" }),
    );
    await commandMenu(
      canvasElement,
      "Actions for feature/api",
      "Unapply stack",
    );
    await applyPreview(canvasElement);
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/workspace" }),
    );
    await commandMenu(
      canvasElement,
      "Actions for feature/workspace",
      "Unapply stack",
    );
    await applyPreview(canvasElement);
    await userEvent.click(
      within(
        canvas.getByRole("tablist", { name: "Local repository views" }),
      ).getByRole("tab", { name: "Workspace" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Browse stacks to apply" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Branches" }));
    await userEvent.click(canvas.getByRole("option", { name: "feature/api" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Apply feature/api" }),
    );
    await applyPreview(canvasElement);
    await userEvent.click(
      within(
        canvas.getByRole("tablist", { name: "Local repository views" }),
      ).getByRole("tab", { name: "Workspace" }),
    );
    await expect(
      canvas.getByRole("region", { name: "Stack feature/api" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Create workspace and stack"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await commandMenu(canvasElement, "Workspace actions", "Create workspace");
    await userEvent.type(await portal.findByLabelText("Name"), "Review");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("combobox", { name: "Active stack workspace" }),
    ).toHaveTextContent("Review");
    await userEvent.click(canvas.getByRole("button", { name: "Create stack" }));
    await userEvent.type(
      await portal.findByLabelText("Name"),
      "feature/review",
    );
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("region", { name: "Stack feature/review" }),
    ).toBeVisible();
    await expect(
      canvas.getByText(
        "No commits yet. Create a commit or assign selected changes.",
      ),
    ).toBeVisible();
  }}
/>

<Story
  name="Commit message editing and keyboard menu"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const menu = await canvas.findByRole("button", {
      name: "Actions for Expose repository API",
    });
    menu.focus();
    await userEvent.keyboard("{Enter}");
    await userEvent.click(
      await workspacePortal(canvasElement).findByRole("menuitem", {
        name: "Reword commit",
      }),
    );
    await messagePreview(canvasElement, "Expose bounded repository API");
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("button", {
        name: "Expose bounded repository API",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Conflict preview and resolution"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/search" }),
    );
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Apply feature/search",
      }),
    );
    await expect(
      await portal.findByText("Applying introduces conflicts"),
    ).toBeVisible();
    await applyPreview(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve conflicts" }),
    );
    await userEvent.click(
      await portal.findByRole("button", {
        name: "Accept All Incoming Changes from Left",
      }),
    );
    await waitFor(() =>
      expect(
        portal.getByRole("button", { name: "Preview resolution" }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      portal.getByRole("button", { name: "Preview resolution" }),
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.queryByText("Workspace has unresolved conflicts"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Native checkout"
  args={{ profile: "native" }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByRole("region", {
      name: "Unstaged files",
    });
    await unassignedFile(canvasElement, "README.md");
    await expect(
      within(canvasElement).getByRole("button", {
        name: "Assign hunk document shortcut",
      }),
    ).toBeEnabled();
  }}
/>
<Story name="Empty local repository" args={{ empty: true }} />
<Story
  name="Local folder unavailable"
  args={{ availability: "unavailable" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await expect(
      await canvas.findByText("Local checkout unavailable"),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Create stack" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Native repository without local link"
  args={{ profile: "native", availability: "unlinked" }}
/>
<Story name="Initial loading" args={{ loading: true }} />
<Story
  name="Immutable commit"
  args={{ immutable: true }}
  play={async ({ canvasElement }) => {
    await expect(
      await within(canvasElement).findByRole("button", {
        name: "Actions for Prepare add repository model",
      }),
    ).toBeEnabled();
    await userEvent.click(
      within(canvasElement).getByRole("button", {
        name: "Actions for Prepare add repository model",
      }),
    );
    const menu = workspacePortal(canvasElement);
    expect(
      menu.getByRole("menuitem", { name: "Reword commit" }),
    ).toHaveAttribute("aria-disabled", "true");
    expect(menu.getByRole("menuitem", { name: "Copy" })).not.toHaveAttribute(
      "aria-disabled",
      "true",
    );
    await userEvent.keyboard("{Escape}");
  }}
/>
<Story
  name="Narrow layout"
  args={{ narrow: true }}
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    await expect(
      await canvas.findByRole("tab", { name: "Workspace" }),
    ).toBeVisible();
    await expect(
      canvasElement.querySelector(".stack-workspace")!.scrollWidth,
    ).toBeLessThanOrEqual(390);
  }}
/>
