<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import {
    workingCopies as fixtureCopies,
    IDS,
    revisions,
  } from "../../fixtures/repository.js";
  const workingCopies = [
    ...fixtureCopies,
    { ...fixtureCopies[0]!, workspace: "agent/review" },
    {
      ...fixtureCopies[0]!,
      workspace: "feature/native objects 日本語",
      commitId: IDS.left,
      changeId: revisions.find((item) => item.eventId === IDS.left)!.changeId,
    },
  ];
  const { Story } = defineMeta({
    title: "JJ Store/Workspaces",
    component: StoreRepositoryApplication,
    tags: ["visual-pending", "application"],
    args: { initialPage: "files", initiallyCollapsed: false, workingCopies },
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
  async function choose(canvasElement: HTMLElement, workspace: string) {
    const canvas = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    );
    const search = await portal.findByLabelText("Search workspaces");
    await userEvent.type(search, workspace);
    await userEvent.click(
      await portal.findByRole("option", {
        name: new RegExp(workspace.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
      }),
    );
    await waitFor(() =>
      expect(
        canvas.getByRole("combobox", { name: "Repository workspace" }),
      ).toHaveTextContent(workspace),
    );
  }
</script>

<Story
  name="Picker and tab history"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const header = canvas.getByLabelText("Workspace section header");
    const tabs = canvas.getByRole("tablist", { name: "Repository sections" });
    await expect(header.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      tabs.getBoundingClientRect().bottom,
    );
    await expect(
      within(header).getByRole("heading", { name: "Workspace", level: 1 }),
    ).toBeVisible();
    await choose(canvasElement, "agent/review");
    await expect(
      await canvas.findByRole("combobox", {
        name: "Repository bookmark or change",
      }),
    ).toHaveTextContent("Working copy · agent/review");
    await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("agent/review");
    await choose(canvasElement, "default");
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("agent/review");
    await userEvent.click(canvas.getByRole("button", { name: "Forward" }));
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("default");
  }}
/>

<Story
  name="Keyboard search and repository overview"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    );
    const search = await portal.findByLabelText("Search workspaces");
    await userEvent.type(search, "no-such-workspace");
    await expect(
      await portal.findByText("No matching workspaces."),
    ).toBeVisible();
    await userEvent.clear(search);
    await userEvent.type(search, "agent/review");
    await portal.findByRole("option", { name: /agent\/review/ });
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await waitFor(() =>
      expect(
        canvas.getByRole("combobox", { name: "Repository workspace" }),
      ).toHaveTextContent("agent/review"),
    );
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    );
    await userEvent.click(
      await portal.findByRole("option", { name: /Repository overview/ }),
    );
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("Repository overview");
  }}
/>

<Story
  name="Collapsed sidebar"
  args={{ initiallyCollapsed: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await choose(canvasElement, "agent/review");
    await choose(canvasElement, "default");
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("default");
  }}
/>

<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Mobile section header"
  args={{}}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await choose(canvasElement, "feature/native objects 日本語");
    const header = canvas.getByLabelText("Workspace section header");
    await expect(header.scrollWidth).toBeLessThanOrEqual(header.clientWidth);
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toBeVisible();
  }}
/>

<Story
  name="File editing retains the workspace"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Edit README" }));
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toBeDisabled();
    await userEvent.click(canvas.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(
        canvas.getByRole("combobox", { name: "Repository workspace" }),
      ).toBeEnabled(),
    );
  }}
/>
