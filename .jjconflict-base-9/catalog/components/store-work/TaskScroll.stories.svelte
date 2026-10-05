<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Task scrolling",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    args: { detail: true, longTaskDetail: true },
    parameters: { layout: "fullscreen" },
  });
  async function fixedTaskChrome(
    canvasElement: HTMLElement,
    width: number,
    readonly = false,
  ) {
    const c = within(canvasElement);
    const task = await c.findByRole("article", { name: "Task details" });
    const host = task.closest<HTMLElement>(".store-projects")!;
    host.style.width = `${width}px`;
    host.style.height = "480px";
    const region = within(task).getByRole("region", { name: "Task content" });
    const viewport = region.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const header = task.querySelector("header")!;
    const composer = readonly
      ? null
      : within(task).getByRole("group", { name: /^Comment$/ });
    if (composer)
      await userEvent.type(
        within(composer).getByRole("textbox"),
        "Retain this draft while scrolling",
      );
    await waitFor(() =>
      expect(viewport.scrollHeight - viewport.clientHeight).toBeGreaterThan(
        100,
      ),
    );
    const headerBefore = header.getBoundingClientRect(),
      composerBefore = composer?.getBoundingClientRect();
    viewport.scrollTo(0, viewport.scrollHeight);
    await waitFor(() => expect(viewport.scrollTop).toBeGreaterThan(100));
    await expect(header.getBoundingClientRect().top).toBeCloseTo(
      headerBefore.top,
      0,
    );
    await expect(viewport.getBoundingClientRect().top).toBeGreaterThanOrEqual(
      header.getBoundingClientRect().bottom - 1,
    );
    if (composer && composerBefore) {
      await expect(composer.getBoundingClientRect().bottom).toBeCloseTo(
        composerBefore.bottom,
        0,
      );
      await expect(composer.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        host.getBoundingClientRect().bottom,
      );
      await expect(viewport.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        composer.getBoundingClientRect().top,
      );
      await expect(within(composer).getByRole("textbox")).toHaveTextContent(
        "Retain this draft while scrolling",
      );
      await userEvent.click(
        within(task).getByRole("button", { name: "Activity" }),
      );
      await expect(composer).toBeVisible();
      await expect(within(composer).getByRole("textbox")).toHaveTextContent(
        "Retain this draft while scrolling",
      );
    } else
      await expect(c.queryByRole("group", { name: /^Comment$/ })).toBeNull();
    await expect(task.scrollWidth).toBeLessThanOrEqual(task.clientWidth + 1);
  }
</script>

<Story
  name="Fixed header and composer"
  play={async ({ canvasElement }) => fixedTaskChrome(canvasElement, 950)}
/>
<Story
  name="Narrow fixed header and composer"
  args={{ mobile: true }}
  play={async ({ canvasElement }) => fixedTaskChrome(canvasElement, 380)}
/>
<Story
  name="Read only fixed header"
  args={{ readonly: true }}
  play={async ({ canvasElement }) => fixedTaskChrome(canvasElement, 380, true)}
/>
<Story
  name="Task content scroll survives returning"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const task = await c.findByRole("article", { name: "Task details" });
    const host = task.closest<HTMLElement>(".store-projects")!;
    host.style.height = "480px";
    const viewport = task.querySelector<HTMLElement>(
      '.community-projects__task-scroll [data-ui-part="scroll-area-viewport"]',
    )!;
    await userEvent.type(
      within(task).getByRole("textbox"),
      "Retain this draft when returning",
    );
    viewport.scrollTo(0, 400);
    await waitFor(() => expect(viewport.scrollTop).toBeCloseTo(400, 0));
    await userEvent.click(c.getByRole("button", { name: "Back to tasks" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Open task Browser work" }),
    );
    const reopened = await c.findByRole("article", { name: "Task details" });
    const restoredViewport = reopened.querySelector<HTMLElement>(
      '.community-projects__task-scroll [data-ui-part="scroll-area-viewport"]',
    )!;
    await waitFor(() => expect(restoredViewport.scrollTop).toBeCloseTo(400, 0));
    await expect(within(reopened).getByRole("textbox")).toHaveTextContent(
      "Retain this draft when returning",
    );
  }}
/>
