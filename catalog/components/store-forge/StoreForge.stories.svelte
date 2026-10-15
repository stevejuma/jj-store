<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import ForgeFixture from "../../fixtures/ForgeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Pull requests",
    component: ForgeFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Edit description and inspect CI logs"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Edit description" }),
    );
    const description = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "Updated explanation");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save PR description" }),
    );
    await expect(await canvas.findByText("Updated explanation")).toBeVisible();
    await expect(
      canvas.queryByRole("textbox", { name: "Markdown editor" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Checks and runs" }),
    );
    await expect(
      await canvas.findByRole("region", { name: "CI checks" }),
    ).toHaveTextContent("running");
    await userEvent.click(canvas.getByRole("button", { name: "Jobs" }));
    await userEvent.click(
      await canvas.findByRole("button", { name: "View logs" }),
    );
    await expect(await canvas.findByLabelText("Job logs")).toHaveTextContent(
      "Assertion failed",
    );
  }}
/>
<Story
  name="Failed save keeps draft and publishes a toast"
  args={{ failure: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Edit description" }),
    );
    const description = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(description);
    await userEvent.keyboard("{Control>}a{/Control}{Backspace}");
    await userEvent.type(description, "Keep this draft");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save PR description" }),
    );
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "The forge refused this description update",
      ),
    ).toBeVisible();
    await expect(description).toHaveTextContent("Keep this draft");
    await userEvent.click(
      canvas.getByRole("button", { name: "Preview description" }),
    );
    await expect(canvas.getByText("Keep this draft")).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Edit description" }),
    );
    await expect(
      canvas.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Keep this draft");
  }}
/>

<Story
  name="Markdown description and merge method menu"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("heading", { name: "Review summary" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("link", { name: "review guidance" }),
    ).toHaveAttribute("href", "https://example.com/review");
    await userEvent.click(
      canvas.getByRole("button", { name: "Edit description" }),
    );
    const composer = canvas.getByRole("group", { name: "PR description" });
    await expect(
      within(composer).getByRole("textbox", { name: "Markdown editor" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Preview description" }),
    );
    await expect(
      canvas.getByRole("heading", { name: "Review summary" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Merge method: Squash" }),
    );
    const menu = within(canvasElement.ownerDocument.body);
    await expect(
      menu.getByRole("menuitemradio", { name: "Squash" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.keyboard("{End}");
    await expect(
      menu.getByRole("menuitemradio", { name: "Rebase" }),
    ).toHaveFocus();
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByRole("button", { name: "Merge method: Rebase" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Provider-supported merge methods"
  args={{ mergeMethods: ["rebase"] }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Merge method: Rebase" }),
    );
    const menu = within(canvasElement.ownerDocument.body);
    await expect(menu.getAllByRole("menuitemradio")).toHaveLength(1);
    await expect(
      menu.getByRole("menuitemradio", { name: "Rebase" }),
    ).toHaveAttribute("aria-checked", "true");
    await userEvent.keyboard("{Escape}");
    await expect(
      canvas.getByRole("button", { name: "Merge method: Rebase" }),
    ).toHaveFocus();
  }}
/>
