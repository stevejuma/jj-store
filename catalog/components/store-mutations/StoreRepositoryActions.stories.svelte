<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryActions from "../../../web/src/lib/ui/store-mutations/StoreRepositoryActions.svelte";
  import StoreRepositoryActionsFixture from "../../fixtures/StoreRepositoryActionsFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Mutations/Repository actions",
    component: StoreRepositoryActions,
    tags: ["visual-pending"],
  });

  async function waitUntilInteractive(document: Document) {
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  }

  async function closeDialog(
    page: ReturnType<typeof within>,
    dialog: HTMLElement,
    name: string,
  ) {
    const footer = dialog.querySelector<HTMLElement>(
      '[data-ui-part="dialog-frame-footer"]',
    )!;
    await userEvent.click(
      within(footer).getByRole("button", { name: "Close" }),
    );
    await waitFor(() =>
      expect(page.queryByRole("dialog", { name })).not.toBeInTheDocument(),
    );
    await waitUntilInteractive(dialog.ownerDocument);
  }
</script>

<Story
  name="Preview and apply"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await waitUntilInteractive(canvasElement.ownerDocument);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository actions" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Create change" }),
    );
    const dialog = await page.findByRole("dialog", { name: "Create change" });
    await userEvent.type(
      within(dialog).getByLabelText("Description"),
      "Create the browser mutation workflow",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview change" }),
    );
    await expect(within(dialog).getByText("Preview prepared")).toBeVisible();
    await expect(within(dialog).getByText("8192")).toBeVisible();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    await expect(within(dialog).getByText("Repository updated")).toBeVisible();
    await expect(canvas.getByTestId("repository-state")).toHaveTextContent(
      `${"ef".repeat(64)} ${"cd".repeat(64)}`,
    );
    await closeDialog(page, dialog, "Create change");
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture />{/snippet}
</Story>

<Story
  name="Destructive confirmation"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await waitUntilInteractive(canvasElement.ownerDocument);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository actions" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Delete bookmark" }),
    );
    const dialog = await page.findByRole("dialog", {
      name: "Delete bookmark",
    });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview deletion" }),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    const confirmation = page.getByRole("alertdialog", {
      name: "Delete bookmark?",
    });
    await expect(confirmation).toBeVisible();
    await userEvent.click(
      within(confirmation).getByRole("button", { name: "Apply command" }),
    );
    await expect(within(dialog).getByText("Repository updated")).toBeVisible();
    await expect(canvas.getByTestId("repository-state")).not.toHaveTextContent(
      /\bmain\b/u,
    );
    await closeDialog(page, dialog, "Delete bookmark");
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture />{/snippet}
</Story>

<Story
  name="Unknown outcome recovery"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await waitUntilInteractive(canvasElement.ownerDocument);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository actions" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Create change" }),
    );
    const dialog = await page.findByRole("dialog", { name: "Create change" });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview change" }),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    await expect(
      within(dialog).getByText("Publication outcome unknown"),
    ).toBeVisible();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Check status" }),
    );
    await expect(within(dialog).getByText("Repository updated")).toBeVisible();
    await closeDialog(page, dialog, "Create change");
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture
      outcome="unknown"
    />{/snippet}
</Story>
