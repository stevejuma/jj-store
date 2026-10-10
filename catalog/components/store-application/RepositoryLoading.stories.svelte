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
    const status = await within(canvasElement).findByRole("status", {
      name: label,
    });
    await expect(status).toHaveAttribute("aria-busy", "true");
    await expect(status).toHaveAttribute("data-layout", layout);
    await expect(
      status.querySelector('[data-ui-component="skeleton"]'),
    ).not.toBeNull();
  }

  async function expectReviewDetailSkeleton(canvasElement: HTMLElement) {
    await expectSkeleton(
      canvasElement,
      "Loading review details",
      "review-detail",
    );
    const status = await within(canvasElement).findByRole("status", {
      name: "Loading review details",
    });
    const header = status.querySelector<HTMLElement>(
      ".community-projects__review-header",
    )!;
    const body = status.querySelector<HTMLElement>(
      ".community-projects__review-body",
    )!;
    const footer = status.querySelector<HTMLElement>(
      ".community-projects__review-footer",
    )!;
    await expect(header).not.toBeNull();
    await expect(
      body.querySelector(".community-projects__review-meta"),
    ).not.toBeNull();
    await expect(
      body.querySelectorAll(".community-projects__review-section"),
    ).toHaveLength(4);
    await expect(footer).not.toBeNull();
    const scroll = status.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await expect(scroll.contains(header)).toBe(false);
    await expect(scroll.contains(footer)).toBe(false);
    await expect(status.getBoundingClientRect().width).toBeGreaterThan(
      canvasElement
        .querySelector(".store-application__body-regions")!
        .getBoundingClientRect().width * 0.95,
    );
    await expect(header.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      scroll.getBoundingClientRect().top,
    );
    await expect(scroll.getBoundingClientRect().bottom).toBeLessThanOrEqual(
      footer.getBoundingClientRect().top,
    );
    await expect(within(status).queryByRole("textbox")).toBeNull();
    await expect(
      within(canvasElement).queryByText("Review unavailable"),
    ).toBeNull();
  }
</script>

<Story
  name="Connection loading branches"
  args={{ initialPage: "branches", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Connection loading log"
  args={{ initialPage: "log", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Connection loading oplog"
  args={{ initialPage: "oplog", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Connection loading files"
  args={{ initialPage: "files", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Connection loading agents"
  args={{ initialPage: "agents", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Connection loading reviews"
  args={{ initialPage: "reviews", connectionLoading: true }}
  play={async ({ canvasElement }) => {
    await expect(
      canvasElement.querySelector(
        '[role="status"] [data-ui-component="skeleton"]',
      ),
    ).not.toBeNull();
    await expect(
      within(canvasElement).queryByText(
        /No agent sessions|No review threads|No branches|No files/,
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Reviews"
  args={{ initialPage: "reviews" }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByRole("status", {
        name: "Loading review threads",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Review detail"
  args={{ initialPage: "reviews", loading: false, detailLoading: true }}
  play={async ({ canvasElement }) => {
    await expectReviewDetailSkeleton(canvasElement);
  }}
/>

<Story
  name="Review detail before list read"
  args={{ initialPage: "reviews", detailLoading: true }}
  play={async ({ canvasElement }) => {
    await expectReviewDetailSkeleton(canvasElement);
  }}
/>

<Story
  name="Review detail before connection"
  args={{
    initialPage: "reviews",
    connectionLoading: true,
    detailLoading: true,
  }}
  play={async ({ canvasElement }) => {
    await expectReviewDetailSkeleton(canvasElement);
  }}
/>

<Story
  name="Narrow review detail"
  args={{
    initialPage: "reviews",
    detailLoading: true,
    initiallyCollapsed: true,
  }}
  play={async ({ canvasElement }) => {
    const host = canvasElement.querySelector<HTMLElement>(
      ".store-application__body-regions",
    )!;
    host.style.width = "380px";
    host.style.height = "640px";
    await expectReviewDetailSkeleton(canvasElement);
    const status = await within(canvasElement).findByRole("status", {
      name: "Loading review details",
    });
    await expect(status.scrollWidth).toBeLessThanOrEqual(status.clientWidth);
  }}
/>

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
      "Loading repository files",
      "directory",
    );
    await expect(
      within(canvasElement).queryByRole("complementary", {
        name: "Repository file tree",
      }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="File detail"
  args={{ initialPage: "files", loading: false, detailLoading: true }}
  play={async ({ canvasElement }) => {
    await within(canvasElement).findByRole(
      "list",
      { name: "Repository breadcrumb" },
      { timeout: 10000 },
    );
    const status = await within(canvasElement).findByRole("status", {
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
    const files = await within(canvasElement).findByRole("region", {
      name: "Changed files",
    });
    await expect(within(files).getByText("Changed files")).toBeVisible();
    await expect(
      within(files).getByRole("radio", { name: "Show files as folders" }),
    ).toBeVisible();
    await expect(
      files.querySelector('[data-ui-part="changed-files-count"]'),
    ).not.toBeNull();
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
    const files = await within(canvasElement).findByRole("region", {
      name: "Changed files in operation",
    });
    await expect(
      within(files).getByText("Changed files in operation"),
    ).toBeVisible();
    await expect(
      within(files).getByRole("radio", { name: "Show files as list" }),
    ).toBeVisible();
    await expect(
      files.querySelector('[data-ui-part="changed-files-count"]'),
    ).not.toBeNull();
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
