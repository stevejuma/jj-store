<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRepositoryLog from "../../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryLog.svelte";
  import RepositoryFixture from "../../fixtures/RepositoryFixture.svelte";
  import { IDS } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Repository",
    component: CommunityProjectsRepositoryLog,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      docs: {
        source: {
          language: "svelte",
          code: '<script>\n  import { CommunityProjectsRepositoryLog } from "./catalog/index.js";\n<\/script>\n<CommunityProjectsRepositoryLog page="log" {graph} {now} {selectedId} {files} {selectedPath} {patch} {onSelectBranch} {onSelectRevision} {onSelectPath} />',
        },
      },
    },
  });
  async function inspectDiff(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    const files = await canvas.findByRole("region", { name: "Changed files" });
    await userEvent.click(
      within(files).getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(
      within(files).getByRole("button", { name: "src/lib.rs" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="file-diff"]'),
      ).not.toBeNull(),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="file-diff"]'),
    ).toHaveTextContent("publish_native_objects()?;");
    await userEvent.click(
      canvas.getByRole("radio", { name: "Show split diff" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-view-mode="split"]'),
      ).not.toBeNull(),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Wrap text" }));
    await expect(
      canvasElement.querySelector('[data-wrap="true"]'),
    ).not.toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Close Diff column" }),
    );
    await expect(
      canvas.queryByRole("button", { name: "Wrap text" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(files).getByRole("button", { name: "src/lib.rs" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Wrap text" }),
    ).toBeVisible();
  }
</script>

<Story
  name="Branches to diff"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("option", { name: "feature/native-objects" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Stream immutable native objects" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", {
        name: "Preserve concurrent publication history",
      }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("option", { name: "main" }));
    await inspectDiff(canvasElement);
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture />
    </div>{/snippet}</Story
>
<Story
  name="JJ Log"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("button", {
        name: "Stream immutable native objects",
      }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", {
        name: "Preserve concurrent publication history",
      }),
    ).toBeVisible();
    const graphRow = canvas
      .getByRole("button", { name: "Stream immutable native objects" })
      .closest<HTMLElement>(".community-revision-graph__row");
    if (!graphRow) throw new Error("Expected a revision graph row");
    await expect(getComputedStyle(graphRow.querySelector("svg")!).zIndex).toBe(
      "2",
    );
    await inspectDiff(canvasElement);
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="log" />
    </div>{/snippet}</Story
>
<Story
  name="Conflicted bookmark"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("option", { name: /conflicted/ }));
    await expect(
      within(canvasElement.ownerDocument.body).getByText("Conflicted bookmark"),
    ).toBeVisible();
    await expect(
      canvas.queryByText("Conflicted bookmark"),
    ).not.toBeInTheDocument();
    await expect(
      await canvas.findByRole("button", {
        name: "Stream immutable native objects",
      }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", {
        name: "Preserve concurrent publication history",
      }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("region", { name: "Changed files" }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("option", { name: /retired/ }));
    await expect(
      canvas.getByText("No revisions for this bookmark"),
    ).toBeVisible();
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture />
    </div>{/snippet}</Story
>
<Story
  name="Op Log"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      within(canvasElement.ownerDocument.body).getByText(/2 operation heads/),
    ).toBeVisible();
    await expect(
      canvas.queryByText(/2 operation heads/),
    ).not.toBeInTheDocument();
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "describe: native object history",
      }),
    );
    const files = await canvas.findByRole("region", {
      name: "Changed files in operation",
    });
    await expect(files).toBeVisible();
    const details = canvas.getByRole("region", { name: "Selected operation" });
    const diffColumn = details.closest<HTMLElement>('[data-ui-part="column"]')!;
    const closeButton = within(diffColumn).getByRole("button", {
      name: "Close Diff column",
    });
    await expect(closeButton).toBeVisible();
    await expect(getComputedStyle(closeButton).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(
      canvas.queryByText("Operation details"),
    ).not.toBeInTheDocument();
    const metadata = canvas.getByRole("region", {
      name: "Selected operation metadata",
    });
    await expect(getComputedStyle(metadata).borderBottomWidth).toBe("0px");
    const copyButton = within(details).getByRole("button", {
      name: `Copy full operation ID: ${IDS.opLeft}`,
    });
    await expect(copyButton).toBeVisible();
    const parentButton = within(details).getByRole("button", {
      name: /Parent/,
    });
    await expect(getComputedStyle(copyButton).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
    await expect(getComputedStyle(parentButton).backgroundColor).toBe(
      getComputedStyle(copyButton).backgroundColor,
    );
    await expect(
      within(diffColumn).getByText("No file selected"),
    ).toBeVisible();
    await expect(within(diffColumn).getByRole("status")).toHaveTextContent(
      "Select a changed file to preview the operation diff.",
    );
    await expect(
      diffColumn.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    const operationToggle = within(diffColumn).getByRole("button", {
      name: "describe: native object history",
    });
    await expect(operationToggle).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(operationToggle);
    await expect(
      canvas.queryByRole("region", { name: "Selected operation metadata" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("region", { name: "Selected operation" }),
    ).toBeVisible();
    await userEvent.click(operationToggle);
    await expect(
      canvas.getByRole("region", { name: "Selected operation metadata" }),
    ).toBeVisible();
    await userEvent.click(
      within(files).getByRole("radio", { name: "Show files as list" }),
    );
    await userEvent.click(
      within(files).getByRole("button", { name: "assets/logo.bin" }),
    );
    await expect(
      within(diffColumn).getByText("Binary diff unavailable"),
    ).toBeVisible();
    await expect(within(diffColumn).getByRole("status")).toHaveTextContent(
      "This file contains binary or invalid UTF-8 data.",
    );
    await userEvent.click(
      within(files).getByRole("button", { name: "src/lib.rs" }),
    );
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-ui-component="file-diff"]'),
      ).not.toBeNull(),
    );
    await expect(
      canvasElement.querySelector('[data-ui-component="file-diff"]'),
    ).toHaveTextContent("publish_native_objects()?");
    const selectedFileDiff = canvas.getByRole("region", {
      name: "Selected file diff",
    });
    await expect(details.getBoundingClientRect().left).toBeCloseTo(
      selectedFileDiff.getBoundingClientRect().left,
      0,
    );
    await expect(details.getBoundingClientRect().right).toBeCloseTo(
      selectedFileDiff.getBoundingClientRect().right,
      0,
    );
    const expandedDetails = canvas.getByRole("region", {
      name: "Selected operation",
    });
    await userEvent.click(
      within(expandedDetails).getByRole("button", { name: /Parent/ }),
    );
    await expect(
      canvas.getByRole("region", { name: "Selected operation" }),
    ).toHaveTextContent("Root operation");
    await expect(
      within(canvasElement.ownerDocument.body).getByText(/2 operation heads/),
    ).toBeVisible();
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="oplog" />
    </div>{/snippet}</Story
>
<Story
  name="Op Log comparison warning"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await expect(page.getByText("Comparison needs a target")).toBeVisible();
    await expect(
      canvas.queryByText("Comparison needs a target"),
    ).not.toBeInTheDocument();
    await expect(
      page.getByText(/One affected change has ambiguous lineage/),
    ).toBeVisible();
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="oplog" unavailableComparisonCount={1} />
    </div>{/snippet}</Story
>
<Story name="Loading"
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="log" state="loading" />
    </div>{/snippet}</Story
>
<Story name="Empty"
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="log" state="empty" />
    </div>{/snippet}</Story
>
<Story
  name="Error"
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement.ownerDocument.body).getByText("History unavailable"),
    ).toBeVisible();
    await expect(
      within(canvasElement).queryByText("History unavailable"),
    ).not.toBeInTheDocument();
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="log" state="error" />
    </div>{/snippet}</Story
>
<Story name="Refreshing retains history"
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="log" state="refreshing" />
    </div>{/snippet}</Story
>

<style>
  .repository-story {
    height: 100dvh;
    display: flex;
    min-height: 0;
  }
</style>
