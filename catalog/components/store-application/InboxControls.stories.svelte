<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentInboxTreeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Inbox controls",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/community-app-shell/CommunityAppShell.svelte",
      },
      layout: "fullscreen",
    },
  });
  async function controls({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement),
      body = within(document.body);
    const filter = c.getByRole("button", {
      name: "Filter loaded conversations",
    });
    await expect(c.queryByRole("searchbox")).toBeNull();
    await userEvent.click(filter);
    const a = body.getAllByRole("option", {
      name: "Repository: Sample repository",
    });
    await expect(a).toHaveLength(2);
    await userEvent.click(a[0]!);
    await userEvent.click(a[1]!);
    await userEvent.click(body.getByRole("option", { name: "Agent: Codex" }));
    await userEvent.click(body.getByRole("button", { name: "Close filters" }));
    await expect(filter).toHaveAttribute("aria-pressed", "true");
    await expect(
      c.getByRole("button", { name: /Open (session|Thread) First chat/ }),
    ).toBeVisible();
    await expect(
      c.getByRole("button", {
        name: /Open (session|Thread) Other repository chat/,
      }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: /Open (session|Thread) Second chat/ }),
    ).toBeNull();
    await expect(
      c.getByRole("textbox", { name: "Conversation draft" }),
    ).toHaveValue("Retained conversation draft");
    await userEvent.click(
      c.getByRole("button", { name: "Search loaded conversations" }),
    );
    const search = c.getByRole("searchbox", {
      name: "Search loaded conversations",
    });
    await expect(search).toHaveFocus();
    await userEvent.type(search, "other");
    await expect(
      c.queryByRole("button", { name: /Open (session|Thread) First chat/ }),
    ).toBeNull();
    await expect(
      c.getByRole("button", {
        name: /Open (session|Thread) Other repository chat/,
      }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await expect(
      c.getByRole("button", { name: "Search loaded conversations" }),
    ).toHaveFocus();
    await userEvent.click(filter);
    await userEvent.click(body.getByRole("button", { name: "Clear filters" }));
    await userEvent.click(body.getByRole("button", { name: "Close filters" }));
    await expect(
      c.getByRole("button", { name: /Open (session|Thread) Second chat/ }),
    ).toBeVisible();
    const handle = c.getByRole("button", {
      name: /Resize (session chats|thread conversations)/,
    });
    handle.focus();
    await userEvent.keyboard("{Home}");
    const bar = canvasElement.querySelector(
      ".lc-community-inbox__list .store-application__tabs",
    )!;
    await c.findByRole("button", { name: /More (session|thread) views/ });
    await waitFor(() =>
      expect(bar.scrollWidth).toBeLessThanOrEqual(bar.clientWidth + 1),
    );
    const more = c.getByRole("button", { name: /More (session|thread) views/ });
    await waitFor(() => expect(c.queryByRole("tab")).toBeNull());
    await waitFor(() => expect(more).toBeEnabled());
    await userEvent.click(more);
    await waitFor(async () => {
      await expect(body.getByRole("menuitem", { name: "All" })).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    for (const header of canvasElement.querySelectorAll(
      ".lc-community-inbox__list-header, .lc-community-inbox__thread-header",
    )) {
      await expect(header.getBoundingClientRect().height).toBe(52);
      await expect(
        getComputedStyle(
          header.matches(".lc-community-inbox__list-header")
            ? header.querySelector(".store-application__tabs")!
            : header,
        ).paddingLeft,
      ).toBe("8px");
    }
  }
</script>

<Story
  name="Sessions combine multi-select filters with expanding search and retained drafts"
  play={controls}
/>
<Story
  name="Threads reuse the same narrow controls and header geometry"
  args={{ kind: "thread" }}
  play={controls}
/>

<Story
  name="Empty conversation lists use the shared icon Empty composition"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Hide loaded sessions" }),
    );
    const empty = c
      .getByText("No sessions yet", { exact: true })
      .closest('[data-ui-part="empty"]')!;
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
    await expect(c.queryByRole("button", { name: /Open session/ })).toBeNull();
  }}
/>
<Story
  name="Empty Threads lists use the same icon composition"
  args={{ kind: "thread" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Hide loaded sessions" }),
    );
    const empty = c
      .getByText("No Threads yet", { exact: true })
      .closest('[data-ui-part="empty"]')!;
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).not.toBeNull();
  }}
/>

<Story
  name="Pinned conversations lead the tree and device archives preserve composition through remount"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const row = c
      .getByRole("button", { name: /Open session First chat/ })
      .closest(".lc-community-inbox__item-row")!;
    const title = row.querySelector("strong")!;
    const width = title.getBoundingClientRect().width;
    await userEvent.hover(row);
    await expect(title.getBoundingClientRect().width).toBe(width);
    await expect(
      getComputedStyle(row.querySelector(".inbox-row-actions")!).position,
    ).toBe("absolute");
    const pin = within(row as HTMLElement).getByRole("button", {
      name: "Pin conversation",
    });
    pin.focus();
    await expect(pin).toHaveAttribute("data-size", "icon-xs");
    await expect(pin).toHaveAttribute("data-variant", "secondary");
    await expect(
      within(row as HTMLElement).getByRole("button", {
        name: "Archive on this device",
      }),
    ).toHaveAttribute("data-variant", "secondary");
    await userEvent.keyboard("{Enter}");
    const pinned = c.getByRole("group", { name: "Pinned sessions" });
    await expect(c.getAllByRole("group", { name: /sessions$/ })[0]).toBe(
      pinned,
    );
    await expect(
      within(pinned).getByRole("button", { name: /Open session First chat/ }),
    ).toBeVisible();
    const archive = within(pinned).getByRole("button", {
      name: "Archive on this device",
    });
    archive.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      c.queryByRole("button", { name: /Open session First chat/ }),
    ).toBeNull();
    await expect(
      c.getByRole("textbox", { name: "Conversation draft" }),
    ).toHaveValue("Retained conversation draft");
    const chooseArchive = async () => {
      if (c.queryByRole("tab", { name: "Archived" }))
        await userEvent.click(c.getByRole("tab", { name: "Archived" }));
      else {
        await userEvent.click(
          c.getByRole("button", { name: "More session views" }),
        );
        await userEvent.click(body.getByRole("menuitem", { name: "Archived" }));
      }
    };
    await userEvent.click(c.getByRole("button", { name: "Unmount inbox" }));
    await userEvent.click(c.getByRole("button", { name: "Remount inbox" }));
    await expect(
      c.queryByRole("button", { name: /Open session First chat/ }),
    ).toBeNull();
    await chooseArchive();
    const restored = c.getByRole("group", { name: "Pinned sessions" });
    const restore = within(restored).getByRole("button", {
      name: "Restore on this device",
    });
    restore.focus();
    await userEvent.keyboard("{Enter}");
    await userEvent.click(c.getByRole("tab", { name: "All" }));
    await expect(
      c.getByRole("group", { name: "Pinned sessions" }),
    ).toBeVisible();
  }}
/>
