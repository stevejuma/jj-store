<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow triggers",
    component: WorkflowsFixture,
    parameters: {
      layout: "fullscreen",
      visualDelta: { tags: ["visual-pending"] },
    },
  });
</script>

<Story
  name="Scoped messages and reactions retain their form and YAML values"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const dialog = within(
      await body.findByRole("dialog", { name: "Create workflow" }),
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Edit workflow name" }),
    );
    await userEvent.type(
      dialog.getByRole("textbox", { name: "Name" }),
      "Reply to deploy",
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow name" }),
    );
    await userEvent.click(dialog.getByRole("button", { name: "build build" }));
    const unit = dialog.getByRole("button", { name: "Timeout unit" });
    await expect(
      getComputedStyle(unit.closest('[data-slot="input-group-addon"]')!)
        .backgroundColor,
    ).toBe(
      getComputedStyle(dialog.getByRole("spinbutton", { name: "Timeout" }))
        .backgroundColor,
    );
    await userEvent.click(unit);
    await expect(
      getComputedStyle(body.getByRole("listbox")).backgroundColor,
    ).toBe(
      getComputedStyle(dialog.getByRole("spinbutton", { name: "Timeout" }))
        .backgroundColor,
    );
    await userEvent.click(body.getByRole("option", { name: "min" }));
    await expect(
      dialog.getByRole("spinbutton", { name: "Timeout" }),
    ).toHaveValue(5);
    await userEvent.click(
      dialog.getByRole("button", { name: "Trigger Run manually" }),
    );
    await userEvent.click(dialog.getByRole("button", { name: "When" }));
    const message = body.getByRole("option", {
      name: "Thread message posted",
    });
    await expect(message.querySelector("svg")).not.toBeNull();
    await userEvent.click(message);
    await userEvent.click(dialog.getByRole("combobox", { name: "Thread" }));
    await userEvent.type(body.getByLabelText("Find a thread"), "general");
    await userEvent.click(await body.findByRole("option", { name: /general/ }));
    await userEvent.click(dialog.getByRole("button", { name: "Text match" }));
    await userEvent.click(body.getByRole("option", { name: "Contains" }));
    await userEvent.type(
      dialog.getByRole("textbox", { name: "Message text" }),
      "deploy",
    );
    await userEvent.click(dialog.getByRole("combobox", { name: "Authors" }));
    const author = dialog
        .getByRole("combobox", { name: "Authors" })
        .getBoundingClientRect(),
      match = dialog
        .getByRole("button", { name: "Author match" })
        .getBoundingClientRect();
    await expect(Math.abs(author.top - match.top)).toBeLessThan(2);
    await expect(
      (
        await body.findByRole("option", { name: new RegExp("22".repeat(16)) })
      ).querySelector('[data-ui-component="community-member-avatar"]'),
    ).not.toBeNull();
    await userEvent.type(
      body.getByLabelText("Find or enter an author"),
      "01".repeat(16),
    );
    await userEvent.click(
      await body.findByRole("option", { name: new RegExp("01".repeat(16)) }),
    );
    await userEvent.keyboard("{Escape}");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("message_posted");
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("deploy");
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("33".repeat(16));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("01".repeat(16));
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await userEvent.click(dialog.getByRole("button", { name: "When" }));
    await userEvent.click(body.getByRole("option", { name: "Reaction added" }));
    await expect(
      dialog.queryByRole("textbox", { name: "Message text" }),
    ).toBeNull();
    await userEvent.click(
      dialog.getByRole("button", { name: "Reaction match" }),
    );
    await userEvent.click(body.getByRole("option", { name: "Is not" }));
    await userEvent.click(
      dialog.getByRole("button", { name: "Choose reaction" }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("excluded_emoji");
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent(`custom:${"55".repeat(16)}`);
    await userEvent.click(dialog.getByRole("tab", { name: "Form" }));
    await userEvent.click(
      dialog.getByRole("button", { name: "Choose reaction" }),
    );
    const preset = await body.findByRole("button", { name: "React ✅" });
    await expect(preset.getBoundingClientRect().top).toBeGreaterThanOrEqual(0);
    await userEvent.click(preset);
    await expect(
      dialog.getByRole("button", { name: "Choose reaction" }),
    ).toHaveTextContent("✅");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("✅");
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
    await expect(
      canvas.getByRole("heading", { name: "Reply to deploy" }),
    ).toBeVisible();
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
<Story
  name="An unsuitable metadata home shows a toast before creating a pending request"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create workflow" }),
    );
    const element = await body.findByRole("dialog", {
        name: "Create workflow",
      }),
      dialog = within(element);
    await userEvent.click(
      dialog.getByRole("button", { name: "Edit workflow name" }),
    );
    await userEvent.type(
      dialog.getByRole("textbox", { name: "Name" }),
      "Safe draft",
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow name" }),
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Save workflow" }),
    );
    await expect(
      await body.findByText("Choose a metadata home", { exact: true }),
    ).toBeVisible();
    await expect(
      dialog.getByRole("button", { name: "Save workflow" }),
    ).toBeEnabled();
    await expect(element.querySelector(".store-notice")).toBeNull();
    await expect(dialog.getByText("Safe draft", { exact: true })).toBeVisible();
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
  }}>{#snippet template()}<WorkflowsFixture noHome />{/snippet}</Story
>
<Story
  name="Loading cards represent workflow tiles controls and metadata"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(await canvas.findByRole("status")).toHaveTextContent(
      "Loading workflows",
    );
    const cards = canvasElement.querySelectorAll(".workflow-card");
    await expect(cards).toHaveLength(3);
    for (const card of cards) {
      await expect(card.querySelector(".workflow-card__tiles")).not.toBeNull();
      await expect(
        card.querySelector(".workflow-card__controls"),
      ).not.toBeNull();
      await expect(card.querySelector("footer")).not.toBeNull();
    }
  }}>{#snippet template()}<WorkflowsFixture loading />{/snippet}</Story
>
