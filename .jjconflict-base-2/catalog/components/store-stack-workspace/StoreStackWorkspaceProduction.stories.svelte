<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceProductionFixture from "../../fixtures/StackWorkspaceProductionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Production reads",
    component: StackWorkspaceProductionFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function browse(canvasElement: HTMLElement) {
    const canvas = within(canvasElement),
      body = within(document.body);
    const picker = await canvas.findByRole("combobox", {
      name: "Workspace target branch",
    });
    await userEvent.click(picker);
    await userEvent.click(await body.findByRole("option", { name: /main/ }));
    await userEvent.click(await canvas.findByRole("option", { name: /topic/ }));
    const stack = await canvas.findByRole("region", { name: "Stack topic" });
    await userEvent.click(
      await within(stack).findByRole("button", { name: "Topic ancestor" }),
    );
    await userEvent.click(
      await within(stack).findByRole("button", { name: "src/topic.ts" }),
    );
    const diff = await canvas.findByRole("region", {
      name: /^Selected changes from/,
    });
    await within(diff).findByRole("group", { name: /^Draggable hunk/ });
    expect(canvas.getByRole("button", { name: "Create stack" })).toBeDisabled();
    expect(stack.querySelector("[data-revision-id]")).toHaveAttribute(
      "draggable",
      "false",
    );
  }
</script>

<Story
  name="Bounded real adapter in application shell"
  play={async ({ canvasElement }) => browse(canvasElement)}
/>
<Story
  name="Cold graph preparation restores only Stacks"
  args={{ cold: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await within(document.body).findByRole("button", {
        name: "Prepare graph index",
      }),
    );
    await browse(canvasElement);
  }}
/>

<Story
  name="Graph preparation survives cleared feedback"
  args={{ cold: true, clearTransientFeedback: true }}
  play={async ({ canvasElement }) => {
    await userEvent.click(
      await within(document.body).findByRole("button", {
        name: "Prepare graph index",
      }),
    );
    await browse(canvasElement);
  }}
/>
<Story
  name="Dependent native branch segments"
  args={{ dependent: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("combobox", { name: "Workspace target branch" }),
    );
    await userEvent.click(await body.findByRole("option", { name: /main/ }));
    await userEvent.click(
      await canvas.findByRole("option", { name: /^topic/ }),
    );
    const stack = await canvas.findByRole("region", { name: "Stack topic" });
    await within(stack).findByRole("button", {
      name: "Show all changes in foundation",
    });
    await waitFor(() =>
      expect(
        within(stack).getAllByRole("button", {
          name: "Topic ancestor",
        }),
      ).toHaveLength(1),
    );
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in topic",
      }),
    );
    await expect(
      await within(stack).findByRole("region", {
        name: "Changed files in branch topic",
      }),
    ).toBeVisible();
    await within(
      within(stack).getByRole("region", {
        name: "Changed files in branch topic",
      }),
    ).findByRole("button", { name: "src/topic.ts" });
    await userEvent.click(
      within(stack).getByRole("button", {
        name: "Show all changes in foundation",
      }),
    );
    await expect(
      await within(stack).findByRole("region", {
        name: "Changed files in branch foundation",
      }),
    ).toBeVisible();
    await within(
      within(stack).getByRole("region", {
        name: "Changed files in branch foundation",
      }),
    ).findByRole("button", { name: "src/topic.ts" });
  }}
/>
