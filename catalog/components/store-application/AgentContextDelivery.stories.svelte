<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
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
    const inspector = article.querySelector<HTMLDetailsElement>(
      '[data-ui-component="agent-sent-context"]',
    )!;
    expect(
      article.querySelectorAll('[data-ui-component="agent-sent-context"]'),
    ).toHaveLength(1);
    expect(inspector.open).toBe(false);
    const summary = c.getByText("Queued context", { exact: true });
    await userEvent.click(summary);
    expect(inspector.open).toBe(true);
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
    expect(inspector.open).toBe(true);
    expect(
      inspector.compareDocumentPosition(
        c.getByText("Please update the parser."),
      ) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).not.toBe(0);
    await userEvent.click(c.getByText("Raw context payload", { exact: true }));
    await expect(
      c.getByText(/"wireContext"/, { selector: "pre" }),
    ).toBeVisible();
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
