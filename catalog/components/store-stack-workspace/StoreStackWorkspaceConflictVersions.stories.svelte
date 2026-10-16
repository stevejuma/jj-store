<script module lang="ts">
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StackWorkspaceConflictApplicationFixture from "../../fixtures/StackWorkspaceConflictApplicationFixture.svelte";
  import { mergeEditorSettingsKey } from "../../../web/src/lib/ui/store-conflicts/merge-editor-settings.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Conflict versions",
    component: StackWorkspaceConflictApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await Promise.all([
        views.stacks(),
        views.files(),
        views.log(),
        views.stackDetails(),
        views.stackConflicts(),
      ]);
      const prior = localStorage.getItem(mergeEditorSettingsKey);
      localStorage.removeItem(mergeEditorSettingsKey);
      return () => {
        if (prior === null) localStorage.removeItem(mergeEditorSettingsKey);
        else localStorage.setItem(mergeEditorSettingsKey, prior);
      };
    },
  });
</script>

<Story
  name="Focused Op Log uses the same version picker and merge settings"
  args={{ multisided: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await canvas.findByRole("region", { name: "Unstaged files" });
    await userEvent.click(canvas.getByRole("tab", { name: "Op Log" }));
    expect(canvas.getByRole("tab", { name: "Op Log" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    const notices = page.getByRole("region", { name: "Notifications" });
    await userEvent.click(
      await within(notices).findByRole("button", {
        name: "Resolve conflicts",
      }),
    );
    const surface = await page.findByRole("region", {
      name: "Resolve workspace conflicts",
    });
    const view = within(surface);
    const picker = view.getByRole("combobox", { name: "Comparison version" });
    expect(picker.closest(".ui-diff-merge-editor__toolbar")).not.toBeNull();
    expect(picker.getBoundingClientRect().height).toBe(24);
    await userEvent.click(
      view.getByRole("button", { name: "Merge editor settings" }),
    );
    for (const name of [
      "Wrap lines",
      "Sync horizontal scroll",
      "Ignore whitespace",
      "Ignore case",
    ])
      await waitFor(async () => {
        expect(page.getByRole("menuitemcheckbox", { name })).toBeVisible();
      });
    await userEvent.keyboard("{Escape}");
    expect(view.getByRole("textbox", { name: "Edit Resolved" })).toBeVisible();
  }}
/>

<Story
  name="Merge header settings persist and preserve the resolution draft"
  args={{ multisided: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await canvas.findByRole("region", { name: "Unstaged files" });
    await userEvent.click(
      canvas.getByRole("button", { name: "Workspace actions" }),
    );
    const standardMenuItem = page.getByRole("menuitem", {
      name: "Workspace settings",
    });
    const fontFields = [
      "fontFamily",
      "fontSize",
      "fontWeight",
      "lineHeight",
    ] as const;
    const standardFont = fontFields.map(
      (field) => getComputedStyle(standardMenuItem)[field],
    );
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve conflicts" }),
    );
    let surface = await page.findByRole("region", {
      name: "Resolve workspace conflicts",
    });
    let trigger = within(surface).getByRole("button", {
      name: "Merge editor settings",
    });
    expect(trigger.closest(".ui-diff-merge-editor__nav")).not.toBeNull();
    await userEvent.click(trigger);
    const settingsItem = page.getByRole("menuitemcheckbox", {
      name: "Wrap lines",
    });
    expect(
      fontFields.map((field) => getComputedStyle(settingsItem)[field]),
    ).toEqual(standardFont);
    expect(page.queryByText("Merge editor", { exact: true })).toBeNull();
    expect(
      page
        .getByRole("menuitemcheckbox", { name: "Sync horizontal scroll" })
        .getBoundingClientRect().height,
    ).toBeLessThanOrEqual(32);
    expect(
      page.getByRole("menuitemcheckbox", { name: "Sync horizontal scroll" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Wrap lines" }),
    );
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Sync horizontal scroll" }),
    );
    await userEvent.keyboard("{Escape}");
    expect(surface).toBeVisible();
    expect(trigger).toHaveFocus();
    expect(JSON.parse(localStorage.getItem(mergeEditorSettingsKey)!)).toEqual({
      version: 1,
      wrap: true,
      syncHorizontalScroll: false,
      ignoreWhitespace: false,
      ignoreCase: false,
    });
    await userEvent.click(
      within(surface).getByRole("button", { name: "Close conflict editor" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Resolve conflicts" }),
    );
    surface = await page.findByRole("region", {
      name: "Resolve workspace conflicts",
    });
    const editor = within(surface).getByRole("textbox", {
      name: "Edit Resolved",
    });
    expect(
      editor.closest("[data-ui-component='merge-editor']"),
    ).toHaveAttribute("data-wrap", "true");
    await userEvent.clear(editor);
    await userEvent.type(editor, "retained resolution\n");
    trigger = within(surface).getByRole("button", {
      name: "Merge editor settings",
    });
    await userEvent.click(trigger);
    expect(
      page.getByRole("menuitemcheckbox", { name: "Wrap lines" }),
    ).toHaveAttribute("aria-checked", "true");
    expect(
      page.getByRole("menuitemcheckbox", { name: "Sync horizontal scroll" }),
    ).toHaveAttribute("aria-checked", "false");
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Wrap lines" }),
    );
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Ignore whitespace" }),
    );
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Ignore case" }),
    );
    await userEvent.keyboard("{Escape}");
    expect(editor).toHaveValue("retained resolution\n");
    expect(
      within(surface).getByRole("button", { name: "Preview resolution" }),
    ).toBeDisabled();
    expect(JSON.parse(localStorage.getItem(mergeEditorSettingsKey)!)).toEqual({
      version: 1,
      wrap: false,
      syncHorizontalScroll: false,
      ignoreWhitespace: true,
      ignoreCase: true,
    });
  }}
/>

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
    const layer = surface.closest('[data-ui-part="surface-layer"]')!;
    expect(
      Math.abs(
        surface.getBoundingClientRect().right -
          layer.getBoundingClientRect().right,
      ),
    ).toBeLessThanOrEqual(1);
    const view = within(surface);
    const editor = view.getByRole("textbox", { name: "Edit Resolved" });
    expect(editor).toBeVisible();
    expect(view.queryByText(/Compare each native version/)).toBeNull();
    const trigger = view.getByRole("combobox", { name: "Comparison version" });
    const toolbar = trigger.closest(".ui-diff-merge-editor__toolbar");
    expect(toolbar).not.toBeNull();
    const header = toolbar!.getBoundingClientRect();
    const button = trigger.getBoundingClientRect();
    const title = toolbar!
      .querySelector(".ui-diff-merge-editor__title")!
      .getBoundingClientRect();
    const pager = toolbar!
      .querySelector(".ui-diff-merge-editor__nav")!
      .getBoundingClientRect();
    expect(button.height).toBeLessThanOrEqual(24);
    expect(
      Math.abs(
        button.left + button.width / 2 - (header.left + header.width / 2),
      ),
    ).toBeLessThanOrEqual(1);
    expect(title.right).toBeLessThanOrEqual(button.left);
    expect(button.right).toBeLessThanOrEqual(pager.left);
    expect(
      Math.abs(button.top + button.height / 2 - (pager.top + pager.height / 2)),
    ).toBeLessThanOrEqual(1);
    await userEvent.click(trigger);
    const firstPicker = await page.findByRole("dialog", {
      name: "Choose conflict version",
    });
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(firstPicker).not.toBeInTheDocument());
    expect(editor).toBeVisible();
    expect(trigger).toHaveFocus();
    await userEvent.click(trigger);
    const picker = await page.findByRole("dialog", {
      name: "Choose conflict version",
    });
    await waitFor(async () => {
      expect(
        within(picker).getByText("1 line · 6 bytes · third"),
      ).toBeVisible();
    });
    await userEvent.type(
      within(picker).getByRole("combobox", {
        name: "Search conflict versions",
      }),
      "third",
    );
    await userEvent.keyboard("{Enter}");
    await waitFor(() => expect(picker).not.toBeInTheDocument());
    expect(trigger).toHaveTextContent("Version 3");
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
    await userEvent.click(
      view.getByRole("combobox", { name: "Comparison version" }),
    );
    const picker = await page.findByRole("dialog", {
      name: "Choose conflict version",
    });
    expect(
      within(picker).getByRole("option", { name: /Version 3/ }),
    ).toHaveAttribute("aria-disabled", "true");
    await waitFor(async () => {
      expect(
        within(picker).getByText(
          "Deleted version · unavailable for text comparison",
        ),
      ).toBeVisible();
    });
    expect(
      view.getByRole("button", { name: "Preview resolution" }),
    ).toBeDisabled();
    await userEvent.type(
      within(picker).getByRole("combobox", {
        name: "Search conflict versions",
      }),
      "left",
    );
    await userEvent.keyboard("{Enter}");
    await waitFor(() => expect(picker).not.toBeInTheDocument());
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
