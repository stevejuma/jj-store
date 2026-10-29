<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StoreWorkFixture from "../../fixtures/StoreWorkFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects and tasks",
    component: StoreWorkFixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-work/StoreWork.svelte",
      },
      layout: "fullscreen",
    },
  });
</script>

<Story
  name="Repository links"
  args={{ initialTab: "repositories" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const body = within(document.body);
    await expect(
      canvas.queryByLabelText("Repository ID", { exact: true }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Link repository" }),
    );
    let dialog = await body.findByRole("dialog", {
      name: "Link repository",
    });
    const input = within(dialog).getByLabelText("Repository ID", {});
    await userEvent.type(input, "43".repeat(16));
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Link repository" }),
    );
    dialog = await body.findByRole("dialog", {
      name: "Link repository",
    });
    await expect(
      within(dialog).getByLabelText("Repository ID", { exact: true }),
    ).toHaveValue("43".repeat(16));
    await userEvent.click(
      within(dialog).getByRole("button", {
        name: "Link repository",
      }),
    );
    await waitFor(() =>
      expect(
        body.queryByRole("dialog", { name: "Link repository" }),
      ).toBeNull(),
    );
    await expect(
      canvas.getAllByRole("button", { name: "Retire link" }),
    ).toHaveLength(2);
    await expect(
      canvas.queryByRole("status", { name: "Work command status" }),
    ).toBeNull();
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
    const parent = within(task).getByRole("button", { name: /Storage work/ });
    await expect(parent).not.toHaveTextContent("Parent:");
    await expect(parent).toHaveAttribute("data-variant", "secondary");
    await expect(
      parent.querySelector('[data-status="in_progress"]'),
    ).not.toBeNull();
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
      canvas.queryByRole("status", { name: "Work command status" }),
    ).toBeNull();
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
  name="Unknown repository link retains draft and receipt recovery"
  args={{ initialTab: "repositories", unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "Link repository" }));
    const dialog = within(
      await body.findByRole("dialog", { name: "Link repository" }),
    );
    const input = dialog.getByLabelText("Repository ID", { exact: true });
    await userEvent.type(input, "43".repeat(16));
    await userEvent.click(
      dialog.getByRole("button", { name: "Link repository" }),
    );
    await expect(
      dialog.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Outcome unknown");
    await expect(input).toHaveValue("43".repeat(16));
    await expect(input).toBeDisabled();
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await userEvent.click(
      c.getByRole("button", { name: "Check task command" }),
    );
    await expect(
      c.getAllByRole("button", { name: "Retire link" }),
    ).toHaveLength(2);
    await userEvent.click(c.getByRole("button", { name: "Link repository" }));
    await expect(
      body.getByLabelText("Repository ID", { exact: true }),
    ).toHaveValue("");
    await userEvent.keyboard("{Escape}");
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
      canvas.queryByRole("status", { name: "Work command status" }),
    ).toBeNull();
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
      canvas.queryByRole("status", { name: "Work command status" }),
    ).toBeNull();
  }}
/>

<Story
  name="Task comment reactions and action surfaces"
  args={{ threadFragments: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const root = canvas.getByRole("listitem", { name: /^Comment 3131/ });
    await userEvent.hover(root.querySelector(".community-message")!);
    await waitFor(() =>
      expect(root.querySelector(".community-message")).toHaveAttribute(
        "data-actions-visible",
      ),
    );
    await userEvent.click(
      within(root).getByRole("button", { name: "Add reaction" }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    const reaction = await within(root).findByRole("button", {
      name: /1 reaction with Lapis/,
    });
    await expect(reaction).toBeVisible();
    await userEvent.click(reaction);
    await waitFor(() =>
      expect(
        within(root).queryByRole("button", { name: /1 reaction with Lapis/ }),
      ).toBeNull(),
    );
    const removed = canvas.getByRole("listitem", { name: /^Comment 3333/ });
    await expect(
      within(removed).queryByRole("button", { name: "Add reaction" }),
    ).toBeNull();
    await userEvent.hover(root.querySelector(".community-message")!);
    await waitFor(() =>
      expect(root.querySelector(".community-message")).toHaveAttribute(
        "data-actions-visible",
      ),
    );
    await userEvent.click(
      within(root).getByRole("button", { name: "More message actions" }),
    );
    const edit = await body.findByRole("menuitem", { name: "Edit comment" });
    const copy = body.getByRole("menuitem", { name: /^Copy$/ });
    await expect(
      Math.abs(
        copy.querySelector("svg")!.getBoundingClientRect().left -
          edit.querySelector("svg")!.getBoundingClientRect().left,
      ),
    ).toBeLessThanOrEqual(1);
    await userEvent.keyboard("{Escape}");
    const editTask = canvas.getByRole("button", { name: /^Edit task$/ });
    await expect(getComputedStyle(editTask).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    // The projects page supplies Add subtask; both buttons and native CSS :hover
    // are asserted with real pointer input in task-reactions.spec.ts.
  }}
/>
<Story
  name="Task reactions during background reads"
  args={{ backgroundRead: true }}
  play={async ({ canvasElement }) => {
    const root = within(canvasElement).getByRole("listitem", {
      name: /^Comment 3131/,
    });
    await userEvent.hover(root.querySelector(".community-message")!);
    await waitFor(() =>
      expect(root.querySelector(".community-message")).toHaveAttribute(
        "data-actions-visible",
      ),
    );
    const picker = within(root).getByRole("button", { name: "Add reaction" });
    await expect(picker).toBeEnabled();
    await userEvent.click(picker);
    await userEvent.click(
      await within(document.body).findByRole("button", { name: ":lapis:" }),
    );
    await expect(
      await within(root).findByRole("button", {
        name: /1 reaction with Lapis/,
      }),
    ).toBeVisible();
  }}
/>
<Story
  name="Read only task reactions"
  args={{ writable: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const root = c.getByRole("listitem", { name: /^Comment 3131/ });
    await userEvent.hover(root.querySelector(".community-message")!);
    await expect(
      within(root).queryByRole("button", { name: "Add reaction" }),
    ).toBeNull();
    await expect(c.queryByRole("button", { name: /^Edit task$/ })).toBeNull();
  }}
/>
