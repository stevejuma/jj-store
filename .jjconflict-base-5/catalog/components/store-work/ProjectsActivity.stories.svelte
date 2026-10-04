<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects activity",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Inset skeletons across all main tabs"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Repositories",
      "Activity",
    ]) {
      await userEvent.click(c.getByRole("tab", { name }));
      await waitFor(() =>
        expect(
          canvasElement.querySelector(
            '.store-projects__loading [role="status"]',
          ),
        ).toBeVisible(),
      );
      await expect(c.queryByText(/Checking home repositories/)).toBeNull();
      await expect(c.queryByText(/Loading project…/)).toBeNull();
      await expect(
        c.queryByText(
          /No accessible|No matching|No reviews yet|No repository activity yet/,
        ),
      ).toBeNull();
      for (const loading of canvasElement.querySelectorAll<HTMLElement>(
        ".store-projects__loading",
      )) {
        const style = getComputedStyle(loading);
        await expect(
          Number.parseFloat(style.paddingLeft),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingTop),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingRight),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingBottom),
        ).toBeGreaterThanOrEqual(16);
      }
    }
  }}
/>

<Story
  name="Discovery skeleton before home reads"
  args={{ loading: true, discoveryLoading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("status", { name: "Loading projects" })).toBeVisible(),
    );
    await expect(c.queryByText(/Checking home repositories/)).toBeNull();
  }}
/>

<Story
  name="Bootstrap skeleton before discovery"
  args={{ loading: true, bootstrapLoading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Repositories",
      "Activity",
    ]) {
      await userEvent.click(c.getByRole("tab", { name }));
      await waitFor(() =>
        expect(
          c.getByRole("status", { name: `Loading ${name.toLowerCase()}` }),
        ).toBeVisible(),
      );
      await expect(
        c.queryByText(
          /No accessible|No matching|No reviews yet|No repository activity yet|Checking home repositories/,
        ),
      ).toBeNull();
    }
  }}
/>

<Story
  name="Task details skeleton"
  args={{ loading: true, detail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("status", { name: "Loading task details" }),
      ).toBeVisible(),
    );
    await expect(c.queryByText("Loading project…")).toBeNull();
    await expect(c.queryByRole("article", { name: "Task details" })).toBeNull();
  }}
/>

<Story
  name="Loaded projects survive refresh"
  args={{ deferredRefresh: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const target = await c.findByRole("button", {
      name: "Open project Native release",
    });
    await userEvent.click(
      c.getByRole("button", { name: "Pause project refresh" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Refresh projects" }),
      ).toBeDisabled(),
    );
    await expect(target).toBeVisible();
    await expect(
      c.queryByRole("status", { name: /Loading projects/ }),
    ).toBeNull();
    await expect(c.queryByText(/Checking home repositories/)).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Finish project refresh" }),
    );
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Refresh projects" })).toBeEnabled(),
    );
    await expect(
      c.getByRole("button", { name: "Open project Native release" }),
    ).toBe(target);
    for (const name of ["Tasks", "Reviews", "Repositories", "Activity"]) {
      await userEvent.click(c.getByRole("tab", { name }));
      await waitFor(() =>
        expect(
          c.getByRole("button", { name: "Refresh projects" }),
        ).toBeEnabled(),
      );
      const rows = [
        ...canvasElement.querySelectorAll<HTMLElement>(
          '[role="tabpanel"] [role="list"]',
        ),
      ];
      expect(rows.length).toBeGreaterThan(0);
      await userEvent.click(
        c.getByRole("button", { name: "Pause project refresh" }),
      );
      await waitFor(() =>
        expect(
          c.getByRole("button", { name: "Refresh projects" }),
        ).toBeDisabled(),
      );
      for (const row of rows) await expect(row).toBeVisible();
      await expect(
        canvasElement.querySelector(".store-projects__loading"),
      ).toBeNull();
      await userEvent.click(
        c.getByRole("button", { name: "Finish project refresh" }),
      );
      await waitFor(() =>
        expect(
          c.getByRole("button", { name: "Refresh projects" }),
        ).toBeEnabled(),
      );
    }
  }}
/>

<Story
  name="Task detail retains composer during refresh"
  args={{ detail: true, deferredRefresh: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const detail = await c.findByRole("article", { name: "Task details" });
    const editor = within(
      c.getByRole("group", { name: /^Comment$/ }),
    ).getByRole("textbox");
    await userEvent.type(editor, "Keep this draft while refreshing");
    await userEvent.click(
      c.getByRole("button", { name: "Pause project refresh" }),
    );
    await expect(detail).toBeVisible();
    await expect(editor).toHaveTextContent("Keep this draft while refreshing");
    await expect(
      c.queryByRole("status", { name: "Loading task details" }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Finish project refresh" }),
    );
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Refresh projects" })).toBeEnabled(),
    );
    await expect(
      within(c.getByRole("group", { name: /^Comment$/ })).getByRole("textbox"),
    ).toBe(editor);
  }}
/>

<Story
  name="Shared repository activity and agent context"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("tab", { name: "Activity" }));
    await expect(
      await c.findByRole("list", { name: "Repository activity" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Agent context" }));
    await expect(
      c.getByRole("log", { name: "Agent conversation activity" }),
    ).toHaveTextContent("Updated the parser");
    await userEvent.click(c.getByRole("button", { name: "Search activity" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Search activity" }),
      "no matching revision{Enter}",
    );
    await expect(await c.findByText("No matching activity")).toBeVisible();
  }}
/>
