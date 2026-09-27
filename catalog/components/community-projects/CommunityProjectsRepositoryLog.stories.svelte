<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRepositoryLog from "./CommunityProjectsRepositoryLog.svelte";
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
    await expect(canvas.getByText("Conflicted bookmark")).toBeVisible();
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
    await expect(canvas.getByRole("status")).toHaveTextContent(
      "2 operation heads",
    );
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "describe: native object history",
      }),
    );
    const details = canvas.getByRole("region", { name: "Selected operation" });
    await expect(
      within(details).getByRole("button", {
        name: `Copy full operation ID: ${IDS.opLeft}`,
      }),
    ).toBeVisible();
    await userEvent.click(
      within(details).getByRole("button", { name: /Parent/ }),
    );
    await expect(
      canvas.getByRole("region", { name: "Selected operation" }),
    ).toHaveTextContent("Root operation");
    await expect(canvas.getByRole("status")).toHaveTextContent(
      "2 operation heads",
    );
  }}
  >{#snippet template()}<div class="repository-story">
      <RepositoryFixture page="oplog" />
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
<Story name="Error"
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
