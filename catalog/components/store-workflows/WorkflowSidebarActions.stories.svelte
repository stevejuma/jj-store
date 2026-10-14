<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const initialHash = "#/workflows?tab=history";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow sidebar actions",
    component: WorkflowsFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
    args: { initialHash, historyExamples: true },
  });
  async function openMenu(canvasElement: HTMLElement) {
    const nav = within(
      await within(canvasElement).findByRole("navigation", {
        name: "Workflows by repository",
      }),
    );
    const trigger = nav.getByRole("button", {
      name: "Actions for Build parser",
    });
    await waitFor(() =>
      expect(getComputedStyle(trigger).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(trigger);
    const body = within(document.body);
    await body.findByRole("menuitem", { name: "Edit workflow" });
    return body;
  }
  async function closeDialog() {
    const body = within(document.body);
    await userEvent.click(
      within(body.getByRole("dialog")).getByRole("button", { name: "Cancel" }),
    );
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  }
</script>

<Story
  name="Sidebar actions open edit run and disabled duplicate drafts without selecting a workflow"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    let body = await openMenu(canvasElement);
    for (const label of [
      "Edit workflow",
      "Run workflow",
      "Duplicate",
      "Disable workflow",
      "Remove workflow",
    ])
      await expect(
        body.getByRole("menuitem", { name: label }).querySelector("svg"),
      ).not.toBeNull();
    await userEvent.click(
      body.getByRole("menuitem", { name: "Edit workflow" }),
    );
    await expect(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    ).toBeVisible();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      initialHash,
    );
    await closeDialog();
    body = await openMenu(canvasElement);
    await userEvent.click(body.getByRole("menuitem", { name: "Run workflow" }));
    await expect(
      await body.findByRole("dialog", { name: "Run Build parser" }),
    ).toBeVisible();
    await closeDialog();
    body = await openMenu(canvasElement);
    await userEvent.click(body.getByRole("menuitem", { name: "Duplicate" }));
    const dialog = within(
      await body.findByRole("dialog", { name: "Create workflow" }),
    );
    await expect(
      dialog.getByText("Build parser copy", { exact: true }),
    ).toBeVisible();
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("enabled: false");
    await closeDialog();
    await expect(
      canvas.getByRole("button", { name: /^Build parser$/ }),
    ).toBeVisible();
  }}
/>

<Story
  name="Sidebar enable disable and remove reuse definition commands and preserve run history"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    let body = await openMenu(canvasElement);
    await userEvent.click(
      body.getByRole("menuitem", { name: "Disable workflow" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Last workflow command")).toHaveTextContent(
        '"enabled":false',
      ),
    );
    await waitFor(() =>
      expect(body.queryAllByRole("menuitem")).toHaveLength(0),
    );
    body = await openMenu(canvasElement);
    await userEvent.click(
      body.getByRole("menuitem", { name: "Enable workflow" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Last workflow command")).toHaveTextContent(
        '"enabled":true',
      ),
    );
    await waitFor(() =>
      expect(body.queryAllByRole("menuitem")).toHaveLength(0),
    );
    body = await openMenu(canvasElement);
    await userEvent.click(
      body.getByRole("menuitem", { name: "Remove workflow" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Last workflow command")).toHaveTextContent(
        '"type":"remove"',
      ),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", { name: /^Build parser$/ }),
      ).toBeNull(),
    );
    await expect(
      canvas.getByLabelText("Last workflow command"),
    ).not.toHaveTextContent('"type":"run"');
  }}
/>

<Story
  name="Read only sidebar permits inspection and blocks definition mutations"
  args={{ readOnly: true }}
  play={async ({ canvasElement }) => {
    const body = await openMenu(canvasElement);
    for (const label of [
      "Run workflow",
      "Duplicate",
      "Disable workflow",
      "Remove workflow",
    ])
      await expect(body.getByRole("menuitem", { name: label })).toHaveAttribute(
        "aria-disabled",
        "true",
      );
    await userEvent.click(
      body.getByRole("menuitem", { name: "Edit workflow" }),
    );
    await expect(
      within(
        await body.findByRole("dialog", { name: "Edit workflow" }),
      ).getByRole("button", { name: "Save workflow" }),
    ).toBeDisabled();
    await closeDialog();
  }}
/>

<Story
  name="Unknown sidebar mutation uses a toast and disables subsequent mutations"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    let body = await openMenu(canvasElement);
    await userEvent.click(
      body.getByRole("menuitem", { name: "Disable workflow" }),
    );
    const notice = await body.findByText(
      "The outcome is still unknown. Check the original request before continuing.",
    );
    await expect(notice.closest('[data-ui-part="toast"]')).not.toBeNull();
    await waitFor(() =>
      expect(body.queryAllByRole("menuitem")).toHaveLength(0),
    );
    body = await openMenu(canvasElement);
    for (const label of [
      "Run workflow",
      "Duplicate",
      "Disable workflow",
      "Remove workflow",
    ])
      await expect(body.getByRole("menuitem", { name: label })).toHaveAttribute(
        "aria-disabled",
        "true",
      );
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Shared global History body remains without the Actions page divider"
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByRole("navigation", {
      name: "Workflows by repository",
    });
    await expect(
      getComputedStyle(canvasElement.querySelector(".workflow-history-layout")!)
        .borderTopWidth,
    ).toBe("0px");
  }}
/>
