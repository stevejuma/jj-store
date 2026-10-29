<script module lang="ts">
  import { workspaceFileName } from "../../fixtures/stack-workspace/story-actions.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    commandMenu,
    dropWorkspaceChange,
    messagePreview,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace/Menus",
    component: StackWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  async function submenu(element: HTMLElement, label: string, item: string) {
    const portal = workspacePortal(element);
    await userEvent.click(await portal.findByRole("menuitem", { name: label }));
    await userEvent.keyboard("{ArrowRight}");
    await userEvent.click(await portal.findByRole("menuitem", { name: item }));
  }
</script>

<Story
  name="Commit copy and positional submenus"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    const action = "Actions for Expose repository API";
    const original = navigator.clipboard.writeText;
    let copied = "";
    navigator.clipboard.writeText = async (value) => {
      copied = value;
    };
    try {
      for (const item of [
        "Copy change ID",
        "Copy commit hash",
        "Copy commit message",
      ]) {
        await userEvent.click(
          await canvas.findByRole("button", { name: action }),
        );
        await submenu(canvasElement, "Copy", item);
        expect(copied).toBe(
          item === "Copy commit hash"
            ? "api2"
            : item === "Copy commit message"
              ? "Expose repository API"
              : "61706932000000000000000000000000",
        );
      }
      navigator.clipboard.writeText = async () => {
        throw new Error("Clipboard unavailable");
      };
      await userEvent.click(canvas.getByRole("button", { name: action }));
      await submenu(canvasElement, "Copy", "Copy commit hash");
      await waitFor(async () => {
        expect(
          await portal.findByText("Could not copy. Try again."),
        ).toBeVisible();
      });
      await userEvent.click(
        portal.getByRole("button", { name: "Dismiss Copy failed" }),
      );
    } finally {
      navigator.clipboard.writeText = original;
    }
    for (const placement of ["above", "below"]) {
      await userEvent.click(canvas.getByRole("button", { name: action }));
      await submenu(
        canvasElement,
        "Add empty commit",
        `Add empty commit ${placement}`,
      );
      await messagePreview(canvasElement, `Empty ${placement}`);
      await waitFor(async () => {
        expect(
          await portal.findByText(
            new RegExp(`${placement} “Expose repository API”`),
          ),
        ).toBeVisible();
      });
      await applyPreview(canvasElement);
      const rows = Array.from(
        canvasElement.querySelectorAll("[data-revision-id]"),
      );
      const index = rows.findIndex(
        (row) => row.getAttribute("data-revision-id") === "api2",
      );
      expect(rows[index + (placement === "above" ? -1 : 1)]).toHaveTextContent(
        `Empty ${placement}`,
      );
    }
    for (const placement of ["above", "below"]) {
      await userEvent.click(canvas.getByRole("button", { name: action }));
      await submenu(
        canvasElement,
        "Create branch",
        `Create branch ${placement}`,
      );
      await userEvent.type(
        portal.getByLabelText("Name"),
        `feature/${placement}`,
      );
      await userEvent.click(
        portal.getByRole("button", { name: "Preview command" }),
      );
      await applyPreview(canvasElement);
      expect(
        canvas.getByRole("button", {
          name: `Show all changes in feature/${placement}`,
        }),
      ).toBeVisible();
    }
  }}
/>

<Story
  name="Edit commit in inline composer"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await commandMenu(
      canvasElement,
      "Actions for Expose repository API",
      "Edit commit",
    );
    let form = within(
      canvas.getByRole("form", { name: "Edit commit in feature/api" }),
    );
    expect(form.getByLabelText("Commit title")).toHaveValue(
      "Expose repository API",
    );
    expect(
      form.getByRole("button", { name: workspaceFileName("src/api.ts") }),
    ).toBeVisible();
    expect(form.getByRole("group", { name: "Commit description" })).toHaveClass(
      "community-chat-composer",
    );
    await userEvent.clear(form.getByLabelText("Commit title"));
    await userEvent.type(form.getByLabelText("Commit title"), "Cancelled edit");
    await userEvent.click(form.getByRole("button", { name: "Cancel" }));
    expect(
      canvas.getByRole("button", { name: "Expose repository API" }),
    ).toBeVisible();
    await commandMenu(
      canvasElement,
      "Actions for Expose repository API",
      "Edit commit",
    );
    form = within(
      canvas.getByRole("form", { name: "Edit commit in feature/api" }),
    );
    const unstaged = within(
      canvas.getByRole("region", { name: "Unstaged files" }),
    );
    await dropWorkspaceChange(
      unstaged.getByRole("button", { name: workspaceFileName("README.md") }),
      form.getByRole("group", { name: "Staged files for feature/api" }),
    );
    expect(
      form.getByRole("button", { name: workspaceFileName("README.md") }),
    ).toBeVisible();
    await userEvent.clear(form.getByLabelText("Commit title"));
    await userEvent.type(
      form.getByLabelText("Commit title"),
      "API with documentation",
    );
    await userEvent.click(form.getByRole("button", { name: "Save commit" }));
    await waitFor(async () => {
      expect(
        await portal.findByText(/only explicitly staged additions/),
      ).toBeVisible();
    });
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    expect(form.getByLabelText("Commit title")).toHaveValue(
      "API with documentation",
    );
    await userEvent.click(form.getByRole("button", { name: "Save commit" }));
    await applyPreview(canvasElement);
    expect(
      canvas.queryByRole("form", { name: "Edit commit in feature/api" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "API with documentation" }),
    );
    expect(
      within(
        canvas.getByRole("region", { name: "Stack feature/api" }),
      ).getByRole("button", { name: workspaceFileName("README.md") }),
    ).toBeVisible();
    expect(
      unstaged.queryByRole("button", { name: workspaceFileName("README.md") }),
    ).toBeNull();
  }}
/>

<Story
  name="Delete commit and undo"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await commandMenu(
      canvasElement,
      "Actions for Expose repository API",
      "Delete commit",
    );
    await waitFor(async () => {
      expect(
        await portal.findByText(/and its file changes. Unstaged edits remain/),
      ).toBeVisible();
    });
    await applyPreview(canvasElement);
    expect(
      canvas.queryByRole("button", { name: "Expose repository API" }),
    ).toBeNull();
    await userEvent.click(canvas.getByRole("button", { name: "Undo" }));
    await applyPreview(canvasElement);
    expect(
      canvas.getByRole("button", { name: "Expose repository API" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Branch menu preserves lower segments"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await commandMenu(
      canvasElement,
      "Branch actions for feature/api",
      "Squash all commits",
    );
    await waitFor(async () => {
      expect(
        await portal.findByText(/Lower branch segments remain/),
      ).toBeVisible();
    });
    await applyPreview(canvasElement);
    expect(
      canvas.getByRole("button", { name: "Show all changes in feature/model" }),
    ).toBeVisible();
    await commandMenu(
      canvasElement,
      "Branch actions for feature/api",
      "Rename branch",
    );
    await userEvent.clear(portal.getByLabelText("Name"));
    await userEvent.type(portal.getByLabelText("Name"), "feature/renamed");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await commandMenu(
      canvasElement,
      "Branch actions for feature/renamed",
      "Delete branch",
    );
    await applyPreview(canvasElement);
    expect(
      canvas.queryByRole("button", {
        name: "Show all changes in feature/renamed",
      }),
    ).toBeNull();
    expect(
      canvas.getByRole("button", { name: "Show all changes in feature/model" }),
    ).toBeVisible();
  }}
/>
