<script module lang="ts">
  import { page } from "vitest/browser";
  import { pendingReaction } from "../../fixtures/thread-reaction-play.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Threads",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Channel navigation creation and members"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await expect(
      (await c.findAllByRole("button", { name: "Browse threads" })).find(
        (button) => button.closest(".lc-community-navigation__section-header"),
      )!,
    ).toBeVisible();
    await userEvent.click(
      (await c.findAllByRole("button", { name: "general" }))[0]!,
    );
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: /View thread contributors/ }),
    );
    await expect(
      c.getByRole("complementary", { name: "Thread contributors" }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Close thread panel" }),
    );
    await userEvent.click(c.getByRole("button", { name: "All threads" }));
    await userEvent.click(
      c.getByRole("button", { name: "Start a new post in Threads" }),
    );
    await userEvent.type(
      await c.findByRole("textbox", { name: "Markdown editor" }),
      "planning{Shift>}{Enter}{/Shift}First post",
    );
    await userEvent.click(c.getByRole("button", { name: "Publish post" }));
    await expect(
      await c.findByText("planning", { selector: "[data-ui-part=room-title]" }),
    ).toBeVisible();
    await expect(
      await c.findByRole("button", { name: "planning" }),
    ).toBeVisible();
    await expect(await c.findByText(/First post/)).toBeVisible();
  }}
/>
<Story
  name="Shared composer replies and exact reactions"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const input = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(input);
    await userEvent.type(input, "A message{Control>}{Enter}{/Control}");
    await expect(
      await within(c.getByRole("log", { name: "general messages" })).findByText(
        "A message",
        { exact: true },
      ),
    ).toBeVisible();
    const message =
      c.getByText("A message", { exact: true }).closest("[data-message-id]") ??
      c.getByText("A message", { exact: true }).closest("article");
    await page
      .elementLocator(message!.querySelector(".chat-text-block") ?? message!)
      .hover();
    await waitFor(() =>
      expect(
        within(message as HTMLElement).getAllByRole("button", {
          name: /Reply in thread|View replies|Add reaction/,
        })[0]!,
      ).toBeVisible(),
    );
    await userEvent.click(
      within(message as HTMLElement).getByRole("button", {
        name: "Reply in thread",
      }),
    );
    const replies = c.getByRole("complementary", { name: "Thread replies" });
    const replyInput = within(replies).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(replyInput);
    await userEvent.type(replyInput, "A reply{Control>}{Enter}{/Control}");
    await expect(
      await within(replies).findByText("A reply", { exact: true }),
    ).toBeVisible();
    await expect(
      within(c.getByRole("log", { name: "general messages" })).getByText(
        "A message",
        { exact: true },
      ),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh threads" }));
    await expect(
      await within(replies).findByText("A reply", { exact: true }),
    ).toBeVisible();
    const reply =
      within(replies)
        .getByText("A reply", { exact: true })
        .closest("[data-message-id]") ??
      within(replies).getByText("A reply", { exact: true }).closest("article");
    await page
      .elementLocator(reply!.querySelector(".chat-text-block") ?? reply!)
      .hover();
    await waitFor(() =>
      expect(
        within(reply as HTMLElement).getAllByRole("button", {
          name: /Reply in thread|View replies|Add reaction/,
        })[0]!,
      ).toBeVisible(),
    );
    await userEvent.click(
      within(reply as HTMLElement).getByRole("button", {
        name: /^Add reaction(?: to|$)/,
      }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await expect(
      await within(reply as HTMLElement).findByRole("button", {
        name: /1 reaction with Lapis/,
      }),
    ).toBeVisible();
    const reaction = within(reply as HTMLElement).getByRole("button", {
      name: /1 reaction with Lapis/,
    });
    reaction.focus();
    await expect(
      await body.findByText(/1 reaction with Lapis/, {
        selector: '[data-ui-part="tooltip-content"]',
      }),
    ).toBeVisible();
    await expect(
      body.queryByRole("button", { name: /Who reacted/ }),
    ).toBeNull();
    await userEvent.click(reaction);
    await waitFor(() =>
      expect(
        within(reply as HTMLElement).queryByRole("button", {
          name: /1 reaction with Lapis/,
        }),
      ).toBeNull(),
    );
  }}
/>
<Story
  name="Unknown action keeps its receipt identity"
  args={{ selected: true, unknownFirst: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const input = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(input);
    await userEvent.type(input, "Confirmed once{Control>}{Enter}{/Control}");
    await expect(
      (await c.findAllByText(/action outcome is unknown/))[0]!,
    ).toBeVisible();
    await expect(
      canvasElement.querySelectorAll(
        '.community-message [data-ui-part="bubbles"] [role="status"]',
      ),
    ).toHaveLength(0);
    await userEvent.click(
      c.getAllByRole("button", { name: "Check action status" })[0]!,
    );
    await expect(
      await within(c.getByRole("log", { name: "general messages" })).findByText(
        "Confirmed once",
        { exact: true },
      ),
    ).toBeVisible();
    await waitFor(() =>
      expect(
        c.queryAllByRole("button", { name: "Check action status" }),
      ).toHaveLength(0),
    );
    const dismiss = c.queryByRole("button", { name: "Dismiss" });
    if (dismiss) await userEvent.click(dismiss);
  }}
/>

<Story
  name="Unknown reaction stays in main message metadata"
  args={{ selected: true, unknownFirst: true, withReply: true }}
  play={(context) => pendingReaction(context)}
/>
<Story
  name="Unknown reply reaction stays in reply message metadata"
  args={{ selected: true, unknownFirst: true, replySelected: true }}
  play={(context) => pendingReaction(context, true)}
/>

<Story
  name="Rejected reaction stays in message metadata"
  args={{ selected: true, rejectReaction: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const message = (await c.findByText("Welcome to the channel")).closest(
      "article",
    )!;
    await page
      .elementLocator(message.querySelector(".chat-text-block") ?? message)
      .hover();
    await waitFor(() =>
      expect(
        within(message as HTMLElement).getAllByRole("button", {
          name: /Reply in thread|View replies|Add reaction/,
        })[0]!,
      ).toBeVisible(),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: /^Add reaction(?: to|$)/ }),
    );
    await userEvent.click(await body.findByRole("button", { name: ":lapis:" }));
    await waitFor(() =>
      expect(
        message.querySelector(
          '[data-ui-component="ai-chat-message-metadata"] [role="alert"]',
        ),
      ).toHaveTextContent("Failed"),
    );
    const retry = within(message).getByRole("button", {
      name: "Retry reactions",
    });
    const metadataRow = retry.closest(".community-message__metadata");
    await expect(metadataRow).not.toBeNull();
    await expect(retry.textContent?.trim()).toBe("");
    await expect(metadataRow).toContainElement(
      within(message).getByRole("alert"),
    );
    within(message).getByRole("button", { name: "Reaction failed" }).focus();
    await expect(
      await body.findByText(/access denied/, {
        selector: '[data-ui-part="tooltip-content"]',
      }),
    ).toBeVisible();
    within(message).getByRole("button", { name: "Reaction failed" }).blur();
    await expect(
      canvasElement.querySelector(
        '.store-threads > [data-ui-component="alert"]',
      ),
    ).toBeNull();
    await expect(
      c.queryByRole("button", { name: "Check action status" }),
    ).toBeNull();
    await expect(message).toHaveTextContent("Welcome to the channel");
  }}
/>

<Story
  name="Appearance focus preserves channel and reply drafts"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const welcome = await c.findByText("Welcome to the channel");
    const channel = c.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(channel);
    await userEvent.type(channel, "Unsent channel draft");
    const message = welcome.closest("article")!;
    await page
      .elementLocator(message.querySelector(".chat-text-block") ?? message)
      .hover();
    await waitFor(() =>
      expect(
        within(message as HTMLElement).getAllByRole("button", {
          name: /Reply in thread|View replies|Add reaction/,
        })[0]!,
      ).toBeVisible(),
    );
    await userEvent.click(
      within(message).getByRole("button", { name: "Reply in thread" }),
    );
    const replies = c.getByRole("complementary", { name: "Thread replies" });
    await userEvent.type(
      within(replies).getByRole("textbox", { name: "Markdown editor" }),
      "Unsent reply draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Toggle details" }));
    await expect(
      c.getByRole("complementary", { name: "Repository details" }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: /Open profile menu for/ }),
    );
    await userEvent.click(body.getByRole("menuitem", { name: "Settings" }));
    await userEvent.click(await c.findByRole("button", { name: "Appearance" }));
    await userEvent.click(c.getByRole("radio", { name: "Focus" }));
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
    await expect(
      c.getByRole("complementary", { name: "Repository details" }),
    ).toBeVisible();
    await page
      .elementLocator(c.getByText("Welcome to the channel").closest("article")!)
      .hover();

    await userEvent.click(
      within(
        c.getByText("Welcome to the channel").closest("article")!,
      ).getByRole("button", { name: "Reply in thread" }),
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
