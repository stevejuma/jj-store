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
  name="Menu layout"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository actions" }),
    );
    const menu = page.getByRole("menu", { name: "Repository actions" });
    await waitFor(() => expect(menu.getBoundingClientRect().width).toBe(256));
    for (const name of [
      "Create change",
      "Edit description",
      "Abandon change",
      "Resolve conflicts",
      "Create bookmark",
      "Move bookmark",
      "Delete bookmark",
    ]) {
      await waitFor(async () => {
        await expect(
          within(menu).getByRole("menuitem", { name }),
        ).toBeVisible();
      });
    }
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        page.queryByRole("menu", { name: "Repository actions" }),
      ).toBeNull(),
    );
    await waitUntilInteractive(canvasElement.ownerDocument);
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture />{/snippet}
</Story>

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
    await waitFor(() => expect(dialog).toBeVisible());
    await expect(
      within(dialog).queryByRole("textbox", { name: "Parent commit" }),
    ).not.toBeInTheDocument();
    const parentCommit = within(dialog).getByRole("combobox", {
      name: "Parent commit",
    });
    const parentDetails = dialog.ownerDocument.getElementById(
      parentCommit.getAttribute("aria-describedby")!,
    )!;
    await expect(parentCommit).toHaveAccessibleDescription(/Working copy/);
    await expect(parentDetails).toHaveTextContent("Working copy");
    await expect(parentDetails).toHaveTextContent("main");
    await userEvent.click(parentCommit);
    const search = page.getByRole("combobox", {
      name: "Select parent commit",
    });
    await userEvent.type(search, "Stream immutable native objects");
    await userEvent.click(
      page.getByRole("option", { name: /Stream immutable native objects/u }),
    );
    await expect(parentDetails).toHaveTextContent(
      "Stream immutable native objects",
    );
    await expect(parentDetails).toHaveTextContent("feature/native-objects");
    await userEvent.type(
      within(dialog).getByRole("textbox", { name: "Markdown editor" }),
      "Create the browser mutation workflow",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview change" }),
    );
    await waitFor(async () => {
      await expect(within(dialog).getByText("Preview prepared")).toBeVisible();
    });
    await waitFor(async () => {
      await expect(within(dialog).getByText("8192")).toBeVisible();
    });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Repository updated"),
      ).toBeVisible();
    });
    await expect(canvas.getByTestId("repository-state")).toHaveTextContent(
      `${"ef".repeat(64)} ${"cd".repeat(64)}`,
    );
    await closeDialog(page, dialog, "Create change");
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture />{/snippet}
</Story>

<Story
  name="Edit description context"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await waitUntilInteractive(canvasElement.ownerDocument);
    await userEvent.click(
      canvas.getByRole("button", { name: "Repository actions" }),
    );
    await userEvent.click(
      page.getByRole("menuitem", { name: "Edit description" }),
    );
    const dialog = await page.findByRole("dialog", {
      name: "Edit description",
    });
    await waitFor(() => expect(dialog).toBeVisible());
    const selectedCommit = within(dialog).getByRole("group", {
      name: "Selected commit",
    });
    await expect(
      within(selectedCommit).getByRole("img", { name: "maya@example.test" }),
    ).toBeVisible();
    await waitFor(() =>
      expect(selectedCommit).toHaveTextContent("Working copy"),
    );
    await expect(selectedCommit).toHaveTextContent("main");
    await expect(selectedCommit).toHaveTextContent("default");
    await expect(
      within(selectedCommit).getByRole("button", {
        name: /Copy JJ Change ID/u,
      }),
    ).toBeVisible();
    await expect(
      within(selectedCommit).getByRole("button", {
        name: /Copy Git commit SHA/u,
      }),
    ).toBeVisible();
    await expect(selectedCommit).toHaveTextContent(/seconds ago|just now/u);
    await expect(
      within(dialog).queryByRole("textbox", { name: "Commit" }),
    ).not.toBeInTheDocument();
    const description = within(dialog).getByRole("group", {
      name: "Description",
    });
    await expect(description).toHaveAttribute(
      "data-ui-component",
      "community-chat-composer",
    );
    await expect(
      within(description).getByRole("textbox", { name: "Markdown editor" }),
    ).toBeVisible();
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
    await waitFor(() => expect(confirmation).toBeVisible());
    await userEvent.click(
      within(confirmation).getByRole("button", { name: "Apply command" }),
    );
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Repository updated"),
      ).toBeVisible();
    });
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
    await waitFor(() => expect(dialog).toBeVisible());
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview change" }),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply command" }),
    );
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Publication outcome unknown"),
      ).toBeVisible();
    });
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Check status" }),
    );
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Repository updated"),
      ).toBeVisible();
    });
    await closeDialog(page, dialog, "Create change");
  }}
>
  {#snippet template()}<StoreRepositoryActionsFixture
      outcome="unknown"
    />{/snippet}
</Story>
