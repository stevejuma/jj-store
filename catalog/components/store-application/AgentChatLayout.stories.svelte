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
    const messageInput = c.getByRole("textbox", { name: "Markdown editor" });
    await expect(
      messageInput.closest("[data-chat-next-message]"),
    ).not.toBeNull();
    await expect(
      layout.querySelector('[data-ui-component="ai-chat-composer"]'),
    ).toBeNull();
    let phase = "initial-jump";
    const assertUncovered = () => {
      expect(viewport.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        dock.getBoundingClientRect().top + 1,
      );
      expect(
        c
          .getByRole("textbox", { name: "Markdown editor" })
          .getBoundingClientRect().bottom,
        JSON.stringify({
          phase,
          top: viewport.scrollTop,
          height: viewport.scrollHeight,
          client: viewport.clientHeight,
        }),
      ).toBeLessThanOrEqual(viewport.getBoundingClientRect().bottom + 1);
    };
    await userEvent.click(c.getByRole("button", { name: "Jump to end" }));
    await waitFor(assertUncovered);
    const draft = c.getByRole("textbox", { name: "Markdown editor" });
    await expect(draft).toHaveFocus();
    phase = "typing";
    await userEvent.type(
      draft,
      "A growing draft\nwith multiple lines\nthat stays below the chat",
      { skipClick: true },
    );
    await waitFor(assertUncovered);
    expect(viewport.scrollHeight).toBeGreaterThan(viewport.clientHeight + 100);
    viewport.dispatchEvent(new WheelEvent("wheel", { deltaY: -1200 }));
    viewport.scrollTop = 0;
    viewport.dispatchEvent(new Event("scroll"));
    const latest = await c.findByRole("button", { name: "Jump to end" });
    await waitFor(() => expect(latest).toBeVisible());
    phase = "keyboard-jump";
    latest.focus();
    await userEvent.keyboard("{Enter}");
    await waitFor(() =>
      expect(
        viewport.scrollHeight - viewport.clientHeight - viewport.scrollTop,
      ).toBeLessThan(40),
    );
    await waitFor(assertUncovered);
    await expect(draft).toHaveTextContent("that stays below the chat");
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Jump to end completes after the editor reveals its caret"
  args={{ longHistory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByText("Ready to help.");
    const viewport = canvasElement.querySelector<HTMLElement>(
      '.chat-body-reading-surface [data-ui-part="scroll-area-viewport"]',
    )!;
    const draft = c.getByRole("textbox", { name: "Markdown editor" });
    let revealedCaret = false;
    // Focusing an editor may reveal only its caret; the explicit jump must win.
    draft.addEventListener(
      "focus",
      () => {
        viewport.scrollTo({
          top: viewport.scrollHeight - viewport.clientHeight - 80,
          behavior: "instant",
        });
        revealedCaret = true;
      },
      { once: true },
    );
    await userEvent.click(c.getByRole("button", { name: "Jump to end" }));
    await expect(revealedCaret).toBe(true);
    await expect(draft).toHaveFocus();
    await waitFor(() =>
      expect(draft.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        viewport.getBoundingClientRect().bottom + 1,
      ),
    );
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
