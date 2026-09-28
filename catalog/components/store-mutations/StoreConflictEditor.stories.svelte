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

  async function dialogFor(canvasElement: HTMLElement) {
    const page = within(canvasElement.ownerDocument.body);
    const dialog = await page.findByRole("dialog", {
      name: "Resolve native conflicts",
    });
    return { page, dialog, view: within(dialog) };
  }

  async function chooseTerms(view: ReturnType<typeof within>) {
    await userEvent.click(view.getByRole("button", { name: "Use native" }));
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
    const { page, dialog, view } = await dialogFor(canvasElement);
    await expect(view.getByTitle(conflicts[0]!.fingerprint)).toBeVisible();
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
      '[data-ui-part="dialog-frame-footer"]',
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
    const { view } = await dialogFor(canvasElement);
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
    const { view } = await dialogFor(canvasElement);
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
    const { view } = await dialogFor(canvasElement);
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
