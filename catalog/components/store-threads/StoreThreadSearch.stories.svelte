<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  import { THREAD_IDS } from "../../fixtures/thread-transport.js";
  const { Story } = defineMeta({
    title: "JJ Store/Threads/Inbox search",
    component: Fixture,
    tags: ["visual-pending", "application"],
    args: { searchable: true },
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  async function search({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const general = await c.findByRole("button", {
      name: /^Open Thread general /,
    });
    await expect(general).toHaveAccessibleDescription(
      new RegExp(
        `^Thread ${THREAD_IDS.repo}:${THREAD_IDS.thread}:${THREAD_IDS.repo}(?:, .+)?$`,
      ),
    );
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const input = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.type(input, "release checklist");
    await expect(
      c.getByRole("button", { name: /^Open Thread planning / }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: /^Open Thread general / }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: /^Open Thread planning / }),
    );
    await expect(
      await c.findByText("Release planning discussion"),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Close search" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const nextInput = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.clear(nextInput);
    await userEvent.type(nextInput, "no match");
    await expect(c.getByText("No matching conversations")).toBeVisible();
  }
</script>

<Story
  name="Search loaded Thread titles and previews without requests"
  play={search}
/>
<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Mobile search keeps the repository groups accessible"
  args={{}}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const page = within(canvasElement.ownerDocument.body);
    // The loading skeleton has the same panel and search controls as the inbox.
    // Wait for loaded content before retaining elements across the lazy mount.
    await page.findByText("planning", { selector: "strong" });
    await waitFor(() =>
      expect(
        canvasElement.querySelector(
          '[data-ui-component="app-shell"][data-ui-part="root"]',
        ),
      ).toHaveAttribute("data-display-mode", "mobile"),
    );
    const conversations = await c.findByRole("button", {
      name: "Toggle thread conversations",
    });
    await expect(conversations).toHaveAttribute("aria-pressed", "false");
    await userEvent.click(conversations);
    await expect(conversations).toHaveAttribute("aria-pressed", "true");
    const inbox = await page.findByRole("complementary", {
      name: "Thread conversations",
    });
    await expect(inbox).toBeVisible();
    const list = within(inbox);
    await userEvent.click(
      await list.findByRole("button", { name: "Search loaded conversations" }),
    );
    await userEvent.type(
      await list.findByRole("searchbox", {
        name: "Search loaded conversations",
      }),
      "release",
    );
    await expect(
      await list.findByRole("button", { name: /^Open Thread planning / }),
    ).toBeVisible();
  }}
/>
<Story
  name="Filtered pagination remains in the inbox list"
  args={{ pagedSearch: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: /^Open Thread general / });
    await userEvent.click(
      await c.findByRole("button", { name: "Search loaded conversations" }),
    );
    const input = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await userEvent.type(input, "release");
    await expect(c.getByText("No matching conversations")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: /More Threads/ }));
    await expect(
      await c.findByRole("button", { name: /^Open Thread planning / }),
    ).toBeVisible();
    await expect(input).toHaveValue("release");
  }}
/>
