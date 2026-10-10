<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects review detail",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function openReviewPage(canvasElement: HTMLElement, project = false) {
    const c = within(canvasElement);
    if (project) await c.findByRole("button", { name: "Edit project" });
    await userEvent.click(await c.findByRole("tab", { name: "Reviews" }));
    await userEvent.click(
      await c.findByRole("button", { name: /Open Revision open/ }),
    );
    const detail = await c.findByRole("article", { name: "Review details" });
    await expect(
      c.queryByRole("region", { name: "Project reviews" }),
    ).toBeNull();
    await expect(c.queryByRole("list", { name: "Review threads" })).toBeNull();
    await expect(
      c.queryByRole("complementary", { name: "Review discussion" }),
    ).toBeNull();
    await expect(detail.getBoundingClientRect().width).toBeGreaterThan(
      canvasElement
        .querySelector(".store-application__body-regions")!
        .getBoundingClientRect().width * 0.95,
    );
    await expect(
      within(detail).getByRole("group", { name: "Review comment" }),
    ).toBeVisible();
    await userEvent.click(
      within(detail).getByRole("button", { name: "Back to reviews" }),
    );
    await expect(c.getByRole("list", { name: "Review threads" })).toBeVisible();
  }
</script>

<Story
  name="Directory review opens a full page"
  play={async ({ canvasElement }) => openReviewPage(canvasElement)}
/>
<Story
  name="Selected project review opens a full page"
  args={{ projectDetail: true }}
  play={async ({ canvasElement }) => openReviewPage(canvasElement, true)}
/>
