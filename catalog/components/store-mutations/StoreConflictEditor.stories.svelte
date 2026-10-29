<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreConflictEditor from "../../../web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte";
  import StoreConflictEditorFixture from "../../fixtures/StoreConflictEditorFixture.svelte";
  import { conflicts } from "../../fixtures/conflicts.js";
  import { mergeEditorSettingsKey } from "../../../web/src/lib/ui/store-conflicts/merge-editor-settings.js";

  const { Story } = defineMeta({
    parameters: { jjStoreStory: { kind: "application" } },
    title: "JJ Store/Mutations/Conflict editor",
    component: StoreConflictEditor,
    tags: ["visual-pending", "application"],
    beforeEach: () => {
      const prior = localStorage.getItem(mergeEditorSettingsKey);
      localStorage.removeItem(mergeEditorSettingsKey);
      return () => {
        if (prior === null) localStorage.removeItem(mergeEditorSettingsKey);
        else localStorage.setItem(mergeEditorSettingsKey, prior);
      };
    },
  });

  async function sheetFor(canvasElement: HTMLElement) {
    const page = within(canvasElement.ownerDocument.body);
    const menu = await page.findByRole("button", {
      name: "Application menu",
    });
    await userEvent.click(menu);
    const resolve = await page.findByRole("menuitem", {
      name: "Resolve conflicts",
    });
    await userEvent.click(resolve);
    const dialog = await page.findByRole(
      "region",
      { name: "Resolve native conflicts" },
      { timeout: 10_000 },
    );
    await waitFor(() => {
      expect(dialog).toBeVisible();
    });
    return { page, dialog, view: within(dialog) };
  }

  async function chooseTerms(view: ReturnType<typeof within>) {
    await userEvent.click(
      view.getByRole("button", {
        name: "Accept All Incoming Changes from Left",
      }),
    );
    await userEvent.click(
      view.getByRole("button", { name: "logo.bin Repository file" }),
    );
    await userEvent.click(
      view.getByRole("button", { name: "Use publication" }),
    );
  }
</script>

<Story
  name="Native editor shares merge header settings and retains filtered drafts"
  play={async ({ canvasElement }) => {
    const { page, dialog, view } = await sheetFor(canvasElement);
    const editor = view.getByRole("textbox", { name: "Edit Resolved" });
    await userEvent.clear(editor);
    await userEvent.type(editor, "// Op Log retained draft\n");
    const trigger = view.getByRole("button", { name: "Merge editor settings" });
    expect(trigger.closest(".ui-diff-merge-editor__nav")).not.toBeNull();
    await userEvent.click(trigger);
    for (const name of [
      "Wrap lines",
      "Sync horizontal scroll",
      "Ignore whitespace",
      "Ignore case",
    ]) {
      await waitFor(async () => {
        expect(page.getByRole("menuitemcheckbox", { name })).toBeVisible();
      });
    }
    expect(
      page.getByRole("menuitemcheckbox", { name: "Sync horizontal scroll" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Ignore whitespace" }),
    );
    await userEvent.click(
      page.getByRole("menuitemcheckbox", { name: "Ignore case" }),
    );
    await userEvent.keyboard("{Escape}");
    await waitFor(async () => {
      expect(dialog).toBeVisible();
    });
    expect(trigger).toHaveFocus();
    expect(editor).toHaveValue("// Op Log retained draft\n");
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    expect(view.getByRole("button", { name: "Preview 1/2" })).toBeEnabled();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>

<Story
  name="Atomic term resolution"
  play={async ({ canvasElement }) => {
    const { page, dialog, view } = await sheetFor(canvasElement);
    await chooseTerms(view);
    await expect(
      view.getByText(/native descriptor reports binary/u),
    ).toBeVisible();
    await userEvent.click(view.getByRole("button", { name: "Preview 2/2" }));
    await expect(view.getByText("Preview prepared")).toBeVisible();
    await expect(view.getByText("src/lib.rs, assets/logo.bin")).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "Apply resolution" }),
    );
    const confirmation = page.getByRole("alertdialog", {
      name: "Apply atomic conflict resolution?",
    });
    await userEvent.click(
      within(confirmation).getByRole("button", { name: "Apply resolution" }),
    );
    await expect(
      within(canvasElement.ownerDocument.body).getByText("Conflicts resolved"),
    ).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "Close conflict editor" }),
    );
    await waitFor(() => expect(dialog).not.toBeInTheDocument());
    await expect(
      getComputedStyle(canvasElement.ownerDocument.body).pointerEvents,
    ).not.toBe("none");
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>

<Story
  name="Header actions and file browser"
  play={async ({ canvasElement }) => {
    const { dialog, view } = await sheetFor(canvasElement);
    const sidebar = view.getByRole("complementary", {
      name: "Conflict file browser",
    });
    const header = dialog.querySelector<HTMLElement>(
      '[data-ui-part="surface-layer-header"]',
    )!;
    const body = dialog.querySelector<HTMLElement>(
      '[data-ui-part="surface-layer-body"]',
    )!;
    await expect(getComputedStyle(header).borderBottomWidth).toBe("1px");
    await expect(sidebar).toBeVisible();
    await expect(
      sidebar.querySelector('[data-ui-component="file-listing"]'),
    ).toBeInTheDocument();
    await expect(getComputedStyle(sidebar).borderRightWidth).toBe("1px");
    await expect(
      Math.abs(
        sidebar.getBoundingClientRect().height -
          body.getBoundingClientRect().height,
      ),
    ).toBeLessThan(1);
    await expect(
      view.queryByRole("button", { name: "Cancel" }),
    ).not.toBeInTheDocument();
    await expect(
      dialog.querySelector(".store-conflict-editor__footer"),
    ).not.toBeInTheDocument();
    for (const name of ["Delete path", "Stage merged file", "Preview 0/2"]) {
      const button = within(header).getByRole("button", { name });
      await expect(button).toBeVisible();
      await expect(button.querySelector("svg")).toBeInTheDocument();
    }
    await userEvent.click(
      view.getByRole("button", { name: "Hide conflict file browser" }),
    );
    await expect(sidebar).not.toBeVisible();
    const editor = view.getByRole("textbox", { name: "Edit Resolved" });
    await userEvent.clear(editor);
    await userEvent.type(editor, "// retained merge draft\n");
    await userEvent.click(
      view.getByRole("button", { name: "Show conflict file browser" }),
    );
    await expect(sidebar).toBeVisible();
    await expect(editor).toHaveValue("// retained merge draft\n");
    await userEvent.click(
      within(sidebar).getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(
      await within(sidebar).findByRole("button", {
        name: "assets/logo.bin Repository file",
      }),
    );
    await expect(
      view.queryByRole("button", { name: "Stage merged file" }),
    ).not.toBeInTheDocument();
    await expect(
      within(header).getByRole("button", { name: "Delete path" }),
    ).toBeVisible();
    await userEvent.click(
      await within(sidebar).findByRole("button", {
        name: "src/lib.rs Repository file",
      }),
    );
    await expect(editor).toHaveValue("// retained merge draft\n");
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>

<Story
  name="Staged merged file"
  play={async ({ canvasElement }) => {
    const { view } = await sheetFor(canvasElement);
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    await expect(view.getByText(/Merged file staged/u)).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "logo.bin Repository file" }),
    );
    await userEvent.click(view.getByRole("button", { name: "Delete path" }));
    await userEvent.click(view.getByRole("button", { name: "Preview 2/2" }));
    await expect(view.getByText("Preview prepared")).toBeVisible();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>

<Story
  name="Unknown outcome recovery"
  play={async ({ canvasElement }) => {
    const { view } = await sheetFor(canvasElement);
    await chooseTerms(view);
    await userEvent.click(view.getByRole("button", { name: "Preview 2/2" }));
    await userEvent.click(
      view.getByRole("button", { name: "Apply resolution" }),
    );
    const confirmation = within(canvasElement.ownerDocument.body).getByRole(
      "alertdialog",
      { name: "Apply atomic conflict resolution?" },
    );
    await userEvent.click(
      within(confirmation).getByRole("button", { name: "Apply resolution" }),
    );
    await expect(view.getByText("Publication outcome unknown")).toBeVisible();
    await userEvent.click(view.getByRole("button", { name: "Check status" }));
    await expect(
      within(canvasElement.ownerDocument.body).getByText("Conflicts resolved"),
    ).toBeVisible();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture
      outcome="unknown"
    />{/snippet}
</Story>

<Story
  name="Stale fingerprint"
  play={async ({ canvasElement }) => {
    const { view } = await sheetFor(canvasElement);
    await chooseTerms(view);
    await userEvent.click(view.getByRole("button", { name: "Preview 2/2" }));
    await expect(
      view.getByText("The conflict fingerprint is stale."),
    ).toBeVisible();
    await expect(
      view.getByRole("button", { name: "Preview 2/2" }),
    ).toBeVisible();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture
      failure="The conflict fingerprint is stale."
    />{/snippet}
</Story>

<Story
  name="Shell merge surface"
  play={async ({ canvasElement }) => {
    const { page, dialog, view } = await sheetFor(canvasElement);
    const main = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="main"]',
    )!;
    await expect(main).toHaveAttribute("inert");
    const left = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="sidebar"][data-side="left"]',
    )!;
    await expect(left).toHaveAttribute("inert");
    await expect(
      page.getByRole("button", { name: "Application menu" }),
    ).toBeDisabled();
    await expect(
      page.getByRole("button", { name: "Refresh snapshot" }),
    ).toBeDisabled();
    await expect(
      view.queryByTitle(conflicts[0]!.fingerprint),
    ).not.toBeInTheDocument();
    await expect(
      view.queryByRole("button", { name: "Use native" }),
    ).not.toBeInTheDocument();
    const editor = dialog.querySelector<HTMLElement>(
      '[data-ui-component="merge-editor"]',
    )!;
    await waitFor(() =>
      expect(editor.getBoundingClientRect().height).toBeGreaterThan(
        dialog.getBoundingClientRect().height * 0.7,
      ),
    );
    for (const side of ["left", "base", "right"]) {
      const pane = editor.querySelector<HTMLElement>(
        `[data-ui-part="merge-view"][data-merge-side="${side}"]`,
      )!;
      await expect(
        pane.querySelector(".ui-code-token-keyword"),
      ).toBeInTheDocument();
    }
    await userEvent.click(
      view.getByRole("button", {
        name: "Accept All Incoming Changes from Right",
      }),
    );
    await expect(
      view.getByRole("button", { name: "Preview 1/2" }),
    ).toBeVisible();
    await expect(
      view.getByRole("textbox", { name: "Edit Resolved" }),
    ).toHaveValue(
      conflicts[0]!.editor.status === "text"
        ? conflicts[0]!.editor.contents[2]
        : "",
    );
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(dialog).not.toBeInTheDocument());
    await expect(main).not.toHaveAttribute("inert");
    await expect(left).not.toHaveAttribute("inert");
    await expect(
      page.getByRole("button", { name: "Application menu" }),
    ).toHaveFocus();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>
