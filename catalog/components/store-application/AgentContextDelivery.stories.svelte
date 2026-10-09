<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentContextDeliveryFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent context delivery",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

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
