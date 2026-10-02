<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
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
  args={{ initialTab: "repositories" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByLabelText("Repository ID", { exact: true });
    await userEvent.type(input, "43".repeat(16));
    await userEvent.click(
      canvas.getByRole("button", { name: "Link repository" }),
    );
    await expect(
      canvas.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Saved");
  }}
/>
<Story
  name="Hierarchy and discussion"
  args={{ selectTask: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("list", { name: "Project tasks" }),
    ).toHaveTextContent("Subtask of Storage work");
    await userEvent.click(
      canvas.getByRole("button", { name: "Open task Browser work" }),
    );
    const task = canvasElement.querySelector<HTMLElement>(
      ".community-projects__task-detail",
    )!;
    const taskHeader = task.querySelector<HTMLElement>(
      ".community-projects__task-detail-header",
    )!;
    await expect(
      canvas.queryByRole("complementary", { name: "Task discussion" }),
    ).toBeNull();
    const comments = within(task).getByRole("list", { name: "Task comments" });
    const activity = within(task).getByRole("button", { name: "Activity" });
    const composer = within(task).getByRole("group", { name: /^Comment$/ });
    await expect(activity).toHaveAttribute("aria-expanded", "true");
    await expect(composer.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      comments.getBoundingClientRect().bottom,
    );
    await userEvent.click(activity);
    await waitFor(() => expect(comments).not.toBeVisible());
    await expect(composer).toBeVisible();
    await userEvent.click(activity);
    await expect(
      taskHeader.firstElementChild!.getBoundingClientRect().left -
        task.getBoundingClientRect().left,
    ).toBeLessThanOrEqual(8);
    await expect(
      canvas.getByText("Threaded reply", { exact: true }),
    ).toBeVisible();
    const comment = canvas.getByRole("listitem", { name: /^Comment 3131/ });
    await userEvent.hover(comment.querySelector(".community-message")!);
    await waitFor(() =>
      expect(comment.querySelector(".community-message")).toHaveAttribute(
        "data-actions-visible",
      ),
    );
    await userEvent.click(
      within(comment).getByRole("button", { name: "Reply" }),
    );
    await userEvent.type(
      within(canvas.getByRole("group", { name: /^Comment$/ })).getByRole(
        "textbox",
      ),
      "A new reply",
    );
    const editor = within(composer).getByRole("textbox");
    await userEvent.click(activity);
    await waitFor(() =>
      expect(
        within(task).queryByRole("list", { name: "Task comments" }),
      ).toBeNull(),
    );
    await expect(editor).toBeVisible();
    await expect(editor).toHaveTextContent("A new reply");
    await expect(within(composer).getByRole("textbox")).toBe(editor);
    await userEvent.click(activity);
    await userEvent.click(canvas.getByRole("button", { name: "Add comment" }));
    await expect(
      canvas.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Saved");
    await userEvent.click(canvas.getByRole("button", { name: "Edit task" }));
    const dialog = within(document.body).getByRole("dialog", {
      name: "Edit task",
    });
    await expect(within(dialog).getByLabelText("Task state")).toHaveValue(
      "blocked",
    );
    await expect(within(dialog).getByLabelText("Parent task")).toHaveAttribute(
      "title",
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
      within(canvas.getByRole("group", { name: /^Comment$/ })).getByRole(
        "textbox",
      ),
      "A retained command",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Add comment" }));
    await expect(
      canvas.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Outcome unknown");
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
    await expect(
      canvas.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Saved");
  }}
/>
<Story
  name="Read only"
  args={{ writable: false, initialTab: "repositories" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.queryByRole("button", { name: "New task" })).toBeNull();
    await expect(
      canvas.queryByLabelText("Comment", { exact: true }),
    ).toBeNull();
    await expect(canvas.getByText("Separate access required")).toBeVisible();
  }}
/>

<Story
  name="Native threaded discussion"
  args={{ threadFragments: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const comments = canvas.getByRole("list", { name: "Task comments" });
    await expect(
      within(comments).getByText("Reply survives removal"),
    ).toBeVisible();
    await expect(within(comments).getByText(/earlier page/)).toBeVisible();
    await expect(within(comments).getByText("Comment removed")).toBeVisible();
    const root = within(comments).getByRole("listitem", {
      name: /^Comment 3131/,
    });
    await userEvent.hover(root.querySelector(".community-message")!);
    await waitFor(() =>
      expect(root.querySelector(".community-message")).toHaveAttribute(
        "data-actions-visible",
      ),
    );
    await userEvent.click(
      within(root).getByRole("button", { name: "More message actions" }),
    );
    await userEvent.click(
      within(document.body).getByRole("menuitem", { name: "Edit comment" }),
    );
    await waitFor(() =>
      expect(document.body.style.pointerEvents).not.toBe("none"),
    );
    await expect(
      canvas.getByRole("button", { name: "Save comment" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("group", { name: "Editing message" }),
    ).toHaveTextContent("First discussion");
    await expect(
      within(canvas.getByRole("group", { name: /^Comment$/ })).getByRole(
        "textbox",
      ),
    ).toHaveTextContent("First discussion");
    await userEvent.click(canvas.getByRole("button", { name: "Save comment" }));
    await expect(
      canvas.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Saved");
  }}
/>
