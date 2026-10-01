<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { playOwnershipLayouts } from "./projects-layout-play.js";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  import {
    expectCommunityDisclosures,
    expectReviewRowUsesWidth,
    expectTaskTitleRow,
  } from "../../fixtures/projects-hub-layout.js";
  const { Story } = defineMeta({
    title: "JJ Store/Projects hub",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Create project selects writable home and retains request"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await waitFor(() =>
      expect(c.getByRole("button", { name: "New project" })).toBeVisible(),
    );
    await userEvent.click(c.getByRole("button", { name: "New project" }));
    const dialog = within(body.getByRole("dialog", { name: "Create project" }));
    const home = dialog.getByLabelText("Home repository");
    await expect(home).toHaveTextContent("Parser repository");
    await userEvent.click(home);
    await expect(
      dialog
        .getByLabelText("Project title")
        .closest(".community-settings__option-group"),
    ).not.toBeNull();
    await expect(
      body.queryByRole("option", { name: /UI repository/ }),
    ).toBeNull();
    await userEvent.click(
      body.getByRole("option", { name: "Choose a writable home" }),
    );
    await expect(dialog.queryByLabelText("Project title")).toBeNull();
    await userEvent.click(home);
    await userEvent.click(
      body.getByRole("option", { name: /Parser repository/ }),
    );
    await userEvent.type(
      dialog.getByLabelText("Project title"),
      "New native project",
    );
    await userEvent.click(
      dialog.getByRole("button", { name: "Create project" }),
    );
    await expect(
      dialog.getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Outcome unknown");
    await userEvent.click(dialog.getByRole("button", { name: "Done" }));
    await expect(
      c.getByRole("button", { name: "Retry task command" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Projects repositories and task navigation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Native release" }),
      ).toBeVisible(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Open project Native release" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open task Browser work" }),
      ).toBeVisible(),
    );
    await expect(
      c.queryByRole("heading", { name: "Native release" }),
    ).toBeNull();
    await expect(
      c.queryByText("Release across two repositories.", { exact: true }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Open task Browser work" }),
    );
    await expect(
      c.getByRole("article", { name: "Task details" }),
    ).toHaveTextContent("Browser work");
    await userEvent.click(c.getByRole("button", { name: "Activity" }));
    await expect(
      c.getByRole("region", { name: "Task discussion" }),
    ).toHaveTextContent("First discussion");
  }}
/>
<Story
  name="Failed home retains successful results"
  args={{ failedHome: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Native release" }),
      ).toBeVisible(),
    );
    await expect(c.getByRole("alert")).toHaveTextContent("Home unavailable");
  }}
/>
<Story
  name="Read only task"
  args={{ detail: true, readonly: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("article", { name: "Task details" })).toBeVisible(),
    );
    const detail = c.getByRole("article", { name: "Task details" });
    await expectCommunityDisclosures(detail);
    await expectTaskTitleRow(detail);
    await expect(
      c.getByRole("navigation", { name: "breadcrumb" }),
    ).toBeVisible();
    await expect(c.queryByRole("button", { name: "Edit task" })).toBeNull();
    await expect(c.queryByRole("textbox", { name: "Comment" })).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Parent: Storage work" }),
    );
    const subtasks = c.getByRole("region", { name: "Subtasks" });
    await userEvent.click(
      within(subtasks).getByRole("button", { name: "Subtasks" }),
    );
    await waitFor(() =>
      expect(
        within(subtasks).getByRole("button", {
          name: "Open task Browser work",
        }),
      ).toBeVisible(),
    );
    await expect(
      within(subtasks).getByRole("list").getBoundingClientRect().left,
    ).toBeCloseTo(
      within(subtasks)
        .getByRole("button", { name: "Subtasks" })
        .getBoundingClientRect().left,
      0,
    );
  }}
/>
<Story
  name="Task edit and retained unknown request"
  args={{ detail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Edit task" })).toBeVisible(),
    );
    await userEvent.click(c.getByRole("button", { name: "Edit task" }));
    const dialog = body.getByRole("dialog");
    const composer = within(dialog).getByRole("group", {
      name: "Task description",
    });
    const description = within(dialog).getByText(
      "Add context and acceptance criteria with Markdown.",
    );
    await expect(composer.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      description.getBoundingClientRect().bottom,
    );
    const parent = within(dialog).getByRole("combobox", {
      name: "Parent task",
    });
    await expect(parent).toHaveAttribute("title", "21".repeat(16));
    await userEvent.click(parent);
    await userEvent.type(body.getByPlaceholderText("Search tasks"), "Storage");
    await waitFor(() =>
      expect(body.getByRole("option", { name: /Storage work/ })).toBeVisible(),
    );
    await expect(
      body.getByRole("option", { name: /Storage work/ }),
    ).toHaveTextContent("In progress · Parser repository · 21212121");
    await expect(
      body.queryByRole("option", { name: /Browser work/ }),
    ).toBeNull();
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(parent).toHaveAttribute("title", "21".repeat(16));
    await expect(parent).toHaveFocus();
    await userEvent.clear(within(dialog).getByLabelText("Task title"));
    await userEvent.type(
      within(dialog).getByLabelText("Task title"),
      "Retained task",
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Save task" }),
    );
    await expect(
      within(dialog).getByRole("status", { name: "Work command status" }),
    ).toHaveTextContent("Outcome unknown");
    await userEvent.click(
      within(dialog).getByRole("button", { name: /^Close$/ }),
    );
    await expect(
      c.getByRole("button", { name: "Retry task command" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Mobile task detail"
  args={{ detail: true, mobile: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("article", { name: "Task details" })).toBeVisible(),
    );
  }}
/>

<Story
  name="Landing tabs sidebar hierarchy reviews and icon empty state"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Native release" }),
      ).toBeVisible(),
    );
    const view = within(c.getByRole("region", { name: "Projects and tasks" }));
    await expect(
      view.getAllByRole("tab").map((tab) => tab.textContent?.trim()),
    ).toEqual(["Activity", "Projects", "Tasks", "Reviews", "Repositories"]);
    const expandProject = c.getByRole("button", {
      name: "Expand Native release",
    });
    const projectRow = expandProject.parentElement!;
    await expect(
      projectRow.getBoundingClientRect().right -
        expandProject.getBoundingClientRect().right,
    ).toBeLessThan(8);
    const projectsSection = c
      .getByRole("button", { name: "Your projects" })
      .closest(".lc-community-navigation__section")!;
    await expect(getComputedStyle(projectsSection).marginBlockStart).toBe(
      "20px",
    );
    await userEvent.click(expandProject);
    await expect(
      within(
        c.getByRole("navigation", { name: "Repository navigation" }),
      ).getAllByRole("button", { name: "Parser repository" }),
    ).toHaveLength(2);
    await userEvent.click(view.getByRole("tab", { name: "Reviews" }));
    await userEvent.click(c.getByRole("button", { name: "Toggle details" }));
    await waitFor(() =>
      expect(view.getByRole("list", { name: "Reviews" })).toHaveTextContent(
        "Revision",
      ),
    );
    const reviewRow = view
      .getByRole("list", { name: "Reviews" })
      .querySelector("article")!;
    const reviewHeading = view.getByRole("button", {
      name: "Parser repository",
    });
    await expectReviewRowUsesWidth(reviewRow, reviewHeading);
    await userEvent.click(
      c.getByRole("button", { name: "Filters for reviews" }),
    );
    await userEvent.click(
      within(document.body).getByRole("combobox", {
        name: "Repository filter",
      }),
    );
    await userEvent.click(
      within(document.body).getByRole("option", { name: /UI repository/ }),
    );
    await userEvent.click(
      within(document.body).getByRole("button", { name: "Close filters" }),
    );
    await waitFor(() => expect(view.getByText("No reviews yet")).toBeVisible());
    const empty = view
      .getByText("No reviews yet")
      .closest('[data-ui-part="empty"]');
    await expect(
      empty?.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    await expect(view.queryByLabelText("Search reviews")).toBeNull();
  }}
/>

<Story
  name="Compact projects sharing a repository"
  args={{ sharedRepository: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Shared code project" }),
      ).toBeVisible(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Collapse left sidebar" }),
    );
    const trigger = c.getByRole("button", { name: "Your projects" });
    await userEvent.click(trigger);
    const search = await body.findByPlaceholderText("Search your projects");
    await userEvent.type(search, "Parser repository");
    await expect(
      within(body.getByRole("listbox")).getAllByRole("option", {
        name: "Parser repository",
      }),
    ).toHaveLength(1);
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(trigger).toHaveFocus());
  }}
/>

<Story
  name="Community card layouts and Mine Other groups"
  play={playOwnershipLayouts}
/>
