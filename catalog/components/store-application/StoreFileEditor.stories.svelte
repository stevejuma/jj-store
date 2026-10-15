<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { expectSourceHighlight } from "./source-highlight-assertions.js";

  const { Story } = defineMeta({
    title: "JJ Store/Repository/File editor",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    args: { initialPage: "files" },
  });
</script>

<Story
  name="Return to root clears file editor"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const workspace = canvas.getByRole("combobox", {
      name: "Repository workspace",
    });
    const returnToRoot = async () => {
      const filename = canvas.queryByLabelText("File name");
      await userEvent.click(canvas.getByRole("link", { name: /^Files$/ }));
      if (filename) {
        await expect(filename).toBeInTheDocument();
        await expect(filename).toHaveValue("README.md");
        await userEvent.click(canvas.getByRole("button", { name: "Cancel" }));
        await userEvent.click(canvas.getByRole("link", { name: /^Files$/ }));
      }
      await waitFor(() =>
        expect(
          canvas.queryByLabelText("File name"),
          "root route clears editor",
        ).not.toBeInTheDocument(),
      );
      await expect(workspace).toBeEnabled();
      await expect(
        canvas.getByRole("region", { name: "Repository document preview" }),
      ).toBeVisible();
    };
    await userEvent.click(
      await canvas.findByRole("button", { name: "Show repository file tree" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    await returnToRoot();
    await userEvent.click(canvas.getByRole("button", { name: "Edit README" }));
    await expect(await canvas.findByLabelText("File name")).toHaveValue(
      "README.md",
    );
    await expect(workspace).toBeDisabled();
    await returnToRoot();
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    await waitFor(() =>
      expect(
        canvas.queryByLabelText("File name"),
        "reopened file uses reading mode",
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(canvas.getByRole("button", { name: /^Edit$/ }));
    await expect(canvas.getByLabelText("File name")).toHaveValue("README.md");
    await returnToRoot();
    await userEvent.click(canvas.getByRole("button", { name: "README.md" }));
    await waitFor(() =>
      expect(
        canvas.queryByLabelText("File name"),
        "reopened file uses reading mode",
      ).not.toBeInTheDocument(),
    );
  }}
/>

<Story
  name="Edit and rename"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Open commit details" }),
    );
    await waitFor(() =>
      expect(canvas.getByRole("tab", { name: "Commits" })).toHaveAttribute(
        "aria-selected",
        "true",
      ),
    );
    await userEvent.click(canvas.getByRole("tab", { name: "Overview" }));
    await userEvent.click(
      await canvas.findByRole("button", { name: "Show repository file tree" }),
    );
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
    ).toEqual(["Preview", "Source", "Blame", "Edit"]);
    await userEvent.click(
      within(fileDisplay).getByRole("button", { name: "Edit" }),
    );
    const editor = canvas.getByRole("region", { name: "Edit README.md" });
    const filename = canvas.getByLabelText("File name");
    const editorSurface = editor.querySelector<HTMLElement>(
      ".store-file-editor__contents",
    )!;
    await expect(editor).not.toHaveTextContent("Contents");
    await expect(editorSurface).toHaveAttribute(
      "data-source-language",
      "Markdown",
    );
    const editorShell = editor.querySelector<HTMLElement>(".mira-code-editor")!;
    const codeMirror = editor.querySelector<HTMLElement>(".cm-editor")!;
    const scroller = editor.querySelector<HTMLElement>(".cm-scroller")!;
    const gutters = editor.querySelector<HTMLElement>(".cm-gutters")!;
    const activeLine = editor.querySelector<HTMLElement>(".cm-activeLine")!;
    const firstLine = editor.querySelector<HTMLElement>(".cm-line")!;
    const firstLineNumber = Array.from(
      editor.querySelectorAll<HTMLElement>(".cm-lineNumbers .cm-gutterElement"),
    ).find((element) => element.textContent?.trim() === "1")!;
    await expect(firstLineNumber).toBeDefined();
    await expect(
      gutters.getBoundingClientRect().left -
        scroller.getBoundingClientRect().left,
    ).toBeLessThanOrEqual(16);
    const content = editor.querySelector<HTMLElement>(".cm-content")!;
    await expect(
      scroller.getBoundingClientRect().right -
        content.getBoundingClientRect().right,
    ).toBeLessThanOrEqual(16);
    await expect(content.getBoundingClientRect().width).toBeGreaterThan(
      scroller.clientWidth - 100,
    );
    await expect(scroller.scrollWidth).toBeLessThanOrEqual(
      scroller.clientWidth,
    );
    await expect(getComputedStyle(scroller).display).toBe("flex");
    await waitFor(() =>
      expect(
        Math.abs(
          firstLineNumber.getBoundingClientRect().top -
            firstLine.getBoundingClientRect().top,
        ),
      ).toBeLessThanOrEqual(1),
    );
    await expect(getComputedStyle(gutters).backgroundColor).toBe(
      getComputedStyle(editorShell).backgroundColor,
    );
    await expect(getComputedStyle(editorShell).backgroundColor).toBe(
      getComputedStyle(
        editor.closest<HTMLElement>(".community-projects__file-content-card")!,
      ).backgroundColor,
    );
    await expect(getComputedStyle(activeLine).backgroundColor).toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(getComputedStyle(editorShell).borderWidth).toBe("0px");
    const filenameItem = filename.closest<HTMLElement>(
      ".store-application__breadcrumb-filename",
    )!;
    await expect(getComputedStyle(filenameItem).flexGrow).toBe("0");
    await expect(filename.getBoundingClientRect().width).toBeLessThan(160);
    await expect(
      Number.parseFloat(getComputedStyle(filename).paddingInlineStart),
    ).toBeGreaterThan(0);
    await expect(
      getComputedStyle(filename).getPropertyValue("field-sizing"),
    ).toBe("content");
    await userEvent.clear(filename);
    await userEvent.type(filename, "NOTES.md");
    const contents = within(editor).getByRole("textbox", {
      name: "Editing NOTES.md file contents",
    });
    await userEvent.click(contents);
    await waitFor(() => expect(codeMirror).toHaveClass("cm-focused"));
    await expect(getComputedStyle(codeMirror).outlineStyle).toBe("none");
    await expect(getComputedStyle(codeMirror).boxShadow).toBe("none");
    await expect(contents).toHaveFocus();
    await expect(getComputedStyle(contents).outlineStyle).toBe("none");
    await expect(getComputedStyle(contents).boxShadow).toBe("none");
    await userEvent.clear(contents);
    await userEvent.type(
      contents,
      "# Edited notes\n\nSaved from the repository browser.",
    );
    await waitFor(() =>
      expect(editor.querySelector(".cm-heading")).not.toBeNull(),
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
      within(fileDisplay).getByRole("button", { name: "Source" }),
    );
    await expectSourceHighlight(canvasElement, "markdown", "# Edited notes");
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
    await expect(description).toHaveFocus();
    await expect(getComputedStyle(description).outlineStyle).toBe("none");
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
    await waitFor(() =>
      expect(
        body.queryByRole("dialog", { name: "Save changes" }),
      ).not.toBeInTheDocument(),
    );
  }}
/>
