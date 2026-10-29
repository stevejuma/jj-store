<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StoreWorkflows from "../../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow editor",
    component: StoreWorkflows,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
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
  name="Sequence controls preserve selection and hide the inspector"
  play={async ({ canvasElement }) => {
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const element = await body.findByRole("dialog", {
      name: "Create workflow",
    });
    const dialog = within(element);
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    const inspector = dialog.getByRole("complementary", {
      name: "Workflow inspector",
    });
    await userEvent.click(
      dialog.getByRole("button", { name: "Close inspector" }),
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await expect(inspector).not.toBeVisible();
    await waitFor(async () => {
      await expect(
        dialog.getByRole("button", { name: "Trigger Run manually" }),
      ).toBeVisible();
    });
    await expect(element.querySelectorAll(".workflow-node")).toHaveLength(2);
    await userEvent.click(
      dialog.getByRole("button", { name: "Trigger Run manually" }),
    );
    await expect(inspector).toBeVisible();
    const add = dialog.getByRole("button", { name: "Add action" });
    await pointer.hover(add);
    await expect(getComputedStyle(add).borderRadius).toBe("50%");
    await expect(
      getComputedStyle(add.querySelector(".workflow-connector__plus")!).opacity,
    ).toBe("1");
    await expect(
      getComputedStyle(add.querySelector(".workflow-connector__arrow")!)
        .opacity,
    ).toBe("0");
    await userEvent.click(add);
    const name = dialog.getByLabelText("Action name");
    await userEvent.clear(name);
    await userEvent.type(name, "test");
    const deletion = dialog.getByRole("button", { name: "Delete test" });
    await pointer.hover(dialog.getByLabelText("Name", { exact: true }));
    await expect(getComputedStyle(deletion.parentElement!).opacity).toBe("0");
    await pointer.hover(dialog.getByRole("button", { name: "test build" }));
    await expect(getComputedStyle(deletion.parentElement!).opacity).toBe("1");
    await pointer.hover(deletion);
    await expect(getComputedStyle(deletion).borderRadius).toBe("50%");
    await expect(getComputedStyle(deletion).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await pointer.hover(dialog.getByLabelText("Name", { exact: true }));
    await pointer.hover(dialog.getByRole("button", { name: "test build" }));
    await userEvent.click(dialog.getByRole("button", { name: "Move test up" }));
    await expect(name).toHaveValue("test");
    await expect(
      dialog.getByRole("button", { name: "Move test up" }),
    ).toBeDisabled();
    await expect(
      Array.from(element.querySelectorAll(".workflow-node strong")).map(
        (e) => e.textContent,
      ),
    ).toEqual(["Trigger", "test", "build"]);
    await pointer.hover(dialog.getByRole("button", { name: "test build" }));
    await userEvent.click(
      dialog.getByRole("button", { name: "Move test down" }),
    );
    await expect(name).toHaveValue("test");
    await expect(
      Array.from(element.querySelectorAll(".workflow-node strong")).map(
        (e) => e.textContent,
      ),
    ).toEqual(["Trigger", "build", "test"]);
    const inputs = dialog.getByLabelText("Inputs (JSON)");
    await userEvent.clear(inputs);
    await userEvent.type(inputs, "unfinished");
    await userEvent.click(
      dialog.getByRole("button", { name: "Close inspector" }),
    );
    await expect(inspector).not.toBeVisible();
    await expect(getComputedStyle(deletion.parentElement!).opacity).toBe("0");
    await userEvent.click(dialog.getByRole("button", { name: "test build" }));
    await expect(inputs).toHaveValue("unfinished");
    await pointer.hover(dialog.getByRole("button", { name: "test build" }));
    await userEvent.click(dialog.getByRole("button", { name: "Delete test" }));
    await expect(element.querySelectorAll(".workflow-node")).toHaveLength(2);
    await expect(
      dialog.getByRole("button", { name: "Delete build" }),
    ).toBeEnabled();
    await expect(
      dialog.getByRole("button", { name: "Save workflow" }),
    ).toBeEnabled();
    await pointer.hover(dialog.getByRole("button", { name: "build build" }));
    const controls = dialog.getByRole("button", {
      name: "Move build up",
    }).parentElement!;
    await expect(getComputedStyle(controls).opacity).toBe("1");
    await pointer.hover(dialog.getByLabelText("Name", { exact: true }));
    await expect(getComputedStyle(controls).opacity).toBe("0");
    const home = dialog.getByRole("combobox", { name: "Metadata home" });
    await expect(home.closest(".workflow-canvas")).not.toBeNull();
    await expect(getComputedStyle(home).borderTopWidth).toBe("0px");
    await expect(getComputedStyle(inspector).borderLeftWidth).toBe("0px");
    await expect(getComputedStyle(inspector).borderRadius).toBe("16px");
    await expect(
      getComputedStyle(
        element.querySelector('[data-ui-part="dialog-frame-header"]')!,
      ).alignItems,
    ).toBe("center");
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Settings layout and menu manage the current draft"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const element = await body.findByRole("dialog", {
      name: "Create workflow",
    });
    const dialog = within(element);
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    const name = dialog.getByLabelText("Name", { exact: true });
    const labels = dialog.getByRole("combobox", { name: "Runner labels" });
    await expect(name.closest(".workflow-editor-name")).not.toBeNull();
    const labelsRow = labels.closest('[data-slot="field"]')!;
    await expect(
      getComputedStyle(labelsRow).gridTemplateColumns.split(" "),
    ).toHaveLength(1);
    const chipBox = labels.closest(".chip-autocomplete-box")!;
    await expect(
      parseFloat(
        getComputedStyle(labels.closest(".workflow-labels")!).marginLeft,
      ),
    ).toBeGreaterThan(30);
    await expect(getComputedStyle(chipBox).borderRadius).toBe(
      getComputedStyle(name).borderRadius,
    );
    if (dialog.queryByRole("button", { name: "Edit workflow name" }))
      await userEvent.click(
        dialog.getByRole("button", { name: "Edit workflow name" }),
      );
    await userEvent.click(labels);
    await expect(getComputedStyle(chipBox).boxShadow).toContain(
      "0px 0px 0px 3px",
    );
    await userEvent.click(name);
    await expect(getComputedStyle(chipBox).borderTopWidth).toBe("1px");
    await userEvent.type(name, "Draft build");
    await expect(
      element.querySelector(
        '[data-ui-part="dialog-frame-header"] [role="switch"]',
      ),
    ).toBeNull();
    await userEvent.click(
      dialog.getByRole("button", { name: "Workflow actions" }),
    );
    await expect(
      body.getByRole("menuitem", { name: "Trigger" }),
    ).toHaveAttribute("aria-disabled", "true");
    await expect(
      body.getByRole("menuitem", { name: "Delete" }),
    ).toHaveAttribute("aria-disabled", "true");
    const enable = body.getByRole("menuitemcheckbox", { name: "Enable" });
    await userEvent.click(enable);
    await expect(enable).toHaveAttribute("aria-checked", "true");
    await expect(
      getComputedStyle(
        enable.querySelector(
          '[data-slot="dropdown-menu-checkbox-item-indicator"]',
        )!,
      ).display,
    ).toBe("none");
    await userEvent.click(body.getByRole("menuitem", { name: "Duplicate" }));
    await userEvent.click(
      await body.findByRole("button", { name: "Edit workflow name" }),
    );
    await waitFor(() =>
      expect(body.getByLabelText("Name", { exact: true })).toHaveValue(
        "Draft build copy",
      ),
    );
    const copy = within(body.getByRole("dialog", { name: "Create workflow" }));
    await userEvent.click(
      copy.getByRole("button", { name: "Workflow actions" }),
    );
    await expect(
      body.getByRole("menuitemcheckbox", { name: "Enable" }),
    ).toHaveAttribute("aria-checked", "false");
    await userEvent.click(body.getByRole("menuitem", { name: "Edit" }));
    await expect(copy.getByRole("tab", { name: "Form" })).toHaveAttribute(
      "data-state",
      "active",
    );
    await userEvent.click(copy.getByRole("button", { name: "Save workflow" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    const savedCard = canvas.getByRole("button", {
      name: "View Draft build copy",
    });
    await waitFor(() =>
      expect(getComputedStyle(savedCard).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(savedCard);
    const saved = within(
      await body.findByRole("dialog", { name: "Edit workflow" }),
    );
    await userEvent.click(
      saved.getByRole("button", { name: "Workflow actions" }),
    );
    await expect(
      body.getByRole("menuitem", { name: "Trigger" }),
    ).not.toHaveAttribute("aria-disabled", "true");
    await userEvent.click(body.getByRole("menuitem", { name: "Delete" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    await expect(
      canvas.queryByText("Draft build copy", { exact: true }),
    ).toBeNull();
    await expect(
      canvas.getByText("Build parser", { exact: true }),
    ).toBeVisible();
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Header name chip and sidebar deletion edit only the draft"
  play={async ({ canvasElement }) => {
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const element = await body.findByRole("dialog", {
      name: "Create workflow",
    });
    const dialog = within(element);
    const edit = dialog.getByRole("button", { name: "Edit workflow name" });
    await expect(getComputedStyle(edit).borderRadius).toBe("50%");
    await waitFor(async () => {
      await expect(
        dialog.getByRole("heading", { name: "Create workflow" }),
      ).toBeVisible();
    });
    await userEvent.click(edit);
    const name = dialog.getByRole("textbox", { name: "Name" });
    await expect(getComputedStyle(name).borderTopWidth).toBe("0px");
    await expect(getComputedStyle(name).boxShadow).toBe("none");
    await expect(
      getComputedStyle(name.closest(".workflow-editor-name")!).boxShadow,
    ).toContain("0px 0px 0px 3px");
    await userEvent.type(name, "Build all");
    const accept = dialog.getByRole("button", { name: "Save workflow name" });
    await expect(getComputedStyle(accept).borderRadius).toBe("50%");
    await userEvent.click(accept);
    await expect(dialog.queryByRole("textbox", { name: "Name" })).toBeNull();
    await waitFor(async () => {
      await expect(
        dialog.getByText("Build all", { exact: true }),
      ).toBeVisible();
    });
    await userEvent.click(dialog.getByRole("button", { name: "build build" }));
    const remove = dialog.getByRole("button", {
      name: "Delete selected action",
    });
    await expect(remove).toBeEnabled();
    await pointer.hover(remove);
    await expect(getComputedStyle(remove).borderRadius).toBe("50%");
    await expect(getComputedStyle(remove).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await userEvent.click(remove);
    await expect(element.querySelectorAll(".workflow-node")).toHaveLength(1);
    await waitFor(async () => {
      await expect(
        dialog.getByRole("button", { name: "Trigger Run manually" }),
      ).toBeVisible();
    });
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    await waitFor(async () => {
      await expect(
        await body.findByText(
          "Choose a name, source repository, trigger and at least one action",
        ),
      ).toBeVisible();
    });
    await userEvent.click(dialog.getByRole("button", { name: "Add action" }));
    await expect(element.querySelectorAll(".workflow-node")).toHaveLength(2);
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    await expect(
      canvas.queryByRole("heading", { name: "Build all" }),
    ).toBeNull();
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
