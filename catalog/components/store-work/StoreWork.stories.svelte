<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreWorkFixture from "../../fixtures/StoreWorkFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects and tasks",
    component: StoreWorkFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Repository links"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByLabelText("Repository ID", { exact: true });
    await userEvent.type(input, "43".repeat(16));
    await userEvent.click(
      canvas.getByRole("button", { name: "Link repository" }),
    );
    await expect(canvas.getByRole("status")).toHaveTextContent("Saved");
  }}
/>
<Story
  name="Hierarchy and discussion"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("list", { name: "Project tasks" }),
    ).toHaveTextContent("Subtask of Storage work");
    await expect(
      canvas.getByText("Threaded reply", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getAllByRole("button", { name: "Reply" })[0]!);
    await userEvent.type(
      canvas.getByLabelText("Comment", { exact: true }),
      "A new reply",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Add comment" }));
    await expect(canvas.getByRole("status")).toHaveTextContent("Saved");
    await userEvent.click(canvas.getByRole("button", { name: "Edit task" }));
    await expect(canvas.getByLabelText("Task state")).toHaveValue("blocked");
    await expect(canvas.getByLabelText("Parent task")).toHaveValue(
      "21".repeat(16),
    );
  }}
/>
<Story
  name="Unknown outcome"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.type(
      canvas.getByLabelText("Comment", { exact: true }),
      "A retained command",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Add comment" }));
    await expect(canvas.getByRole("status")).toHaveTextContent(
      "Outcome unknown",
    );
    await expect(
      canvas.getByRole("button", { name: "Add comment" }),
    ).toBeDisabled();
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh projects" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Check task command" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Check task command" }),
    );
    await expect(canvas.getByRole("status")).toHaveTextContent("Saved");
  }}
/>
<Story
  name="Read only"
  args={{ writable: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.queryByRole("button", { name: "New task" })).toBeNull();
    await expect(
      canvas.queryByLabelText("Comment", { exact: true }),
    ).toBeNull();
    await expect(canvas.getByText("Separate access required")).toBeVisible();
  }}
/>
