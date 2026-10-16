<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StoreWorkflows from "../../../web/src/lib/ui/store-workflows/StoreWorkflows.svelte";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow YAML",
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
  name="YAML fills create and edit dialogs without changing Form inspector state"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    for (const create of [true, false]) {
      const opener = await canvas.findByRole("button", {
        name: create ? "Create workflow" : "View Build parser",
      });
      await waitFor(() =>
        expect(getComputedStyle(opener).pointerEvents).not.toBe("none"),
      );
      await userEvent.click(opener);
      const element = await body.findByRole("dialog", {
        name: create ? "Create workflow" : "Edit workflow",
      });
      const dialog = within(element);
      if (create)
        await userEvent.click(
          dialog.getByRole("button", { name: "Close inspector" }),
        );
      else
        await userEvent.click(
          dialog.getByRole("button", { name: "build build" }),
        );
      await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
      await expect(
        dialog.getByRole("textbox", { name: "Workflow YAML" }),
      ).toBeVisible();
      await expect(
        dialog.queryByRole("complementary", { name: "Workflow settings" }),
      ).toBeNull();
      const layout = element.querySelector<HTMLElement>(
        ".workflow-editor--yaml",
      )!;
      await waitFor(() => {
        const style = getComputedStyle(layout);
        const canvasWidth = layout
          .querySelector<HTMLElement>(".workflow-canvas")!
          .getBoundingClientRect().width;
        expect(
          Math.abs(
            canvasWidth -
              (layout.clientWidth -
                parseFloat(style.paddingLeft) -
                parseFloat(style.paddingRight)),
          ),
        ).toBeLessThanOrEqual(1);
      });
      await expect(layout.querySelector(".workflow-inspector")).toBeNull();
      const yamlArea = layout.querySelector<HTMLElement>(".workflow-yaml")!;
      const editor = yamlArea.querySelector<HTMLElement>(
        '[data-ui-component="code-editor"]',
      )!;
      await waitFor(() => {
        const yamlStyle = getComputedStyle(yamlArea),
          editorStyle = getComputedStyle(editor);
        expect(
          Math.abs(
            editor.getBoundingClientRect().width -
              (yamlArea.clientWidth -
                parseFloat(yamlStyle.paddingLeft) -
                parseFloat(yamlStyle.paddingRight) -
                parseFloat(editorStyle.marginLeft) -
                parseFloat(editorStyle.marginRight)),
          ),
        ).toBeLessThanOrEqual(1);
      });
      await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
      const inspector = element.querySelector<HTMLElement>(
        ".workflow-inspector",
      )!;
      if (create) {
        await expect(inspector).not.toBeVisible();
        await userEvent.click(
          dialog.getByRole("button", { name: "Trigger Run manually" }),
        );
        await expect(inspector).toBeVisible();
      } else {
        await expect(inspector).toBeVisible();
        await userEvent.click(
          dialog.getByRole("button", { name: "build build" }),
        );
        await expect(dialog.getByLabelText("Command name")).toHaveValue(
          "build",
        );
      }
      await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
      await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    }
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
