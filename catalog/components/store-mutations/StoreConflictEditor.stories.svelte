<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreConflictEditor from "../../../web/src/lib/ui/store-conflicts/StoreConflictEditor.svelte";
  import StoreConflictEditorFixture from "../../fixtures/StoreConflictEditorFixture.svelte";
  import { conflicts } from "../../fixtures/conflicts.js";

  const { Story } = defineMeta({
    title: "JJ Store/Mutations/Conflict editor",
    component: StoreConflictEditor,
    tags: ["visual-pending"],
  });

  async function sheetFor(canvasElement: HTMLElement) {
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      page.getByRole("button", { name: "Repository actions" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Resolve conflicts" }),
    );
    const dialog = await page.findByRole("region", {
      name: "Resolve native conflicts",
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
      view.getByRole("button", { name: "assets/logo.bin" }),
    );
    await userEvent.click(
      view.getByRole("button", { name: "Use publication" }),
    );
  }
</script>

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
    await expect(view.getByText("Conflicts resolved")).toBeVisible();
    const footer = dialog.querySelector<HTMLElement>(
      ".store-conflict-editor__footer",
    )!;
    await userEvent.click(
      within(footer).getByRole("button", { name: "Close" }),
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
  name="Staged merged file"
  play={async ({ canvasElement }) => {
    const { view } = await sheetFor(canvasElement);
    await userEvent.click(
      view.getByRole("button", { name: "Stage merged file" }),
    );
    await expect(view.getByText(/Merged file staged/u)).toBeVisible();
    await userEvent.click(
      view.getByRole("button", { name: "assets/logo.bin" }),
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
    await expect(view.getByText("Conflicts resolved")).toBeVisible();
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
      page.getByRole("button", { name: "Repository actions" }),
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
      page.getByRole("button", { name: "Repository actions" }),
    ).toHaveFocus();
  }}
>
  {#snippet template()}<StoreConflictEditorFixture />{/snippet}
</Story>
