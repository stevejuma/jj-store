<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { playOwnershipLayouts } from "./projects-layout-play.js";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  import {
    playReadOnlyTask,
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
  name="Unsupported reviews empty state"
  args={{ localReviews: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    const home = await c.findByRole("region", {
      name: "Reviews in UI repository",
    });
    const empty = within(home)
      .getByText("Reviews unavailable")
      .closest('[data-slot="empty"]');
    await expect(empty).toBeVisible();
    await expect(
      empty?.querySelector('[data-ui-part="empty-media"] svg'),
    ).toBeVisible();
    await expect(empty).toHaveTextContent(
      "Reviews are not supported for local JJ repositories.",
    );
    await expect(within(home).queryByRole("alert")).toBeNull();
    await expect(
      within(home).queryByRole("button", { name: "Retry reviews" }),
    ).toBeNull();
    await expect(
      within(
        c.getByRole("region", { name: "Reviews in Parser repository" }),
      ).getByRole("list", { name: "Reviews" }).firstChild,
    ).not.toBeNull();
  }}
/>
<Story
  name="Review load failure retains retry"
  args={{ failedReviews: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    const home = await c.findByRole("region", {
      name: "Reviews in Parser repository",
    });
    await expect(within(home).getByRole("alert")).toHaveTextContent(
      "Reviews temporarily unavailable",
    );
    await userEvent.click(
      within(home).getByRole("button", { name: "Retry reviews" }),
    );
    await expect(await within(home).findByRole("alert")).toHaveTextContent(
      "Reviews temporarily unavailable",
    );
  }}
/>

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
    await expect(
      within(c.getByRole("article", { name: "Task details" })).getByRole(
        "region",
        { name: "Task activity" },
      ),
    ).toHaveTextContent("First discussion");
    await expect(
      c.queryByRole("button", { name: "Open task discussion" }),
    ).toBeNull();
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
  play={playReadOnlyTask}
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
    const selected = body
      .getByRole("option", { name: /Storage work/ })
      .querySelector('[aria-label="Selected"]')!;
    await expect(selected).not.toBeNull();
    await expect(selected.getBoundingClientRect().width).toBeLessThanOrEqual(
      18,
    );
    await expect(selected.closest('[data-ui-part="shortcut"]')).toBeNull();
    await expect(selected.querySelector("circle")).not.toBeNull();
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
    ).toEqual([
      "Activity",
      "Projects",
      "Tasks",
      "Reviews",
      "Agents",
      "Repositories",
    ]);
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
    expect(
      c
        .getByRole("heading", { name: /^Projects$/ })
        .parentElement?.querySelector("svg.lucide-folder-root"),
    ).not.toBeNull();
    expect(
      c
        .getByRole("button", { name: /^Projects$/ })
        .querySelector("svg.lucide-folder-root"),
    ).not.toBeNull();
    expect(trigger.querySelector("svg.lucide-folder-kanban")).not.toBeNull();
    await userEvent.click(trigger);
    const search = await body.findByPlaceholderText("Search your projects");
    const options = within(body.getByRole("listbox"));
    await expect(
      options.getByRole("option", { name: "Native release" }),
    ).toBeVisible();
    await expect(
      options.getByRole("option", { name: "Shared code project" }),
    ).toBeVisible();
    await expect(
      options.queryByRole("option", { name: "Parser repository" }),
    ).toBeNull();
    await userEvent.type(search, "Parser repository");
    await expect(body.getByText("No matching your projects")).toBeVisible();
    await userEvent.clear(search);
    await userEvent.type(search, "Shared code project");
    await expect(
      within(body.getByRole("listbox")).getAllByRole("option", {
        name: "Shared code project",
      }),
    ).toHaveLength(1);
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(trigger).toHaveFocus());
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Repository navigation" }),
      ).getByRole("button", { name: "Repositories" }),
    );
    await expect(
      within(body.getByRole("listbox")).getByRole("option", {
        name: "Parser repository",
      }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Community card layouts and Mine Other groups"
  play={playOwnershipLayouts}
/>
