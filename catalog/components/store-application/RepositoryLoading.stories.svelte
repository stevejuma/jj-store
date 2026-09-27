<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";

  const { Story } = defineMeta({
    title: "JJ Store/Application loading",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { loading: true, initiallyCollapsed: false },
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });

  async function expectSkeleton(
    canvasElement: HTMLElement,
    label: string,
    layout: string,
  ) {
    const status = within(canvasElement).getByRole("status", { name: label });
    await expect(status).toHaveAttribute("aria-busy", "true");
    await expect(status).toHaveAttribute("data-layout", layout);
    await expect(
      status.querySelector('[data-ui-component="skeleton"]'),
    ).not.toBeNull();
  }
</script>

<Story
  name="Branches"
  args={{ initialPage: "branches" }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(canvasElement, "Loading branches", "bookmarks");
    await expect(
      canvasElement.querySelectorAll('[data-ui-part="column"]'),
    ).toHaveLength(1);
  }}
/>

<Story
  name="JJ Log"
  args={{ initialPage: "log" }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(canvasElement, "Loading JJ log", "graph");
    await expect(
      within(canvasElement).queryByRole("status", { name: "Loading branches" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Op Log"
  args={{ initialPage: "oplog" }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(canvasElement, "Loading operation log", "graph");
    await expect(
      canvasElement.querySelector(
        ".community-projects-repository-skeleton__status",
      ),
    ).not.toBeNull();
  }}
/>

<Story
  name="Files"
  args={{ initialPage: "files" }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(
      canvasElement,
      "Loading repository file tree",
      "file-tree",
    );
    await expectSkeleton(
      canvasElement,
      "Loading repository files",
      "directory",
    );
    await expect(
      within(canvasElement).getByRole("complementary", {
        name: "Repository file tree",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="File detail"
  args={{ initialPage: "files", loading: false, detailLoading: true }}
  play={async ({ canvasElement }) => {
    const status = within(canvasElement).getByRole("status", {
      name: "Loading README.md",
    });
    await expect(status).toBeVisible();
    await expect(
      status.querySelectorAll('[data-ui-component="skeleton"]').length,
    ).toBeGreaterThan(3);
  }}
/>

<Story
  name="JJ Log detail"
  args={{ initialPage: "log", loading: false, detailLoading: true }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(
      canvasElement,
      "Loading changed files",
      "changed-files",
    );
  }}
/>

<Story
  name="Op Log detail"
  args={{ initialPage: "oplog", loading: false, detailLoading: true }}
  play={async ({ canvasElement }) => {
    await expectSkeleton(
      canvasElement,
      "Loading changed files in operation",
      "changed-files",
    );
    await expectSkeleton(
      canvasElement,
      "Loading selected operation comparison",
      "diff",
    );
  }}
/>
