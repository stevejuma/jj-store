<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/WorkflowFilePickerFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow YAML files",
    component: Fixture,
    tags: ["visual-pending"],
  });
  async function selectFile({ canvasElement }: { canvasElement: HTMLElement }) {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    const picker = await canvas.findByRole("textbox", {
      name: "Workflow file",
    });
    await expect(picker).toBeEnabled();
    await userEvent.click(canvas.getByRole("button", { name: "Browse…" }));
    await userEvent.click(
      await body.findByRole("option", {
        name: "check.yaml .github/workflows/check.yaml",
      }),
    );
    await expect(picker).toHaveValue(".github/workflows/check.yaml");
  }
  async function failedRead({ canvasElement }: { canvasElement: HTMLElement }) {
    const canvas = within(canvasElement),
      body = within(canvasElement.ownerDocument.body);
    const picker = await canvas.findByRole("textbox", {
      name: "Workflow file",
    });
    await expect(picker).toBeEnabled();
    await userEvent.click(canvas.getByRole("button", { name: "Browse…" }));
    await waitFor(async () => {
      await expect(
        await body.findByText(
          "Workflow source read failed. Your selection is retained.",
        ),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await expect(picker).toHaveValue(".github/workflows/ci.yml");
    await expect(canvas.queryByRole("alert")).toBeNull();
  }
</script>

<Story name="Pinned YAML choices use the command picker" play={selectFile}
  >{#snippet template()}<Fixture />{/snippet}</Story
>
<Story
  name="Read failures use a toast and preserve the selected workflow"
  play={failedRead}>{#snippet template()}<Fixture fail />{/snippet}</Story
>
