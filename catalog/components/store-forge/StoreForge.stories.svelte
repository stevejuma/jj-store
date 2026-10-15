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
    const description = await canvas.findByRole("textbox", {
      name: "Description",
    });
    await userEvent.clear(description);
    await userEvent.type(description, "Updated explanation");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save PR description" }),
    );
    await expect(description).toHaveValue("Updated explanation");
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
    const description = await canvas.findByRole("textbox", {
      name: "Description",
    });
    await userEvent.clear(description);
    await userEvent.type(description, "Keep this draft");
    await userEvent.click(
      canvas.getByRole("button", { name: "Save PR description" }),
    );
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        "The forge refused this description update",
      ),
    ).toBeVisible();
    await expect(description).toHaveValue("Keep this draft");
  }}
/>
