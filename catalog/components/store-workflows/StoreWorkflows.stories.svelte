<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StoreWorkflows from "../../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflows",
    component: StoreWorkflows,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      docs: {
        source: {
          code: "<StoreWorkflows {controller} />",
          language: "tsx",
          type: "code",
        },
      },
    },
  });
</script>

<Story
  name="Overview and sequence editor"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await expect(
      await canvas.findByRole("heading", { name: "Build parser" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: /Build parser Bookmark/ }),
    );
    const dialog = within(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await expect(
      dialog.getByRole("button", { name: /Trigger Bookmark main/ }),
    ).toBeVisible();
    await userEvent.click(dialog.getByRole("button", { name: "build build" }));
    await expect(dialog.getByLabelText("Command name")).toHaveValue("build");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("bookmark_changed");
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await userEvent.clear(dialog.getByLabelText("Name", { exact: true }));
    await userEvent.type(
      dialog.getByLabelText("Name", { exact: true }),
      "Build parser and tests",
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    await waitFor(() =>
      expect(body.queryByRole("dialog", { name: "Edit workflow" })).toBeNull(),
    );
    await expect(
      canvas.getByRole("heading", { name: "Build parser and tests" }),
    ).toBeVisible();
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
<Story
  name="Unknown outcome retains editor and original request"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: /Build parser Bookmark/ }),
    );
    const dialog = within(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    const notice = await body.findByText(
      "The outcome is still unknown. Check the original request before continuing.",
    );
    await expect(notice.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(
      dialog.queryByRole("status", { name: "Workflow command status" }),
    ).toBeNull();
    await expect(dialog.getByLabelText("Name", { exact: true })).toHaveValue(
      "Build parser",
    );
    await expect(
      body.getByRole("button", { name: "Check original request" }),
    ).toBeEnabled();
  }}>{#snippet template()}<WorkflowsFixture unknown />{/snippet}</Story
>
<Story name="Application navigation"
  >{#snippet template()}<WorkflowsFixture shell />{/snippet}</Story
>
<Story
  name="History pins source and preserves editor draft"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: /Build parser Bookmark/ }),
    );
    const editor = within(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    );
    if (editor.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        editor.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.clear(editor.getByLabelText("Name", { exact: true }));
    await userEvent.type(
      editor.getByLabelText("Name", { exact: true }),
      "Unsaved build draft",
    );
    await userEvent.click(editor.getByRole("button", { name: "History" }));
    const history = within(
      await body.findByRole("dialog", { name: /Run history/ }),
    );
    await userEvent.click(
      await history.findByRole("button", { name: /51515151/ }),
    );
    await expect(await history.findByText("61".repeat(64))).toBeVisible();
    await expect(history.getByText("62".repeat(64))).toBeVisible();
    await expect(history.getByLabelText("stdout log")).toHaveTextContent(
      "12 tests passed",
    );
    await expect(
      history.getByRole("button", { name: "Download result.txt" }),
    ).toBeVisible();
    await expect(
      history.getByRole("button", { name: "Pin this history" }),
    ).toBeDisabled();
    await userEvent.click(history.getByRole("button", { name: "Close" }));
    await waitFor(() =>
      expect(editor.getByLabelText("Name", { exact: true })).toHaveValue(
        "Unsaved build draft",
      ),
    );
    await userEvent.click(editor.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
<Story
  name="Create draft switches modes and enters runner chips"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await canvas.findByRole("heading", { name: "Build parser" });
    await expect(
      canvas.getAllByRole("button", { name: "Create workflow" }),
    ).toHaveLength(1);
    await expect(
      canvas.queryByRole("combobox", { name: "Metadata home" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Create workflow" }),
    );
    const dialog = within(
      await body.findByRole("dialog", { name: "Create workflow" }),
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await expect(
      dialog.getByRole("button", { name: "Trigger Run manually" }),
    ).toBeVisible();
    const labels = dialog.getByRole("combobox", { name: "Runner labels" });
    await userEvent.type(labels, "linux", { delay: 30 });
    await expect(labels).toHaveValue("linux");
    await userEvent.keyboard("{Enter}");
    await expect(
      await dialog.findByRole("button", { name: "Remove linux" }),
    ).toBeVisible();
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    const yaml = dialog.getByRole("textbox", { name: "Workflow YAML" });
    await expect(yaml).toHaveTextContent("linux");
    await userEvent.click(yaml);
    await userEvent.keyboard("{Control>}a{/Control}");
    await userEvent.type(yaml, "name: [[", { delay: 30, skipClick: true });
    const retainedYaml = yaml.textContent!;
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await expect(
      dialog.getByRole("button", { name: "Trigger Run manually" }),
    ).toBeVisible();
    const notice = await body.findByText(/your YAML text is retained/);
    await expect(notice).toBeVisible();
    await expect(notice.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(body.getAllByText(/your YAML text is retained/)).toHaveLength(
      1,
    );
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(
        getComputedStyle(
          canvas.getByRole("button", { name: "Create workflow" }),
        ).pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Create workflow" }),
    );
    const reopened = within(
      await body.findByRole("dialog", { name: "Create workflow" }),
    );
    if (reopened.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        reopened.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.click(reopened.getByRole("tab", { name: "YAML" }));
    await expect(
      reopened.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent(retainedYaml);
    await userEvent.click(reopened.getByRole("tab", { name: "Form" }));
    await userEvent.click(reopened.getByRole("button", { name: "Cancel" }));
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
<Story
  name="Validation errors use toasts and retain form fields"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const dialogElement = await body.findByRole("dialog", {
      name: "Create workflow",
    });
    const dialog = within(dialogElement);
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    const missingName = await body.findByText(
      "Choose a name, source repository, trigger and at least one action",
    );
    await expect(missingName.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(dialog.getByLabelText("Name", { exact: true })).toHaveValue(
      "",
    );
    await userEvent.click(
      body.getByRole("button", { name: "Dismiss Workflow needs attention" }),
    );
    await userEvent.type(
      dialog.getByLabelText("Name", { exact: true }),
      "Build draft",
    );
    await userEvent.click(dialog.getByRole("button", { name: "build build" }));
    const inputs = dialog.getByLabelText("Inputs (JSON)");
    await userEvent.clear(inputs);
    await userEvent.type(inputs, "broken");
    await userEvent.click(dialog.getByLabelText("Name", { exact: true }));
    const invalid = await body.findByText(
      "Inputs must be a JSON object of strings",
    );
    await expect(invalid.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(inputs).toHaveAttribute("aria-invalid", "true");
    await expect(
      dialog.getByRole("button", { name: "Save workflow" }),
    ).toBeDisabled();
    await expect(dialog.getByLabelText("Name", { exact: true })).toHaveValue(
      "Build draft",
    );
    await userEvent.clear(inputs);
    await userEvent.type(inputs, '{{"target":"test"}');
    await userEvent.click(dialog.getByLabelText("Name", { exact: true }));
    await waitFor(() =>
      expect(
        body.queryByText("Inputs must be a JSON object of strings"),
      ).toBeNull(),
    );
    await expect(
      dialog.getByRole("button", { name: "Save workflow" }),
    ).toBeEnabled();
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(dialogElement.isConnected).toBe(false));
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
