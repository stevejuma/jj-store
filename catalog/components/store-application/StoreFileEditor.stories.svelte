<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Repository/File editor",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    args: { initialPage: "files" },
  });
</script>

<Story
  name="Edit and rename"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    const metadata = canvas.getByRole("region", {
      name: "Selected file metadata",
    });
    const treeToggle = within(metadata).getByRole("button", {
      name: "Hide repository file tree",
    });
    await expect(metadata.firstElementChild).toBe(treeToggle);
    const fileDisplay = canvas.getByRole("group", { name: "File display" });
    await expect(
      within(fileDisplay)
        .getAllByRole("button")
        .map((button) => button.textContent?.trim()),
    ).toEqual(["Preview", "Source", "Edit"]);
    await userEvent.click(
      within(fileDisplay).getByRole("button", { name: "Edit" }),
    );
    const editor = canvas.getByRole("region", { name: "Edit README.md" });
    const filename = canvas.getByLabelText("File name");
    await userEvent.clear(filename);
    await userEvent.type(filename, "NOTES.md");
    await userEvent.clear(within(editor).getByLabelText("Contents"));
    await userEvent.type(
      within(editor).getByLabelText("Contents"),
      "# Edited notes\n\nSaved from the repository browser.",
    );
    await userEvent.click(
      within(fileDisplay).getByRole("button", { name: "Preview" }),
    );
    await expect(
      canvas.getByRole("region", {
        name: "Edited Markdown preview for NOTES.md",
      }),
    ).toHaveTextContent("Saved from the repository browser.");
    await userEvent.click(
      within(fileDisplay).getByRole("button", { name: "Edit" }),
    );
    await userEvent.click(
      within(editor).getByRole("button", { name: "Save changes" }),
    );
    const dialog = body.getByRole("dialog", { name: "Save changes" });
    const description = within(dialog).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.clear(description);
    await userEvent.type(description, "Rename README to project notes");
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview changes" }),
    );
    await expect(within(dialog).getByText("Preview prepared")).toBeVisible();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Apply changes" }),
    );
    await expect(
      await canvas.findByRole("region", {
        name: "Markdown preview for NOTES.md",
      }),
    ).toHaveTextContent("Saved from the repository browser.");
    await expect(
      canvas.getByRole("list", { name: "Repository breadcrumb" }),
    ).toHaveTextContent("NOTES.md");
  }}
/>
