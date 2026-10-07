<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import SkeletonFixture from "../../fixtures/AgentsSkeletonFixture.svelte";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Interactive agent sessions",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="View sessions keeps empty inbox"
  args={{ emptySessions: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "View Repository helper" }),
    );
    const details = within(
      await c.findByRole("complementary", { name: "Agent details sidebar" }),
    );
    await userEvent.click(details.getByRole("button", { name: "View chats" }));
    await c.findByText("No sessions yet", { exact: true });
    await expect(
      c.getByText("Select a session", { exact: true }),
    ).toBeVisible();
    await expect(
      c.queryByRole("form", { name: "Start agent session" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Activity log keeps exact chat and its retained draft"
  args={{ lastActivityAt: 1_735_689_600_000 }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByRole("button", { name: /Open session Repository helper/ });
    const row = await c.findByRole("button", {
      name: /Open session Repository helper/,
    });
    await expect(row.querySelector("time")).toHaveAttribute(
      "datetime",
      "2025-01-01T00:00:00.000Z",
    );
    await expect(row.querySelector("time")).toHaveAccessibleName(
      /^Last activity:/,
    );
    await expect(row).not.toHaveTextContent("ended ·");
    await expect(row).not.toHaveTextContent("11".repeat(16));
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Retained in the inbox");
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await userEvent.click(page.getByRole("menuitem", { name: "Activity log" }));
    await c.findByText("Ready to help.");
    await expect(
      c.queryByRole("region", { name: "Agent activity history" }),
    ).not.toBeInTheDocument();
    await expect(
      await c.findByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retained in the inbox");
  }}
/>
<Story
  name="Legacy inbox sessions omit unknown activity time"
  args={{ status: "Offline", lastActivityAt: null }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const row = await c.findByRole("button", {
      name: /Open session Repository helper/,
    });
    await expect(row.querySelector("time")).not.toBeInTheDocument();
    await expect(
      row.querySelector(".lc-community-inbox__excerpt"),
    ).not.toBeInTheDocument();
    await expect(row).not.toHaveTextContent("ended ·");
    await expect(row).not.toHaveTextContent("11".repeat(16));
  }}
/>
<Story
  name="Agent tiles retain dimensions when details opens"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const card = await c.findByRole("button", {
      name: "View Repository helper",
    });
    const tiles = () =>
      Array.from(
        canvasElement.querySelectorAll(".agents__card, .agents__add-card"),
      );
    const before = tiles().map((tile) => ({
      height: tile.getBoundingClientRect().height,
      width: tile.getBoundingClientRect().width,
    }));
    await userEvent.click(card);
    await c.findByRole("complementary", { name: "Agent details sidebar" });
    await waitFor(() =>
      expect(
        tiles().map((tile) => ({
          height: tile.getBoundingClientRect().height,
          width: tile.getBoundingClientRect().width,
        })),
      ).toEqual(before),
    );
    await expect(
      before.every(
        (size) =>
          size.height === before[0].height && size.width === before[0].width,
      ),
    ).toBe(true);
    const header = canvasElement.querySelector(".store-agent-details-header")!;
    await expect(getComputedStyle(header).borderBottomWidth).toBe("0px");
  }}
/>

<Story
  name="Agent tiles show name and model only"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const opener = await c.findByRole("button", {
      name: "View Repository helper",
    });
    const card = within(opener.parentElement!);
    await expect(
      card.getByText("Repository helper", { exact: true }),
    ).toBeVisible();
    await expect(
      card.getByText("Harness default model", { exact: true }),
    ).toBeVisible();
    for (const text of [
      "Online",
      "Offline",
      "Codex · Installed",
      "Repository review and edits",
    ])
      await expect(
        card.queryByText(text, { exact: true }),
      ).not.toBeInTheDocument();
    await expect(
      card.getByRole("img", { name: "Repository helper, Idle" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Compact profile lifecycle actions"
  args={{ emptySessions: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "View Repository helper" }),
    );
    const actions = within(
      await c.findByRole("group", { name: "Agent profile actions" }),
    );
    const start = actions.getByRole("button", { name: "Start agent" });
    const sessions = actions.getByRole("button", { name: "View chats" });
    await expect(start.getBoundingClientRect().height).toBe(80);
    await expect(sessions.getBoundingClientRect().height).toBe(80);
    await expect(start).toHaveAttribute("data-variant", "default");
    await userEvent.click(sessions);
    await c.findByText("No sessions yet", { exact: true });
  }}
/>

<Story
  name="Card start warms the exact agent without opening a session"
  args={{ status: "Offline", emptySessions: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const start = await c.findByRole("button", {
      name: "Start Repository helper",
    });
    await expect(
      start.closest(".agents__card")!.getBoundingClientRect().height,
    ).toBe(260);
    await userEvent.click(start);
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Agent controls").textContent ?? "[]"),
      ).toMatchObject([{ version: 1, agent_id: "helper", start: true }]),
    );
    await expect(
      c.queryByRole("form", { name: "Start agent session" }),
    ).not.toBeInTheDocument();
    await expect(
      c.getByRole("button", { name: "View Repository helper" }),
    ).toBeVisible();
    await expect(
      c.getByRole("img", { name: "Repository helper, Idle" }),
    ).toBeVisible();
  }}
/>

<Story
  name="New chat keeps draft through connect without submitting it"
  args={{ emptySessions: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "New agent session" }));
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "My first chat message");
    await userEvent.click(c.getByRole("button", { name: "Connect agent" }));
    await c.findByRole("button", { name: "Send" });
    await expect(
      await c.findByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("My first chat message");
    await expect(c.queryByText("Turn completed")).not.toBeInTheDocument();
  }}
/>

<Story
  name="Agents skeleton matches fixed card dimensions"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("status", { name: "Loading your agents" });
    const tiles = Array.from(
      canvasElement.querySelectorAll(".community-skeleton__item"),
    );
    await expect(tiles.length).toBe(4);
    await waitFor(() => {
      for (const tile of tiles) {
        expect(tile.getBoundingClientRect().width).toBe(208);
        expect(tile.getBoundingClientRect().height).toBe(260);
      }
    });
  }}>{#snippet children()}<SkeletonFixture />{/snippet}</Story
>

<Story
  name="Draft empty state selects workspace and revision while retaining the message"
  args={{ alternativeAgent: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(c.getByRole("button", { name: "New agent session" }));
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep my workspace draft");
    const picker = c.getByRole("combobox", { name: "Repository workspace" });
    const empty = canvasElement.querySelector(".agent-draft-empty")!;
    const composer = canvasElement.querySelector(".community-chat-composer")!;
    await expect(empty.contains(picker)).toBe(true);
    await expect(composer.contains(picker)).toBe(false);
    await userEvent.click(picker);
    await userEvent.type(
      page.getByPlaceholderText("Search repositories"),
      "/device/alternate",
    );
    await waitFor(() => expect(page.getAllByRole("option")).toHaveLength(1));
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(picker).toHaveAttribute("title", "alternate");
    await expect(picker).toHaveFocus();
    await expect(draft).toHaveTextContent("Keep my workspace draft");
    await expect(
      c.getByRole("combobox", { name: "Starting revision" }),
    ).toHaveAttribute("title", "66".repeat(20));
    await userEvent.click(
      c.getByRole("combobox", { name: "Starting revision" }),
    );
    await userEvent.click(
      await page.findByRole("option", { name: /main Main branch/ }),
    );
    await expect(
      c.getByRole("combobox", { name: "Starting revision" }),
    ).toHaveAttribute("title", "77".repeat(20));
    await userEvent.click(c.getByRole("combobox", { name: "Agent" }));
    await userEvent.type(
      page.getByPlaceholderText("Search agents"),
      "Planning",
    );
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await c.findByText("Start a chat with Planning helper");
    await expect(
      c.getByRole("combobox", { name: "Repository workspace" }),
    ).toHaveTextContent("Alternate workspace");
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Keep my workspace draft");
    await expect(
      c.getByRole("combobox", { name: "Starting revision" }),
    ).toHaveAttribute("title", "77".repeat(20));
  }}
/>

<Story
  name="Inbox body skeleton waits for retained history"
  args={{ holdHistory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByRole("status", { name: "Loading session history" });
    await expect(
      c.queryByText("Start the conversation"),
    ).not.toBeInTheDocument();
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(
      c.getByRole("button", { name: "Resolve fixture history" }),
    );
    await c.findByText("Ready to help.");
    await expect(
      c.queryByRole("status", { name: "Loading session history" }),
    ).not.toBeInTheDocument();
  }}
/>
