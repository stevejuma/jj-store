<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceApplicationFixture from "../../fixtures/StackWorkspaceApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Settings",
    component: StackWorkspaceApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function openSettings(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      await c.findByRole("button", { name: "Workspace actions" }),
    );
    await userEvent.click(
      await portal.findByRole("menuitem", { name: "Settings" }),
    );
    return c.findByRole("complementary", { name: "Workspace settings" });
  }
</script>

<Story
  name="Edit workspace settings with retained draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    const panel = await openSettings(canvasElement),
      settings = within(panel);
    expect(panel).toHaveClass("community-application__standard-right-sidebar");
    expect(
      panel.querySelector(".lc-channel-settings__image [data-workspace-icon]"),
    ).not.toBeNull();
    const name = settings.getByLabelText("Workspace name");
    expect(name.closest(".community-settings__option-group")).not.toBeNull();
    await userEvent.clear(name);
    await userEvent.type(name, "Focused work");
    await userEvent.click(
      settings.getByRole("combobox", { name: "Workspace target branch" }),
    );
    await userEvent.click(
      await portal.findByRole("option", { name: /feature\/api/ }),
    );
    await userEvent.click(
      settings.getByRole("button", { name: "Save settings" }),
    );
    expect(await portal.findByText(/Focused work.*feature\/api/)).toBeVisible();
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    expect(name).toHaveValue("Focused work");
    await userEvent.click(
      settings.getByRole("button", { name: "Save settings" }),
    );
    await userEvent.click(
      await portal.findByRole("button", { name: "Apply command" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("combobox", { name: "Active stack workspace" }),
      ).toHaveTextContent("Focused work"),
    );
    expect(c.getByRole("region", { name: "Unstaged files" })).toBeVisible();
    await userEvent.click(
      settings.getByRole("button", { name: "Close workspace settings" }),
    );
    await waitFor(() =>
      expect(
        c.queryByRole("complementary", { name: "Workspace settings" }),
      ).toBeNull(),
    );
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Workspace actions" }),
      ).toHaveFocus(),
    );
    await openSettings(canvasElement);
    expect(c.getByLabelText("Workspace name")).toHaveValue("Focused work");
  }}
/>
<Story
  name="Delete workspace preserves branches and requires confirmation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    const panel = await openSettings(canvasElement),
      settings = within(panel);
    await userEvent.click(
      settings.getByRole("button", { name: "Delete workspace" }),
    );
    expect(
      await portal.findByText(
        /Branches, commits.*unfinished JJ work are retained/,
      ),
    ).toBeVisible();
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    expect(
      c.getByRole("combobox", { name: "Active stack workspace" }),
    ).toHaveTextContent("Daily work");
    await userEvent.click(
      settings.getByRole("button", { name: "Delete workspace" }),
    );
    const confirm = within(await portal.findByRole("dialog")).getByRole(
      "button",
      { name: "Delete workspace" },
    );
    expect(confirm).toHaveAttribute("data-variant", "destructive");
    await userEvent.click(confirm);
    await c.findByText("No stack workspace", {
      selector: '[data-ui-part="empty-title"]',
    });
    await userEvent.click(c.getByRole("tab", { name: "Branches" }));
    expect(
      await c.findByRole("option", { name: /feature\/api/ }),
    ).toBeVisible();
    await waitFor(() =>
      expect(
        c.queryByRole("complementary", { name: "Workspace settings" }),
      ).toBeNull(),
    );
  }}
/>

<Story
  name="Settings validation and cancel"
  play={async ({ canvasElement }) => {
    const settings = within(await openSettings(canvasElement));
    const name = settings.getByLabelText("Workspace name");
    await userEvent.clear(name);
    expect(
      settings.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    expect(settings.getByRole("alert")).toHaveTextContent(
      "Enter a workspace name",
    );
    await userEvent.type(name, "Discard this draft");
    await userEvent.click(settings.getByRole("button", { name: "Cancel" }));
    expect(name).toHaveValue("Daily work");
    expect(
      settings.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Unavailable settings stay read only"
  args={{ writable: false }}
  play={async ({ canvasElement }) => {
    const settings = within(await openSettings(canvasElement));
    expect(settings.getByLabelText("Workspace name")).toBeDisabled();
    expect(
      settings.getByRole("combobox", { name: "Workspace target branch" }),
    ).toBeDisabled();
    expect(
      settings.getByRole("button", { name: "Delete workspace" }),
    ).toBeDisabled();
  }}
/>
