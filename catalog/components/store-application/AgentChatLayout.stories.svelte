<script module lang="ts">
  import { page as browser } from "vitest/browser";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent chat layout",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Messages scroll above the bottom composer with keyboard jump to latest"
  args={{ longHistory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByText("Ready to help.");
    const layout = canvasElement.querySelector<HTMLElement>(
      ".chat-body-reading-surface",
    )!;
    const viewport = layout.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const dock = layout.querySelector<HTMLElement>(".chat-body-footer")!;
    const assertUncovered = () => {
      expect(viewport.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        dock.getBoundingClientRect().top + 1,
      );
      expect(
        c
          .getByRole("textbox", { name: "Markdown editor" })
          .getBoundingClientRect().bottom,
      ).toBeLessThanOrEqual(viewport.getBoundingClientRect().bottom + 1);
    };
    await userEvent.click(c.getByRole("button", { name: "Jump to end" }));
    await waitFor(assertUncovered);
    const draft = c.getByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(
      draft,
      "A growing draft\nwith multiple lines\nthat stays below the chat",
    );
    await waitFor(assertUncovered);
    expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight + 100);
    viewport.dispatchEvent(new WheelEvent("wheel", { deltaY: -1200 }));
    viewport.scrollTop = 0;
    viewport.dispatchEvent(new Event("scroll"));
    const latest = await c.findByRole("button", { name: "Jump to end" });
    await waitFor(() => expect(latest).toBeVisible());
    latest.focus();
    await userEvent.keyboard("{Enter}");
    await waitFor(() =>
      expect(
        viewport.scrollHeight - viewport.clientHeight - viewport.scrollTop,
      ).toBeLessThan(40),
    );
    assertUncovered();
    await expect(draft).toHaveTextContent("that stays below the chat");
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="First message hover actions remain within the padded chat viewport"
  args={{ chatFeedback: "completed" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const first = await c.findByRole("article", { name: "Message from You" });
    const viewport = canvasElement.querySelector<HTMLElement>(
      '.chat-body-reading-surface [data-ui-part="scroll-area-viewport"]',
    )!;
    viewport.scrollTop = 0;
    viewport.dispatchEvent(new Event("scroll"));
    await browser
      .getByRole("textbox", { name: "Your message", exact: true })
      .hover();
    const actions = first.querySelector<HTMLElement>(".chat-text-actions")!;
    await waitFor(() => {
      expect(
        within(first).getByRole("button", { name: "Comment on Your message" }),
      ).toBeVisible();
      expect(actions.getBoundingClientRect().top).toBeGreaterThanOrEqual(
        viewport.getBoundingClientRect().top,
      );
    });
    const area = canvasElement.querySelector(".chat-document")!;
    await expect(
      Number.parseFloat(getComputedStyle(area).paddingTop),
    ).toBeGreaterThanOrEqual(12);
    await expect(
      within(first).getByRole("button", { name: "Comment on Your message" }),
    ).toBeVisible();
  }}
/>
