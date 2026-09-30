<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { IDS } from "../../fixtures/repository.js";

  const { Story } = defineMeta({
    title: "JJ Store/Connected application/Branch context",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Actions"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/native-objects" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const details = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    const panel = within(details);
    await expect(panel.getByText("Branches/Bookmarks")).toBeVisible();
    await expect(
      panel.queryByText("Selected bookmark"),
    ).not.toBeInTheDocument();
    await expect(
      panel.getByRole("heading", { name: "feature/native-objects" }),
    ).toBeVisible();
    const bookmarkRevision = panel.getByRole("group", {
      name: "Bookmark revision",
    });
    await expect(
      within(bookmarkRevision).getByRole("img", {
        name: "maya@example.test",
      }),
    ).toBeVisible();
    await expect(
      within(bookmarkRevision).getByText("Stream immutable native objects"),
    ).toBeVisible();
    await expect(bookmarkRevision).toHaveTextContent("Maya Chen");
    await expect(bookmarkRevision).toHaveTextContent(/minutes ago/u);
    const status = panel.getByRole("group", { name: "Bookmark status" });
    await expect(status).toHaveTextContent("Status");
    await expect(status).toHaveTextContent("Local");
    const targets = panel.getByRole("group", { name: "Bookmark targets" });
    await expect(targets).toHaveTextContent("Targets");
    await expect(targets).toHaveTextContent("1");
    await expect(
      within(targets).getByRole("button", { name: /Copy Git commit SHA/u }),
    ).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Add bookmark" }),
    ).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Move bookmark" }),
    ).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Delete bookmark" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("option", { name: "conflicted" }));
    await expect(
      panel.getByRole("heading", { name: "conflicted" }),
    ).toBeVisible();
    await expect(
      panel.getByRole("group", { name: "Bookmark status" }),
    ).toHaveTextContent("Conflicted");
    const conflictedTargets = panel.getByRole("group", {
      name: "Bookmark targets",
    });
    await expect(conflictedTargets).toHaveTextContent("2");
    await expect(
      within(conflictedTargets).getAllByRole("button", {
        name: /Copy Git commit SHA/u,
      }),
    ).toHaveLength(2);
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/native-objects" }),
    );
    await userEvent.click(panel.getByRole("button", { name: "Add bookmark" }));
    const createDialog = await page.findByRole("dialog", {
      name: "Create bookmark",
    });
    await expect(
      within(createDialog).queryByRole("textbox", { name: "Commit" }),
    ).not.toBeInTheDocument();
    const createTarget = within(createDialog).getByRole("combobox", {
      name: "Commit",
    });
    await expect(createTarget).toHaveTextContent(
      "Stream immutable native objects",
    );
    await expect(createTarget).toHaveTextContent("feature/native-objects");
    await userEvent.click(createTarget);
    const search = page.getByRole("combobox", { name: "Select commit" });
    await userEvent.type(search, "Initialize native repository");
    await userEvent.click(
      page.getByRole("option", {
        name: /Initialize native repository/u,
      }),
    );
    await expect(createTarget).toHaveTextContent(
      "Initialize native repository",
    );
    await userEvent.click(
      within(createDialog).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Create bookmark" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(panel.getByRole("button", { name: "Move bookmark" }));
    const dialog = await page.findByRole("dialog", { name: "Move bookmark" });
    await expect(within(dialog).getByLabelText("Bookmark")).toHaveValue(
      "feature/native-objects",
    );
    await expect(
      within(dialog).getByRole("combobox", { name: "Commit" }),
    ).toHaveTextContent("Stream immutable native objects");
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() =>
      expect(
        page.queryByRole("dialog", { name: "Move bookmark" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Stream immutable native objects" }),
    );
    await expect(panel.getByText("Selected change")).toBeVisible();
    await expect(
      panel.queryByText("Selected bookmark"),
    ).not.toBeInTheDocument();
  }}
/>
