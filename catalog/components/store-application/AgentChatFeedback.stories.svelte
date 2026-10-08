<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page as browser } from "vitest/browser";
  import { agentMessageTimestamp } from "../../../web/src/lib/collaboration/presentation.js";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent chat feedback",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Turn completion uses the shared system message"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const completed = await c.findByRole("group", { name: "Turn completed" });
    await expect(completed.querySelector("svg")).not.toBeNull();
    await expect(completed.querySelector("time")).not.toBeNull();
    await expect(completed).toHaveAttribute("title", "Turn completed");
    await expect(
      completed.closest('[data-ui-component="ai-chat-system-message"]'),
    ).toHaveAttribute("data-variant", "divider");
    await expect(completed.closest('[role="log"]')).not.toBeNull();
    await expect(
      c.queryByText("Working", { exact: true }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Running session mirrors Community working in the body and composer activity strip"
  args={{ status: "Working", chatFeedback: "working" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const working = await c.findAllByText("Working", { exact: true });
    const body = working.find((element) => element.closest('[role="log"]'));
    await expect(body).toBeDefined();
    await expect(
      body?.closest('[data-ui-component="ai-chat-reasoning"]'),
    ).toHaveAttribute("data-streaming", "true");
    const strip = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="community-agent-composer-activity"]',
    )!;
    await expect(strip).toBeVisible();
    await expect(
      strip.querySelector(".community-member-avatar"),
    ).not.toBeNull();
    await expect(within(strip).getByRole("status")).toHaveTextContent(
      "Working",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    await expect(
      canvasElement.querySelector(
        '[data-ui-component="community-agent-composer-activity"]',
      ),
    ).toBeNull();
  }}
/>

<Story
  name="Uncertain request keeps one status line with dismissible toast recovery"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await userEvent.type(
      await c.findByRole("textbox", { name: "Markdown editor" }),
      "Retain uncertain message",
    );
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    const strip = await waitFor(() => {
      const node = canvasElement.querySelector<HTMLElement>(
        '[data-ui-component="community-agent-composer-activity"]',
      );
      expect(node).not.toBeNull();
      return node!;
    });
    await expect(within(strip).getByRole("status")).toHaveTextContent(
      "Outcome unknown",
    );
    const page = within(document.body);
    const action = await page.findByRole("button", {
      name: "Review workspace",
    });
    await expect(action.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(c.queryByRole("alert")).not.toBeInTheDocument();
    await expect(strip.querySelector("button")).toBeNull();
    const row = strip.querySelector('[data-ui-part="reasoning-label"]')!;
    const range = document.createRange();
    range.selectNodeContents(row);
    await expect(range.getClientRects()).toHaveLength(1);
    const height = strip.getBoundingClientRect().height;
    await browser.viewport(390, 844);
    try {
      await waitFor(() => {
        expect(range.getClientRects()).toHaveLength(1);
        expect(strip.getBoundingClientRect().height).toBe(height);
      });
    } finally {
      await browser.viewport(1280, 800);
    }
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retain uncertain message");
    await expect(
      c.queryByText(/^Request .*Draft and request identity retained\./),
    ).not.toBeInTheDocument();
    await userEvent.click(
      page.getByRole("button", { name: "Dismiss Agent request unresolved" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByText("Outcome unknown", { exact: true });
    await expect(
      page.queryByRole("button", { name: "Review workspace" }),
    ).not.toBeInTheDocument();
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retain uncertain message");
    await userEvent.click(c.getByRole("button", { name: "Session options" }));
    await expect(
      await page.findByRole("menuitem", { name: "Review workspace" }),
    ).toBeEnabled();
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Keyboard toast recovery retains the draft and never repeats configuration or prompt"
  args={{
    agentModeReady: false,
    uncertainConfiguration: true,
    sessionPreferences: { model: null, effort: null, mode: null },
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const draft = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(draft, "Keep this unsent prompt");
    await userEvent.click(c.getByRole("button", { name: "Send message" }));
    const action = await page.findByRole("button", {
      name: "Review workspace",
    });
    const commands = c.getByLabelText("Session commands").textContent;
    await expect(JSON.parse(commands ?? "[]")).toHaveLength(1);
    action.focus();
    await expect(action).toHaveFocus();
    await userEvent.keyboard("{Enter}");
    await waitFor(() =>
      expect(
        page.queryByRole("button", { name: "Review workspace" }),
      ).not.toBeInTheDocument(),
    );
    await expect(c.getByLabelText("Session commands")).toHaveTextContent(
      commands!,
    );
    await expect(
      c.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Keep this unsent prompt");
    await expect(
      c.queryByText("Outcome unknown", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(c.getByRole("alert")).toHaveTextContent(
      "remains in session history",
    );
  }}
/>

<Story
  name="Messages use shared timestamp metadata and group unrecorded legacy times"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const user = await c.findByRole("article", { name: "Message from You" });
    const assistants = c.getAllByRole("article", {
      name: "Message from Repository helper",
    });
    const assistant = assistants.find((row) =>
      row.textContent?.includes("Ready to help."),
    )!;
    for (const [message, millis] of [
      [user, 1791324000000],
      [assistant, 1791324001000],
    ] as const) {
      const time = message.querySelector("time");
      await expect(time).not.toBeNull();
      await expect(time).toHaveAttribute(
        "datetime",
        new Date(millis).toISOString(),
      );
      await expect(
        time?.closest('[data-ui-component="ai-chat-message-metadata"]'),
      ).not.toBeNull();
      await expect(time?.getAttribute("title")).toBeTruthy();
      await expect(time).toHaveTextContent(
        agentMessageTimestamp(millis).timestamp!,
      );
    }
    const legacy = assistants.find((row) =>
      row.textContent?.includes("Legacy message without timing."),
    )!;
    await expect(legacy.querySelector("time")).toBeNull();
    await expect(
      c.getByText("Time not recorded", { exact: true }),
    ).toBeVisible();
    await expect(
      c.queryByText("Time unknown", { exact: true }),
    ).not.toBeInTheDocument();
    const dividers = canvasElement.querySelectorAll(
      '[data-ui-component="ai-chat-system-message"][data-variant="divider"]',
    );
    await expect(dividers.length).toBe(4);
    for (const divider of dividers)
      await expect(
        divider.querySelectorAll('[data-ui-component="separator"]'),
      ).toHaveLength(2);
  }}
/>
