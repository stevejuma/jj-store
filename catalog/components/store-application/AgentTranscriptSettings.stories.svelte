<script module lang="ts">
  import { selectSettingsSubtab } from "./settings-shell-play.js";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Projects from "../../fixtures/ProjectsHubFixture.svelte";
  import Repository from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import Captured from "../../fixtures/StoreCapturedReactionsFixture.svelte";
  import Prompt from "../../fixtures/AgentPromptFixture.svelte";
  import { agentPromptMarkup } from "../../fixtures/agent-prompt-markup.js";
  import Legacy from "../../fixtures/AgentLegacyTranscriptFixture.svelte";
  import AgentLayout from "../../fixtures/AgentLayoutFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent transcript settings",
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", a11y: { test: "error" } },
    beforeEach: async () => {
      await Promise.all([
        views.projects(),
        views.settings(),
        views.agentsRecorded(),
        views.agentSidebar(),
        views.agents(),
      ]);
    },
  });
  async function response({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    const agents = c.queryByRole("tab", { name: "Agents" });
    if (agents) await userEvent.click(agents);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended|^Codex ended/ }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await waitFor(() =>
      expect(
        panel.querySelector('[data-conversation-view="document"]'),
      ).not.toBeNull(),
    );
    expect(
      panel.querySelector('[data-ui-part="agent-session-footer"]'),
    ).toBeNull();
    expect(
      panel.querySelector('[data-ui-component="ai-chat-composer"]'),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: /Open profile menu for/ }),
    );
    await userEvent.click(
      within(document.body).getByRole("menuitem", { name: "Settings" }),
    );
    await userEvent.click(await c.findByRole("button", { name: "Appearance" }));
    await selectSettingsSubtab(canvasElement, "Preferences");
    await userEvent.click(await c.findByRole("radio", { name: /^Classic$/ }));
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
    const classic = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await waitFor(() =>
      expect(
        classic.querySelector('[data-conversation-view="document"]'),
      ).toBeNull(),
    );
    expect(
      within(classic).getByRole("log", { name: "Agent conversation activity" }),
    ).toBeVisible();
  }
</script>

<Story
  asChild
  name="Projects landing response follows the app conversation setting"
  play={response}
>
  {#snippet children()}<Projects agents />{/snippet}
</Story>

<Story
  asChild
  name="Read-only embedded captures have no prompt input or footer"
  play={async ({ canvasElement }) => {
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-conversation-view="document"]'),
      ).not.toBeNull(),
    );
    expect(canvasElement.querySelector(".community-chat-composer")).toBeNull();
    expect(canvasElement.querySelector(".chat-body-footer")).toBeNull();
    expect(
      within(canvasElement).queryByRole("group", {
        name: "Next message draft",
      }),
    ).toBeNull();
  }}
>
  {#snippet children()}<Captured interactive={false} />{/snippet}
</Story>
<Story
  asChild
  name="Document session review drafts survive Focus and Split remounts"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const focused = await portal.findByRole("region", {
      name: "Agent response",
    });
    expect(
      focused.querySelector('[data-ui-part="agent-session-footer"]'),
    ).toBeNull();
    await userEvent.click(
      within(focused).getByRole("button", {
        name: "Add review",
      }),
    );
    const editor = within(focused).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "Retained session review draft");
    await userEvent.click(
      c.getByRole("button", { name: /Open profile menu for/ }),
    );
    await userEvent.click(portal.getByRole("menuitem", { name: "Settings" }));
    await userEvent.click(await c.findByRole("button", { name: "Appearance" }));
    await selectSettingsSubtab(canvasElement, "Preferences");
    await userEvent.click(await c.findByRole("radio", { name: /^Split$/ }));
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
    const split = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await waitFor(() =>
      expect(
        within(split).getByRole("textbox", { name: "Markdown editor" }),
      ).toHaveTextContent("Retained session review draft"),
    );
    expect(split.querySelector('[data-ui-part="sidebar-footer"]')).toBeNull();
  }}
>
  {#snippet children()}<AgentLayout repository />{/snippet}
</Story>
<Story
  asChild
  name="Selected project response follows the app conversation setting"
  play={response}
>
  {#snippet children()}<Projects projectDetail />{/snippet}
</Story>
<Story
  asChild
  name="Repository response follows the app conversation setting"
  play={response}
>
  {#snippet children()}<Repository initialPage="agents" />{/snippet}
</Story>

<Story
  asChild
  name="Legacy captures retain older-message loading and follow the setting"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-conversation-view="document"]'),
      ).not.toBeNull(),
    );
    expect(canvasElement.querySelector(".chat-body-footer")).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Load older messages" }),
    );
    await c.findByText("Older captured answer");
    expect(c.queryByRole("button", { name: "Load older messages" })).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Use Classic view" }));
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-conversation-view="document"]'),
      ).toBeNull(),
    );
    await c.findByText("Older captured answer");
    expect(
      c.getByRole("log", { name: "Agent conversation activity" }),
    ).toHaveTextContent("Latest captured answer");
  }}
>
  {#snippet children()}<Legacy />{/snippet}
</Story>

<Story
  asChild
  name="Document recorded responses keep Send inside the panel"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const panel = await c.findByRole("complementary", {
      name: "Agent response",
    });
    const send = await within(panel).findByRole("button", {
      name: "Send message",
    });
    await expect(send).toBeVisible();
    const footer = panel.querySelector<HTMLElement>(".chat-body-footer")!;
    await waitFor(() =>
      expect(footer.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        panel.getBoundingClientRect().bottom,
      ),
    );
    const reviewAction = within(panel).getByRole("button", {
      name: "Add review",
    });
    expect(reviewAction.closest(".store-agent-chat-body")).not.toBeNull();
    expect(panel.querySelector('[data-ui-part="sidebar-footer"]')).toBeNull();
  }}
>
  {#snippet children()}<Captured surface="sidebar" />{/snippet}
</Story>
<Story
  asChild
  name="Document captures keep prompt markup readable and inert"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        canvasElement.querySelector('[data-conversation-view="document"]'),
      ).not.toBeNull(),
    );
    await userEvent.click(
      await c.findByRole("button", {
        name: '<harness:context source="cli > desktop">',
      }),
    );
    expect(c.getByRole("heading", { name: /Session notes/ })).toBeVisible();
    expect(
      canvasElement.querySelectorAll(
        ".community-markdown-content script, .community-markdown-content [onclick]",
      ),
    ).toHaveLength(0);
  }}
>
  {#snippet children()}<Prompt content={agentPromptMarkup} />{/snippet}
</Story>

<Story
  asChild
  name="Document capture actions expose native copy identities"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const message = await c.findByRole("article", {
      name: "Message from Maya Chen",
    });
    // Storybook's DOM hover cannot set the native CSS :hover pseudo-class.
    // Focus the reading editor to exercise the same accessible action bar.
    await userEvent.click(
      within(message).getByRole("textbox", { name: "Your message" }),
    );
    const menu = within(message).getByRole("button", {
      name: "More message actions",
    });
    await waitFor(() => expect(menu).toBeVisible());
    await userEvent.click(menu);
    const portal = within(document.body);
    const copy = await portal.findByRole("menuitem", { name: /^Copy$/ });
    await waitFor(() =>
      expect(
        copy.closest('[role="menu"]')!.contains(document.activeElement),
      ).toBe(true),
    );
    copy.focus();
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(async () => {
      await expect(
        await portal.findByRole("menuitem", { name: "Copy message ID" }),
      ).toBeVisible();
    });
    await waitFor(async () => {
      await expect(
        portal.getByRole("menuitem", { name: "Copy session ID" }),
      ).toBeVisible();
    });
    await waitFor(async () => {
      await expect(
        portal.getByRole("menuitem", { name: "Copy turn ID" }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{ArrowDown}");
    await waitFor(() =>
      expect(
        portal
          .getByRole("menu", { name: "Copy" })
          .contains(document.activeElement),
      ).toBe(true),
    );
    await waitFor(() => {
      for (const menu of portal.getAllByRole("menu"))
        expect(
          menu
            .getAnimations()
            .some((animation) => animation.playState === "running"),
        ).toBe(false);
    });
  }}
>
  {#snippet children()}<Prompt content="Retained captured message" />{/snippet}
</Story>
