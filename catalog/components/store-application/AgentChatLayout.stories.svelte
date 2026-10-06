<script module lang="ts">
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
      '[data-ui-component="ai-chat-layout"]',
    )!;
    const viewport = layout.querySelector<HTMLElement>(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    const dock = layout.querySelector<HTMLElement>(
      '[data-ui-part="composer-dock"]',
    )!;
    const assertUncovered = () => {
      expect(viewport.getBoundingClientRect().bottom).toBeLessThanOrEqual(
        dock.getBoundingClientRect().top + 1,
      );
      expect(
        c.getByText("Ready to help.").getBoundingClientRect().bottom,
      ).toBeLessThanOrEqual(viewport.getBoundingClientRect().bottom + 1);
    };
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
    const latest = await c.findByRole("button", { name: "Scroll to latest" });
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
