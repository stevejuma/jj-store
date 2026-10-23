<script module lang="ts">
  import { page } from "vitest/browser";
  import { pendingReaction } from "../../fixtures/thread-reaction-play.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  const { Story } = defineMeta({
    title: "JJ Store/Threads",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
    beforeEach: async () => {
      await views.settings();
    },
  });
</script>

<Story
  name="Appearance focus preserves channel and reply drafts"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const welcome = await c.findByText("Welcome to the channel");
    const channel = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(channel);
    await userEvent.type(channel, "Unsent channel draft");
    const message = welcome.closest("article")!;
    await page
      .elementLocator(message.querySelector(".chat-text-block") ?? message)
      .hover();
    await waitFor(() =>
      expect(
        within(message as HTMLElement).getAllByRole("button", {
          name: /More message actions|Add reaction/,
        })[0]!,
      ).toBeVisible(),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: "More message actions" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Reply in thread" }),
    );
    const replies = c.getByRole("complementary", { name: "Thread replies" });
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.type(
      await within(replies).findByRole("textbox", { name: "Markdown editor" }),
      "Unsent reply draft",
    );
    await userEvent.click(
      c.getByRole("button", { name: /Open profile menu for/ }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Settings" }),
    );
    await userEvent.click(await c.findByRole("button", { name: "Appearance" }));
    await userEvent.click(await c.findByRole("tab", { name: "Preferences" }));
    await userEvent.click(await c.findByRole("radio", { name: "Focus" }));
    await userEvent.click(c.getByRole("radio", { name: "Larger" }));
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
    const dialog = await body.findByRole("region", { name: "Replies" });
    await expect(
      within(dialog).getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Unsent reply draft");
    await expect(
      canvasElement.querySelector(
        ".community-application__standard-right-sidebar",
      ),
    ).toBeNull();
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Close Replies" }),
    );
    await waitFor(() =>
      expect(body.queryByRole("region", { name: "Replies" })).toBeNull(),
    );
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Unsent channel draft");
    await page
      .elementLocator(c.getByText("Welcome to the channel").closest("article")!)
      .hover();

    await userEvent.click(
      within(
        c.getByText("Welcome to the channel").closest("article")!,
      ).getByRole("button", { name: "More message actions" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Reply in thread" }),
    );
    const reopened = await body.findByRole("region", { name: "Replies" });
    await expect(
      within(reopened).getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Unsent reply draft");
    await userEvent.click(
      within(reopened).getByRole("button", { name: "Close Replies" }),
    );
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  }}
/>
