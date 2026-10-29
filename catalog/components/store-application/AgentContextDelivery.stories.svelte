<script module lang="ts">
  import { page as browser } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentContextDeliveryFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent context delivery",
    component: Fixture,
    tags: ["visual-pending", "component"],
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-agent-execution/StoreAgentPromptContent.svelte",
      },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
</script>

<Story
  name="Context button has a subtle resting surface"
  play={async ({ canvasElement }) => {
    const summary = within(canvasElement).getByRole("button", {
      name: "Queued context",
    });
    const resting = getComputedStyle(summary).backgroundColor;
    expect(resting).not.toBe("rgba(0, 0, 0, 0)");
    expect(resting).not.toBe("transparent");
    await browser.getByRole("button", { name: "Queued context" }).hover();
    await waitFor(() =>
      expect(getComputedStyle(summary).backgroundColor).not.toBe(resting),
    );
    await browser.getByRole("heading", { name: "Context delivery" }).hover();
    await userEvent.click(summary);
    await browser.getByRole("heading", { name: "Context delivery" }).hover();
    const expanded = getComputedStyle(summary).backgroundColor;
    await browser.getByRole("button", { name: "Queued context" }).hover();
    await waitFor(() =>
      expect(getComputedStyle(summary).backgroundColor).not.toBe(expanded),
    );
    await browser.getByRole("heading", { name: "Context delivery" }).hover();
  }}
/>

<Story
  name="Wide rendered context stays at the start"
  args={{ wide: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Queued context" }));
    const context = c.getByRole("button", { name: "<agent_context>" });
    const panel = canvasElement.querySelector<HTMLElement>(
      '[aria-label="Application context for this message"]',
    )!;
    expect(panel.getBoundingClientRect().width).toBeGreaterThan(1000);
    expect(
      context.getBoundingClientRect().left - panel.getBoundingClientRect().left,
    ).toBeLessThanOrEqual(16);
    await userEvent.click(context);
    await userEvent.click(c.getByRole("button", { name: "<origin>" }));
    const heading = c.getByRole("heading", { name: "Captured task" });
    // Two nested XML disclosures retain their own 1rem indentation.
    const nestedInset = [
      ...panel.querySelectorAll<HTMLElement>(".store-prompt-element__body"),
    ].reduce(
      (total, body) =>
        total + Number.parseFloat(getComputedStyle(body).paddingInlineStart),
      0,
    );
    expect(
      Math.abs(
        heading.getBoundingClientRect().left -
          context.getBoundingClientRect().left -
          nestedInset,
      ),
    ).toBeLessThanOrEqual(1);
  }}
/>

<Story
  name="One inspector retains disclosure through acknowledgement"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const article = c.getByRole("article", { name: "User message" });
    const inspector = article.querySelector<HTMLElement>(
      '[data-ui-component="agent-sent-context"]',
    )!;
    expect(
      article.querySelectorAll('[data-ui-component="agent-sent-context"]'),
    ).toHaveLength(1);
    const summary = c.getByRole("button", {
      name: "Queued context",
    });
    expect(summary).toHaveAttribute("aria-expanded", "false");
    expect(summary.getBoundingClientRect().height).toBeLessThanOrEqual(28);
    expect(summary.querySelector("svg")).toBe(summary.lastElementChild);
    await userEvent.click(summary);
    expect(summary).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(c.getByRole("button", { name: "<agent_context>" }));
    await userEvent.click(c.getByRole("button", { name: "<origin>" }));
    await expect(
      c.getByRole("heading", { name: "Captured task" }),
    ).toBeVisible();
    await expect(c.getByRole("link", { name: "safe links" })).toHaveAttribute(
      "href",
      "https://example.com/",
    );
    expect(article.querySelector("pre")).toHaveTextContent("<literal />");
    expect(c.queryByRole("button", { name: "<instructions>" })).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Acknowledge start" }));
    await expect(c.getByText("Sent context", { exact: true })).toBeVisible();
    expect(summary).toHaveAttribute("aria-expanded", "true");
    expect(
      inspector.compareDocumentPosition(
        c.getByText("Please update the parser."),
      ) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).not.toBe(0);
    const toggle = c.getByRole("button", { name: "Show raw context" });
    toggle.focus();
    await userEvent.keyboard("{Enter}");
    expect(toggle).toHaveFocus();
    expect(toggle).toHaveAttribute("aria-pressed", "true");
    expect(c.queryByText("Raw context payload", { exact: true })).toBeNull();
    const code = c.getByRole("region", { name: "Raw context" });
    await expect(code).toBeVisible();
    expect(code).toHaveTextContent('"wireContext"');
    expect(
      code.querySelector('[data-ui-component="code-block"]'),
    ).toHaveAttribute("data-language", "json");
    await expect(
      c.getByRole("heading", { name: "Captured task", hidden: true }),
    ).not.toBeVisible();
    await waitFor(() =>
      expect(
        code.querySelector('[data-ui-part="code-block-code"]'),
      ).not.toBeNull(),
    );
    await userEvent.click(summary);
    expect(summary).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(summary);
    const restoredToggle = c.getByRole("button", {
      name: "Show rendered context",
    });
    expect(restoredToggle).toHaveAttribute("aria-pressed", "true");
    restoredToggle.focus();
    await userEvent.keyboard(" ");
    expect(restoredToggle).toHaveAttribute("aria-pressed", "false");
    await expect(
      c.getByRole("heading", { name: "Captured task" }),
    ).toBeVisible();
    expect(c.queryByRole("region", { name: "Raw context" })).toBeNull();
  }}
/>

<Story
  name="No context produces no inspector"
  args={{ empty: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    expect(
      canvasElement.querySelector('[data-ui-component="agent-sent-context"]'),
    ).toBeNull();
    await expect(c.getByText("Please update the parser.")).toBeVisible();
  }}
/>

<Story
  name="Unknown delivery keeps expanded hover feedback"
  args={{ initialDelivery: "unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const button = c.getByRole("button", { name: "Context delivery unknown" });
    button.focus();
    await userEvent.keyboard("{Enter}");
    expect(button).toHaveAttribute("aria-expanded", "true");
    await browser.getByRole("heading", { name: "Context delivery" }).hover();
    const resting = getComputedStyle(button).backgroundColor;
    await browser
      .getByRole("button", { name: "Context delivery unknown" })
      .hover();
    await waitFor(() =>
      expect(getComputedStyle(button).backgroundColor).not.toBe(resting),
    );
    await browser.getByRole("heading", { name: "Context delivery" }).hover();
  }}
/>
