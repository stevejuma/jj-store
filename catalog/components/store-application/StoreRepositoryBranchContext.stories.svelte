<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Connected application/Branch context",
    component: StoreRepositoryApplication,
    tags: ["visual-pending", "application"],
    args: { initiallyCollapsed: false },
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    beforeEach: async () => {
      await Promise.all([views.log()]);
    },
  });
</script>

<Story
  name="Branch selection retains the loaded first column"
  args={{ deferredBranchSelection: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const search = await c.findByPlaceholderText("Search branches");
    await userEvent.type(search, "a");
    const first = c.getByRole("option", { name: "feature/native-objects" });
    const other = c.getByRole("option", { name: "main" });
    const workingCopy = c.getByRole("button", {
      name: "Working copy · default",
    });
    for (const option of [first, other, workingCopy]) {
      await userEvent.click(option);
      await expect(c.getByLabelText("Loading JJ log")).toBeVisible();
      await expect(first).toBeVisible();
      await expect(other).toBeVisible();
      await expect(c.getByRole("option", { name: "main" })).toBe(other);
      await expect(
        c.queryByLabelText("Loading branches"),
      ).not.toBeInTheDocument();
      await expect(search).toHaveValue("a");
      await expect(c.getByPlaceholderText("Search branches")).toBe(search);
      await expect(workingCopy).toBeVisible();
      await storyControl(document.body, "Finish branch load");
      await waitFor(() =>
        expect(c.queryByLabelText("Loading JJ log")).not.toBeInTheDocument(),
      );
    }
  }}
/>

<Story
  name="Actions"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/native-objects" }),
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
    const createDetails = createDialog.ownerDocument.getElementById(
      createTarget.getAttribute("aria-describedby")!,
    )!;
    await expect(createTarget).toHaveAccessibleDescription(
      /Stream immutable native objects/,
    );
    await waitFor(() =>
      expect(createDetails).toHaveTextContent(
        "Stream immutable native objects",
      ),
    );
    await expect(createDetails).toHaveTextContent("feature/native-objects");
    await userEvent.click(createTarget);
    const search = page.getByRole("combobox", { name: "Select commit" });
    await userEvent.type(search, "Initialize native repository");
    await userEvent.click(
      page.getByRole("option", {
        name: /Initialize native repository/u,
      }),
    );
    await expect(createDetails).toHaveTextContent(
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
    await waitFor(() =>
      expect(
        within(dialog).getByRole("combobox", { name: "Commit" }),
      ).toHaveAccessibleDescription(/Stream immutable native objects/),
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
    await expect(panel.queryByText("Selected change")).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("region", { name: "Changed files" }),
    ).toBeVisible();
    await expect(
      panel.queryByText("Selected bookmark"),
    ).not.toBeInTheDocument();
  }}
/>
