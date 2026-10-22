<script module lang="ts">
  import { views as applicationViews } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceConflictApplicationFixture from "../../fixtures/StackWorkspaceConflictApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Conflict presentation",
    component: StackWorkspaceConflictApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await Promise.all([
        applicationViews.stacks(),
        applicationViews.files(),
        applicationViews.log(),
        applicationViews.stackDetails(),
        applicationViews.stackConflicts(),
      ]);
    },
  });
  import {
    rail,
    closeEditor,
    loadingConflict,
  } from "./conflict-story-actions.js";
</script>

<Story
  name="Workspace conflict surface has the shared browser and editable panes"
  args={{ multiple: true }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await within(canvasElement).findByRole("region", {
      name: "Unstaged files",
    });
    await userEvent.click(
      within(canvasElement).getByRole("button", { name: "Resolve conflicts" }),
    );
    const surface = await page.findByRole("region", {
      name: "Resolve workspace conflicts",
    });
    const view = within(surface);
    expect(
      page.getByRole("combobox", { name: "Active stack workspace" }),
    ).toBeDisabled();
    const browser = view.getByRole("complementary", {
      name: "Conflict file browser",
    });
    expect(
      browser.querySelector('[data-ui-component="file-listing"]'),
    ).not.toBeNull();
    const editor = view.getByRole("textbox", { name: "Edit Resolved" });
    await userEvent.clear(editor);
    await userEvent.type(editor, "// retained workspace merge\n");
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    expect(
      view.getByRole("button", { name: "Preview resolution" }),
    ).toBeEnabled();
    await userEvent.click(
      within(browser).getByRole("button", {
        name: "output.ts Repository file",
      }),
    );
    await userEvent.click(
      within(browser).getByRole("button", {
        name: "settings.ts Repository file",
      }),
    );
    expect(editor).toHaveValue("// retained workspace merge\n");
    await userEvent.click(
      view.getByRole("button", { name: "Hide conflict file browser" }),
    );
    expect(browser).not.toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "Close conflict editor" }),
    );
    await waitFor(() => expect(surface).not.toBeInTheDocument());
  }}
/>
