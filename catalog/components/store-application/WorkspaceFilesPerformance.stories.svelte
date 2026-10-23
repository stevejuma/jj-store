<script module lang="ts">
  import { fileRowName } from "../../fixtures/file-row-name.js";
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within, waitFor } from "storybook/test";
  import { page as browser, userEvent as browserEvent } from "vitest/browser";
  import Fixture from "../../fixtures/WorkspaceFilesFixture.svelte";
  const longBranch =
    "feature/retain-exact-workspace-source-and-conversation-reply-identities";
  async function viewFiles() {
    const page = within(document.body);
    await userEvent.click(
      await page.findByRole("menuitem", { name: "View files" }),
    );
    await waitFor(() => {
      expect(page.queryByRole("menu", { name: "Workspace" })).toBeNull();
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none");
    });
  }
  const { Story } = defineMeta({
    title: "JJ Store/Conversation workspace files",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Menu opens without IO and files reopen from cache"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await waitFor(async () => {
      await expect(
        await page.findByRole("menuitem", { name: "View files" }),
      ).toBeVisible();
    });
    await expect(c.getByLabelText("Workspace read count")).toHaveTextContent(
      "0",
    );
    await viewFiles();
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await userEvent.click(
      await rail.findByRole("button", { name: fileRowName("README.md") }),
    );
    const inspector = within(
      await c.findByRole("complementary", { name: "Workspace file" }),
    );
    await c.findByRole("textbox", { name: fileRowName("README.md") });
    await expect(c.getByLabelText("Workspace read count")).toHaveTextContent(
      "2",
    );
    await userEvent.click(
      inspector.getByRole("button", { name: "Close workspace file" }),
    );
    await userEvent.click(
      rail.getByRole("button", { name: fileRowName("src/example.ts") }),
    );
    await c.findByRole("textbox", { name: fileRowName("src/example.ts") });
    await userEvent.click(
      rail.getByRole("button", { name: fileRowName("README.md") }),
    );
    await c.findByRole("textbox", { name: fileRowName("README.md") });
    await expect(c.getByLabelText("Workspace read count")).toHaveTextContent(
      "3",
    );
  }}
/>

<Story
  name="Rail loading mirrors files and changes"
  args={{ controlReads: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Hold next workspace read");
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await viewFiles();
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    const files = await rail.findByRole("status", {
      name: "Loading workspace files",
    });
    expect(
      files.querySelector(
        ".community-projects-repository-skeleton__tree-heading",
      ),
    ).not.toBeNull();
    expect(
      files.querySelectorAll(
        ".community-projects-repository-skeleton__tree-row",
      ),
    ).toHaveLength(6);
    expect(rail.queryByRole("button", { name: "Comparison range" })).toBeNull();
    await storyControl(document.body, "Complete workspace read");
    await rail.findByRole("button", { name: fileRowName("README.md") });
    await storyControl(document.body, "Hold next workspace read");
    await userEvent.click(rail.getByRole("tab", { name: /^Changes$/ }));
    const changes = await rail.findByRole("status", {
      name: "Loading workspace changes",
    });
    expect(
      changes.querySelector(
        ".community-projects-repository-skeleton__tree-heading",
      ),
    ).not.toBeNull();
    expect(
      changes.querySelectorAll(
        ".community-projects-repository-skeleton__tree-row",
      ),
    ).toHaveLength(6);
    expect(
      rail.getByRole("button", { name: "Comparison range" }),
    ).toBeVisible();
    expect(rail.getByRole("button", { name: "Comparison base" })).toBeVisible();
    await storyControl(document.body, "Complete workspace read");
    await rail.findByRole("button", { name: fileRowName("src/example.ts") });
    expect(
      rail.queryByRole("status", { name: "Loading workspace changes" }),
    ).toBeNull();
  }}
/>

<Story
  name="Empty files and changes keep the listing toolbar"
  args={{ emptyFiles: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await viewFiles();
    const panel = await c.findByRole("complementary", {
      name: "Workspace files",
    });
    const rail = within(panel);
    await rail.findByText("No workspace files");
    expect(
      panel.querySelector(".community-projects__file-tree-actions"),
    ).not.toBeNull();
    expect(
      panel.querySelector(".community-projects__file-tree-count"),
    ).toHaveTextContent("0");
    expect(panel.querySelector('[data-slot="empty-icon"] svg')).not.toBeNull();
    await userEvent.click(rail.getByRole("tab", { name: /^Changes$/ }));
    await rail.findByText("No changes");
    expect(
      rail.getByRole("button", { name: "Comparison range" }),
    ).toBeVisible();
    expect(
      panel.querySelector(".community-projects__file-tree-actions"),
    ).not.toBeNull();
    expect(
      panel.querySelector(".community-projects__file-tree-count"),
    ).toHaveTextContent("0");
    expect(panel.querySelector('[data-slot="empty-icon"] svg')).not.toBeNull();
  }}
/>
