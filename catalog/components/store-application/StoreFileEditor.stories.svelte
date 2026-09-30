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
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    const metadata = canvas.getByRole("region", {
      name: "Selected file metadata",
    });
    const treeToggle = within(metadata).getByRole("button", {
      name: "Hide repository file tree",
    });
    await expect(metadata.firstElementChild).toBe(treeToggle);
    const edit = await within(metadata).findByRole("button", {
      name: "Edit file",
    });
    await expect(edit).toHaveTextContent("");
    await userEvent.click(edit);
    const editor = canvas.getByRole("region", { name: "Edit README.md" });
    const filename = canvas.getByLabelText("File name");
    await userEvent.clear(filename);
    await userEvent.type(filename, "NOTES.md");
    await userEvent.clear(within(editor).getByLabelText("Contents"));
    await userEvent.type(
      within(editor).getByLabelText("Contents"),
      "# Edited notes\n\nSaved from the repository browser.",
    );
    const description = within(editor).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.clear(description);
    await userEvent.type(description, "Rename README to project notes");
    await userEvent.click(
      within(editor).getByRole("button", { name: "Preview changes" }),
    );
    await expect(within(editor).getByText("Preview prepared")).toBeVisible();
    await userEvent.click(
      within(editor).getByRole("button", { name: "Apply changes" }),
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
