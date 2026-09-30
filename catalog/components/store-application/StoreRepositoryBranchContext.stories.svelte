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
    await expect(panel.getByText("Selected bookmark")).toBeVisible();
    await expect(panel.getByText("feature/native-objects")).toBeVisible();
    await expect(
      panel.getByText("Stream immutable native objects"),
    ).toBeVisible();
    await expect(panel.getByText("1 target")).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Add bookmark" }),
    ).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Move bookmark" }),
    ).toBeVisible();
    await expect(
      panel.getByRole("button", { name: "Delete bookmark" }),
    ).toBeVisible();
    await userEvent.click(panel.getByRole("button", { name: "Add bookmark" }));
    const createDialog = await page.findByRole("dialog", {
      name: "Create bookmark",
    });
    await expect(within(createDialog).getByLabelText("Commit")).toHaveValue(
      IDS.left,
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
