<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceConflictApplicationFixture from "../../fixtures/StackWorkspaceConflictApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Conflict versions",
    component: StackWorkspaceConflictApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Multi-sided native versions resolve in app"
  args={{ multisided: true }}
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
    const editor = view.getByRole("textbox", { name: "Edit Resolved" });
    expect(editor).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "Compare version 3" }),
    );
    await userEvent.clear(editor);
    await userEvent.type(editor, "reviewed third\n");
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    await userEvent.click(
      view.getByRole("button", { name: "Preview resolution" }),
    );
    const preview = await page.findByRole("dialog");
    await waitFor(() =>
      expect(preview.contains(canvasElement.ownerDocument.activeElement)).toBe(
        true,
      ),
    );
    await userEvent.click(
      await page.findByRole("button", { name: "Apply command" }),
    );
    await waitFor(() =>
      expect(
        within(canvasElement).queryByText(
          /^Workspace has unresolved conflicts/,
        ),
      ).toBeNull(),
    );
  }}
/>

<Story
  name="Deleted native versions are never selected as empty files"
  args={{ deletedVersion: true }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await within(canvasElement).findByRole("region", {
      name: "Unstaged files",
    });
    await userEvent.click(
      within(canvasElement).getByRole("button", { name: "Resolve conflicts" }),
    );
    const view = within(
      await page.findByRole("region", { name: "Resolve workspace conflicts" }),
    );
    expect(
      view.getByRole("button", { name: "Compare version 3" }),
    ).toBeDisabled();
    expect(
      view.getByRole("button", { name: "Preview resolution" }),
    ).toBeDisabled();
    await userEvent.click(
      view.getByRole("button", { name: "Compare version 1" }),
    );
    expect(view.getByRole("textbox", { name: "Edit Resolved" })).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    expect(
      view.getByRole("button", { name: "Preview resolution" }),
    ).toBeEnabled();
    await userEvent.click(
      view.getByRole("button", { name: "Close conflict editor" }),
    );
  }}
/>
<Story
  name="Unsupported native conflicts retain their reason and cannot preview"
  args={{ unsupported: true }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await within(canvasElement).findByRole("region", {
      name: "Unstaged files",
    });
    await userEvent.click(
      within(canvasElement).getByRole("button", { name: "Resolve conflicts" }),
    );
    const view = within(
      await page.findByRole("region", { name: "Resolve workspace conflicts" }),
    );
    expect(
      view.getByText(
        "This structural or mode conflict cannot be resolved as text.",
      ),
    ).toBeVisible();
    expect(view.queryByRole("textbox", { name: "Edit Resolved" })).toBeNull();
    expect(
      view.getByRole("button", { name: "Stage merged file" }),
    ).toBeDisabled();
    expect(
      view.getByRole("button", { name: "Preview resolution" }),
    ).toBeDisabled();
    await userEvent.click(
      view.getByRole("button", { name: "Close conflict editor" }),
    );
  }}
/>
