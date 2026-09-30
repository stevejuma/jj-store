<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

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
      panel.getByRole("navigation", { name: "Repository views" }),
    ).toBeVisible();
    const actions = panel.getByRole("group", {
      name: "Selected repository actions",
    });
    await expect(actions).toBeVisible();
    await expect(
      within(actions).getByRole("button", { name: "Move bookmark" }),
    ).toBeEnabled();
    await expect(details.querySelectorAll("dl")).toHaveLength(0);
    await expect(
      panel.getByRole("button", { name: "Add bookmark" }),
    ).toBeVisible();
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
    await userEvent.click(
      within(actions).getByRole("button", { name: "Move bookmark" }),
    );
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
    await expect(panel.queryByText("Selected change")).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("region", { name: "Changed files" }),
    ).toBeVisible();
    await expect(
      panel.queryByText("Selected bookmark"),
    ).not.toBeInTheDocument();
  }}
/>
