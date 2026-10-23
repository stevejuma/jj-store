<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent chat runtime",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Configured agent enables first send after confirmed runtime configuration"
  args={{
    agentModeReady: false,
    sessionPreferences: { model: null, effort: null, mode: null },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Use my configured agent");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Session commands").textContent ?? "[]"),
      ).toMatchObject([
        {
          session_id: "44".repeat(16),
          action: {
            action: "configure",
            preferences: { model: null, effort: null, mode: "ask" },
          },
        },
        {
          session_id: "44".repeat(16),
          action: { action: "prompt", text: "Use my configured agent" },
        },
      ]),
    );
    await waitFor(() =>
      expect(draft).not.toHaveTextContent("Use my configured agent"),
    );
  }}
/>

<Story
  name="Failed resume uses composer error and retains the draft"
  args={{ status: "Offline", resumeFailure: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep this after failure");
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await userEvent.click(
      await within(document.body).findByRole("menuitem", {
        name: "Resume session",
      }),
    );
    const error = await c.findByRole("alert");
    await waitFor(() => expect(error).toHaveTextContent("agent_rejected"));
    await expect(
      error.closest('[data-ui-component="community-chat-composer"]'),
    ).not.toBeNull();
    await expect(draft).toHaveTextContent("Keep this after failure");
    await expect(draft).toBeEnabled();
    await expect(c.getAllByText(/agent_rejected/)).toHaveLength(1);
  }}
/>

<Story
  name="Retained ACP refusal replaces generic process error in the composer"
  args={{ status: "Offline", retainedError: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const error = await c.findByRole("alert");
    await waitFor(() => expect(error).toHaveTextContent("agent_rejected"));
    await expect(
      error.closest('[data-ui-component="community-chat-composer"]'),
    ).not.toBeNull();
    await expect(
      c.queryByText(/process outcome unknown/),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Unknown configuration outcome retains the exact request and never prompts"
  args={{
    agentModeReady: false,
    uncertainConfiguration: true,
    sessionPreferences: { model: null, effort: null, mode: null },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep uncertain prompt");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await c.findByText(/Outcome unknown/);
    await expect(draft).toHaveTextContent("Keep uncertain prompt");
    await expect(
      JSON.parse(c.getByLabelText("Session commands").textContent ?? "[]"),
    ).toHaveLength(1);
  }}
/>

<Story
  name="Send prepares an ended session automatically and dispatches once"
  args={{ status: "Offline", agentModeReady: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await expect(draft).toBeEnabled();
    await userEvent.type(draft, "Keep this for resume");
    await expect(
      c.queryByText(/Resume this session from Session options/),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByText(/Choose an execution mode in Session options/),
    ).not.toBeInTheDocument();
    await expect(c.getByRole("button", { name: "Send message" })).toBeEnabled();
    await expect(draft).toHaveTextContent("Keep this for resume");
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await waitFor(() => {
      const commands = JSON.parse(
        c.getByLabelText("Session commands").textContent ?? "[]",
      );
      expect(commands).toHaveLength(2);
      expect(commands[0].action.action).toBe("resume");
      expect(commands[1].action.action).toBe("prompt");
      expect(commands[1].action.text).toBe("Keep this for resume");
    });
  }}
/>

<Story
  name="Navigation during runtime preparation retains the draft and sends no prompt"
  args={{
    agentModeReady: false,
    holdConfiguration: true,
    sessionPreferences: { model: null, effort: null, mode: null },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await storyControl(document.body, "Open fixture conversation");
    await userEvent.type(
      await c.findByRole("textbox", { name: "Markdown editor" }),
      "Retain through preparation",
    );
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    await c.findByRole("button", { name: "Resolve fixture configuration" });
    await storyControl(document.body, "Manage fixture agents");
    await storyControl(document.body, "Resolve fixture configuration");
    await waitFor(
      () =>
        expect(c.getByRole("button", { name: "Refresh agents" })).toBeEnabled(),
      { timeout: 4000 },
    );
    await storyControl(document.body, "Open fixture conversation");
    await expect(
      await c.findByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retain through preparation");
    await expect(
      JSON.parse(c.getByLabelText("Session commands").textContent ?? "[]"),
    ).toHaveLength(1);
  }}
/>
