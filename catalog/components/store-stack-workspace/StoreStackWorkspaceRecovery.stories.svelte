<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    commandMenu,
    messagePreview,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace/Recovery",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function createPreview(element: HTMLElement) {
    const canvas = workspaceCanvas(element);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Create commit in feature/api",
      }),
    );
    await messagePreview(element, "Retained commit draft");
  }
</script>

<Story
  name="Immediate membership opens recovery only after an unknown outcome"
  args={{ page: "branches", fault: "unknown" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/api" }),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "Unapply feature/api" }),
    );
    await portal.findByRole("dialog", { name: "Workspace outcome unknown" });
    expect(portal.getByText("Outcome awaiting confirmation")).toBeVisible();
    expect(portal.queryByRole("button", { name: "Apply command" })).toBeNull();
    expect(portal.queryByRole("button", { name: "Cancel preview" })).toBeNull();
    expect(canvas.getByRole("button", { name: "Create stack" })).toBeDisabled();
    await userEvent.click(portal.getByRole("button", { name: "Check status" }));
    await canvas.findByRole("button", { name: "Apply feature/api" });
    expect(portal.queryByRole("dialog")).toBeNull();
  }}
/>

<Story
  name="Unknown outcome status"
  args={{ fault: "unknown" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await createPreview(canvasElement);
    await userEvent.click(
      portal.getByRole("button", { name: "Apply command" }),
    );
    await expect(
      await portal.findByText("Outcome awaiting confirmation"),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Create stack" }),
    ).toBeDisabled();
    await expect(
      portal.getByRole("button", { name: "Cancel preview" }),
    ).toBeDisabled();
    await userEvent.click(portal.getByRole("button", { name: "Check status" }));
    await expect(
      await canvas.findByRole("button", {
        name: "Retained commit draft",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Unknown outcome exact retry"
  args={{ fault: "unknown" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await createPreview(canvasElement);
    await userEvent.click(
      portal.getByRole("button", { name: "Apply command" }),
    );
    const request = (await portal.findByText(/^Request:/)).textContent;
    await userEvent.click(
      portal.getByRole("button", { name: "Retry original command" }),
    );
    await expect(
      await canvas.findByRole("button", {
        name: "Retained commit draft",
      }),
    ).toBeVisible();
    await expect(
      canvas.getAllByRole("button", {
        name: "Retained commit draft",
      }),
    ).toHaveLength(1);
    await expect(request).toMatch(/^Request: [a-f0-9-]+$/);
  }}
/>

<Story
  name="Stale preview retains draft"
  args={{ fault: "stale" }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await createPreview(canvasElement);
    await userEvent.click(
      portal.getByRole("button", { name: "Apply command" }),
    );
    await expect(
      await portal.findByText(/The workspace changed/),
    ).toBeVisible();
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Refresh" }));
    await waitFor(() =>
      expect(canvas.getByRole("button", { name: "Refresh" })).toBeEnabled(),
    );
    await expect(await canvas.findByLabelText("Commit title")).toHaveValue(
      "Retained commit draft",
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Create commit" }),
    );
    await applyPreview(canvasElement);
  }}
/>

<Story
  name="Failure and retry"
  args={{ fault: "failed" }}
  play={async ({ canvasElement }) => {
    const portal = workspacePortal(canvasElement);
    await createPreview(canvasElement);
    await userEvent.click(
      portal.getByRole("button", { name: "Apply command" }),
    );
    await expect(
      await portal.findByText("Workspace command failed"),
    ).toBeVisible();
    await userEvent.click(
      portal.getByRole("button", { name: "Apply command" }),
    );
    await expect(
      await workspaceCanvas(canvasElement).findByRole("button", {
        name: "Retained commit draft",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Invalid names stay beside fields"
  play={async ({ canvasElement }) => {
    const portal = workspacePortal(canvasElement);
    await commandMenu(canvasElement, "Workspace actions", "Create workspace");
    await userEvent.type(await portal.findByLabelText("Name"), "Daily work");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await expect(
      await portal.findByText("This workspace name is already in use."),
    ).toBeVisible();
    await expect(portal.getByLabelText("Name")).toHaveValue("Daily work");
  }}
/>
