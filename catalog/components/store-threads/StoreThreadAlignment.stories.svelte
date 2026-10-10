<script module lang="ts">
  import { page } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Threads/Community alignment",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Settings rename lifecycle and removal confirmation"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Conversation details" }),
    );
    await expect(body.queryByRole("menu")).toBeNull();
    const settings = c.getByRole("complementary", {
      name: "Conversation details",
    });
    await expect(within(settings).getByText("Code baseline")).toBeVisible();
    const nameStyle = getComputedStyle(
      within(settings).getByText("Name", { exact: true }),
    );
    for (const label of [
      "Lifecycle",
      "Revision",
      "Code baseline",
      "Linked activity",
    ]) {
      const style = getComputedStyle(
        within(settings).getByText(label, { exact: true }),
      );
      await expect(style.fontSize).toBe(nameStyle.fontSize);
      await expect(style.fontFamily).toBe(nameStyle.fontFamily);
      await expect(style.fontWeight).toBe(nameStyle.fontWeight);
    }
    const mainHeader = c
      .getByRole("heading", { name: "general", level: 1 })
      .closest(".lc-community-inbox__thread-header")!;
    const paneHeader = settings.querySelector(
      ".lc-community-inbox__thread-header",
    )!;
    const title = within(mainHeader as HTMLElement).getByRole("heading", {
      name: "general",
      level: 1,
    });
    const status = within(mainHeader as HTMLElement).getByRole("status");
    await expect(
      Math.abs(
        title.getBoundingClientRect().top +
          title.getBoundingClientRect().height / 2 -
          status.getBoundingClientRect().top -
          status.getBoundingClientRect().height / 2,
      ),
    ).toBeLessThan(1);
    await expect(
      Math.abs(
        mainHeader.getBoundingClientRect().bottom -
          paneHeader.getBoundingClientRect().bottom,
      ),
    ).toBeLessThan(1);
    await expect(getComputedStyle(paneHeader).borderBottomWidth).toBe("0px");

    await expect(settings).toHaveClass("lc-community-inbox__settings");
    await expect(
      settings.closest('[data-ui-part="body-sidebar"]'),
    ).not.toBeNull();
    await userEvent.click(
      within(settings).getByRole("button", { name: /^Name/ }),
    );
    const dialog = await body.findByRole("dialog", { name: "Rename thread" });
    const input = within(dialog).getByRole("textbox", { name: "Thread name" });
    await expect(
      input.closest(".community-settings__option-group"),
    ).not.toBeNull();
    await userEvent.clear(input);
    await userEvent.type(input, "planning");
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Save name" }),
    );
    await expect(
      await c.findByRole("heading", { name: "planning", level: 1 }),
    ).toBeVisible();
    await waitFor(() =>
      expect(body.queryByRole("dialog", { name: "Rename thread" })).toBeNull(),
    );
    await waitFor(() =>
      expect(document.body.style.pointerEvents).not.toBe("none"),
    );
    const archive = within(settings).getByRole("button", {
      name: "Archive thread",
    });
    const removeAction = within(settings).getByRole("button", {
      name: "Remove thread",
    });
    await expect(
      archive.querySelector("svg[data-icon=inline-start]"),
    ).not.toBeNull();
    await expect(
      removeAction.querySelector("svg[data-icon=inline-start]"),
    ).not.toBeNull();
    await expect(removeAction).toHaveClass("lc-channel-settings__destructive");
    await expect(getComputedStyle(removeAction).color).not.toBe(
      getComputedStyle(archive).color,
    );
    await userEvent.click(
      within(settings).getByRole("button", { name: "Set idle" }),
    );
    await expect(
      await within(settings).findByText("idle", { exact: true }),
    ).toBeVisible();
    const activate = within(settings).getByRole("button", {
      name: "Activate thread",
    });
    await expect(
      activate.querySelector("svg[data-icon=inline-start]"),
    ).not.toBeNull();
    await userEvent.click(activate);
    await expect(
      await within(settings).findByText("active", { exact: true }),
    ).toBeVisible();
    await userEvent.click(archive);
    await expect(
      await within(settings).findByText("archived", { exact: true }),
    ).toBeVisible();
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveAttribute("contenteditable", "false");
    await userEvent.click(
      within(settings).getByRole("button", { name: "Restore thread" }),
    );
    await expect(
      await within(settings).findByText("idle", { exact: true }),
    ).toBeVisible();
    await userEvent.click(
      within(settings).getByRole("button", { name: "Remove thread" }),
    );
    const remove = await body.findByRole("dialog", { name: "Remove thread?" });
    await userEvent.click(
      within(remove).getByRole("button", { name: "Cancel" }),
    );
    await expect(
      within(settings).getByText("idle", { exact: true }),
    ).toBeVisible();
  }}
/>

<Story
  name="Native slash references become composer chips"
  args={{ selected: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByText("Welcome to the channel")).toBeVisible();
    const input = c.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.click(input);
    await userEvent.type(input, "/threads/general");
    await userEvent.click(await c.findByRole("option", { name: /general/ }));
    await waitFor(() =>
      expect(
        input.querySelector('[data-ui-part="composer-entity-reference"]'),
      ).not.toBeNull(),
    );
    await userEvent.click(c.getByRole("button", { name: "Remove general" }));
    await waitFor(() =>
      expect(
        input.querySelector('[data-ui-part="composer-entity-reference"]'),
      ).toBeNull(),
    );
    await expect(input.textContent).not.toContain("/repos/");
    await userEvent.click(input);
    await userEvent.type(input, "/threads/general");
    await userEvent.click(await c.findByRole("option", { name: /general/ }));
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    const log = c.getByRole("log", { name: "general messages" });
    await waitFor(() =>
      expect(log.querySelectorAll("[data-message-id]")).toHaveLength(2),
    );
    const last = log.querySelectorAll("[data-message-id]")[1] as HTMLElement;
    await expect(
      await within(last).findByRole("button", { name: "general" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Readonly conversations omit mutation actions"
  args={{ selected: true, readOnly: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const text = await c.findByText("Welcome to the channel");
    const message = text.closest("[data-message-id]") as HTMLElement;
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
    await expect(
      within(message).queryByRole("button", { name: /^Add reaction(?: to|$)/ }),
    ).toBeNull();
    await expect(
      within(message).queryByRole("button", { name: "Reply in thread" }),
    ).toBeNull();
    await userEvent.click(
      within(message).getByRole("button", { name: "More message actions" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "View replies",
      }),
    );
    const replies = c.getByRole("complementary", { name: "Thread replies" });
    await expect(
      within(replies).queryByRole("button", { name: /^Add reaction(?: to|$)/ }),
    ).toBeNull();
    await expect(
      within(replies).getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveAttribute("contenteditable", "false");
    await userEvent.click(
      c.getByRole("button", { name: "Conversation details" }),
    );
    await expect(body.queryByRole("menu")).toBeNull();
    await expect(c.queryByRole("button", { name: "Remove thread" })).toBeNull();
    await expect(c.queryByRole("button", { name: /^Name/ })).toBeNull();
  }}
/>
