<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent session state",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      a11y: { test: "error" },
    },
  });
  async function layout(canvasElement: HTMLElement, label: string) {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const activity = await waitFor(() => {
      const row = canvasElement.querySelector<HTMLElement>(
        '[data-ui-component="community-agent-composer-activity"]',
      );
      expect(row).not.toBeNull();
      expect(row).toHaveTextContent(label);
      return row!;
    });
    await waitFor(() => {
      const transcript = canvasElement.querySelector(".chat-document")!,
        draft = canvasElement.querySelector("[data-chat-next-message]")!;
      expect(
        activity.getBoundingClientRect().top -
          transcript.getBoundingClientRect().bottom,
      ).toBeGreaterThanOrEqual(0);
      expect(
        activity.getBoundingClientRect().top -
          transcript.getBoundingClientRect().bottom,
      ).toBeLessThan(16);
      expect(activity.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        draft.getBoundingClientRect().top,
      );
      const avatar = activity.querySelector(".community-member-avatar")!,
        caption = activity.querySelector(
          ".agent-composer-activity__button > span:last-child",
        )!;
      const a = avatar.getBoundingClientRect(),
        t = caption.getBoundingClientRect();
      expect(
        Math.abs(a.top + a.height / 2 - t.top - t.height / 2),
      ).toBeLessThan(1);
    });
    return c;
  }
</script>

<Story
  name="Working follows messages and Stop preserves the draft"
  args={{ status: "Working", chatFeedback: "working" }}
  play={async ({ canvasElement }) => {
    const c = await layout(canvasElement, "Working");
    const draft = within(
      c.getByRole("group", { name: "Next message draft" }),
    ).getByRole("textbox");
    const reader = canvasElement.querySelector(
      '.chat-text-block[data-kind="markdown"] .cm-content',
    )!;
    expect(getComputedStyle(draft).fontFamily).toBe(
      getComputedStyle(reader).fontFamily,
    );
    expect(getComputedStyle(draft).fontSize).toBe(
      getComputedStyle(reader).fontSize,
    );
    expect(getComputedStyle(draft).lineHeight).toBe(
      getComputedStyle(reader).lineHeight,
    );
    await userEvent.click(draft);
    await userEvent.keyboard("Retained follow-up");
    const stop = c.getByRole("button", { name: "Stop agent" });
    expect(stop).toBeEnabled();
    expect(stop).toHaveAttribute("data-variant", "destructive");
    await userEvent.click(stop);
    await c.findByRole("button", { name: "Send message" });
    expect(draft).toHaveTextContent("Retained follow-up");
    expect(c.queryByRole("button", { name: "Stop agent" })).toBeNull();
    const command = JSON.parse(
      c.getByLabelText("Last session command").textContent!,
    );
    expect(command.action).toEqual({
      action: "cancel",
      target: "55".repeat(16),
    });
    expect(command.session_id).toBe("44".repeat(16));
  }}
/>
<Story
  name="Starting is directly after messages and centred on its avatar"
  args={{ status: "Starting" }}
  play={async ({ canvasElement }) => {
    const c = await layout(canvasElement, "Starting");
    const stop = c.getByRole("button", { name: "Stop agent" });
    expect(stop).toBeEnabled();
    await userEvent.click(stop);
    await waitFor(() => {
      expect(c.queryByRole("button", { name: "Stop agent" })).toBeNull();
      const command = JSON.parse(
        c.getByLabelText("Last session command").textContent!,
      );
      expect(command.action).toEqual({ action: "end" });
    });
  }}
/>
<Story
  name="Interrupted participation outranks a stale Starting receipt"
  args={{ status: "Offline", staleResumeReceipt: true, retainedError: true }}
  play={async ({ canvasElement }) => {
    const c = await layout(canvasElement, "Interrupted");
    expect(
      canvasElement.querySelector(
        '[data-ui-component="community-agent-composer-activity"]',
      ),
    ).toHaveAttribute("data-state", "interrupted");
    expect(c.queryByRole("button", { name: "Stop agent" })).toBeNull();
  }}
/>
