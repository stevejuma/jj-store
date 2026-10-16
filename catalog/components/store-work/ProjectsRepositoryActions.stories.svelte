<script module lang="ts">
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  import { homeB } from "../../fixtures/projects-hub-fixture.js";
  const { Story } = defineMeta({
    title: "JJ Store/Project repository actions",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
    beforeEach: async () => {
      await views.projects();
    },
  });
  async function actions(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      body = within(document.body);
    await c.findByRole("button", { name: "Edit project" });
    await userEvent.click(c.getByRole("tab", { name: "Repositories" }));
    await expect(c.queryByRole("combobox", { name: "Repository" })).toBeNull();
    const link = c.getByRole("button", {
      name: "Link repository",
    });
    await expect(link.querySelector('svg[aria-hidden="true"]')).not.toBeNull();
    await userEvent.click(link);
    const dialog = within(
      await body.findByRole("dialog", { name: "Link repository" }),
    );
    const picker = dialog.getByRole("combobox", {
      name: "Repository",
    });
    await userEvent.click(picker);
    // Active links are excluded; a retired link remains eligible for restoration.
    await expect(
      body.queryByRole("option", { name: /Parser repository/ }),
    ).toBeNull();
    await userEvent.click(body.getByRole("option", { name: /UI repository/ }));
    await expect(picker).toHaveAttribute("title", homeB);
    await userEvent.click(
      dialog.getByRole("switch", { name: "Link by repository ID" }),
    );
    const input = dialog.getByLabelText("Repository ID", { exact: true });
    await expect(input).toHaveValue(homeB);
    await expect(
      input.closest(".community-settings__option-group"),
    ).not.toBeNull();
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(link).toHaveFocus());
    const add = c.getByRole("button", { name: "Add" });
    await expect(c.queryByRole("button", { name: "New task" })).toBeNull();
    await add.focus();
    await userEvent.keyboard("{Enter}");
    const task = await body.findByRole("menuitem", {
      name: "Add task",
    });
    await expect(task.querySelector('svg[aria-hidden="true"]')).not.toBeNull();
    await expect(
      body.queryByRole("menuitem", { name: "Add local repository" }),
    ).toBeNull();
    await waitFor(() => expect(task).toHaveFocus());
    await userEvent.keyboard("{Enter}");
    const create = await body.findByRole("dialog", {
      name: "Create task",
    });
    await waitFor(() =>
      expect(create.contains(document.activeElement)).toBe(true),
    );
    await expect(create.scrollWidth).toBeLessThanOrEqual(
      create.clientWidth + 1,
    );
    await userEvent.click(within(create).getByRole("button", { name: "Done" }));
    await waitFor(() => expect(add).toHaveFocus());
    await userEvent.click(link);
    await expect(
      body.getByLabelText("Repository ID", { exact: true }),
    ).toHaveValue(homeB);
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(document.body.style.pointerEvents).not.toBe("none"),
    );
  }
</script>

<Story
  name="Project add menu and link dialog"
  args={{ projectDetail: true }}
  play={async ({ canvasElement }) => actions(canvasElement)}
/>
<Story
  name="Mobile project repository actions"
  args={{ projectDetail: true, mobile: true }}
  play={async ({ canvasElement }) => actions(canvasElement)}
/>
<Story
  name="Read only project actions"
  args={{ projectDetail: true, readonly: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("tab", { name: "Repositories" });
    await userEvent.click(c.getByRole("tab", { name: "Repositories" }));
    await expect(c.queryByRole("button", { name: "Add" })).toBeNull();
    await expect(
      c.queryByRole("button", { name: "Link repository" }),
    ).toBeNull();
  }}
/>
