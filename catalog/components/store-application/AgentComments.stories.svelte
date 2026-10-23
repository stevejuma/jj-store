<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentLayoutFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent session comments",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
  async function comments(canvasElement: HTMLElement, focus: boolean) {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const detail = focus
      ? await body.findByRole("region", { name: "Agent response" })
      : await c.findByRole("complementary", { name: "Agent response" });
    await userEvent.click(
      within(detail).getByRole("button", { name: "Add review" }),
    );
    const footer = await within(detail).findByRole("group", {
      name: "Agent session comment",
    });
    const editor = within(footer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "A comment about the whole session");
    await userEvent.click(
      within(footer).getByRole("button", { name: "Add comment" }),
    );
    await within(detail).findByText("A comment about the whole session", {
      exact: true,
    });
    await expect(editor).not.toHaveTextContent(
      "A comment about the whole session",
    );
    await expect(
      within(footer).getByRole("button", { name: "Add comment" }),
    ).toBeDisabled();
    const comment = within(detail).getByText(
      "A comment about the whole session",
      { exact: true },
    );
    await expect(
      comment.closest('[data-ui-part="agent-tool-review"]'),
    ).toBeNull();
    const header = detail.querySelector(
      '[data-ui-part="surface-layer-header"], [data-ui-part="sidebar-header"]',
    )!;
    const beforeHeader = header.getBoundingClientRect(),
      beforeFooter = footer.getBoundingClientRect();
    const viewport = detail.querySelector<HTMLElement>(
      '[data-slot="scroll-area-viewport"]',
    )!;
    viewport.scrollTop = viewport.scrollHeight;
    await new Promise((resolve) => requestAnimationFrame(resolve));
    await expect(viewport.scrollTop).toBeGreaterThan(0);
    await expect(header.getBoundingClientRect().top).toBe(beforeHeader.top);
    await expect(footer.getBoundingClientRect().bottom).toBe(
      beforeFooter.bottom,
    );
    await userEvent.type(editor, "Retained session draft");
    await userEvent.click(
      within(detail).getByRole("button", { name: /Close agent response/i }),
    );
    await waitFor(() =>
      expect(
        body.queryByRole(focus ? "region" : "complementary", {
          name: "Agent response",
        }),
      ).toBeNull(),
    );
    await userEvent.click(c.getByRole("button", { name: /Open Codex ended/ }));
    const reopened = focus
      ? await body.findByRole("region", { name: "Agent response" })
      : await c.findByRole("complementary", { name: "Agent response" });
    await expect(
      within(
        within(reopened).getByRole("group", { name: "Agent session comment" }),
      ).getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Retained session draft");
    await within(reopened).findByText("A comment about the whole session", {
      exact: true,
    });
  }
</script>

<Story
  name="Focus keeps session comments and the footer anchored"
  args={{ conversationView: "classic", repository: true, focus: true }}
  play={async ({ canvasElement }) => comments(canvasElement, true)}
/>
<Story
  name="Split keeps session comments and the footer anchored"
  args={{ conversationView: "classic", repository: true, focus: false }}
  play={async ({ canvasElement }) => comments(canvasElement, false)}
/>

<Story
  name="Read-only session responses have no writable footer"
  args={{
    conversationView: "classic",
    repository: true,
    focus: false,
    readOnly: true,
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const detail = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await expect(
      within(detail).queryByRole("group", { name: "Agent session comment" }),
    ).toBeNull();
    const tool = within(detail).getByRole("group", {
      name: "apply_patch tool call",
    });
    await userEvent.hover(
      within(tool).getByRole("group", { name: /tool row$/ }),
    );
    await expect(
      within(tool).queryByRole("button", {
        name: "Reply to apply_patch tool call",
      }),
    ).toBeNull();
  }}
/>
<Story
  name="Unknown session comment outcomes retain the footer draft until checked"
  args={{
    conversationView: "classic",
    repository: true,
    focus: false,
    reviewOutcome: "unknown",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const detail = await c.findByRole("complementary", {
      name: "Agent response",
    });
    const disclosure = within(detail).getByRole("button", {
      name: "Add review",
    });
    await userEvent.click(disclosure);
    const footer = within(detail).getByRole("group", {
      name: "Agent session comment",
    });
    const editor = within(footer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "Retain until receipt");
    await userEvent.click(
      within(footer).getByRole("button", { name: "Add comment" }),
    );
    await expect(editor).toHaveTextContent("Retain until receipt");
    await expect(disclosure).toHaveAttribute("aria-expanded", "true");
    await expect(disclosure).toBeDisabled();
    await expect(
      within(footer).getByRole("button", { name: "Add comment" }),
    ).toBeDisabled();
    await userEvent.click(
      within(detail).getByRole("button", { name: "Check comment status" }),
    );
    await within(detail).findByText("Retain until receipt", { exact: true });
    await expect(editor).not.toHaveTextContent("Retain until receipt");
  }}
/>

<Story
  name="Tool and nested comment outcomes retain drafts until their receipt is checked"
  args={{
    conversationView: "classic",
    repository: true,
    focus: false,
    reviewOutcome: "unknown",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: /Open Codex ended/ }),
    );
    const detail = await c.findByRole("complementary", {
      name: "Agent response",
    });
    const tool = within(detail).getByRole("group", {
      name: "apply_patch tool call",
    });
    await userEvent.hover(
      within(tool).getByRole("group", { name: /tool row$/ }),
    );
    await userEvent.click(
      within(tool).getByRole("button", {
        name: "Reply to apply_patch tool call",
      }),
    );
    const toolComposer = tool.querySelector<HTMLElement>(
      '[data-ui-part="agent-tool-reply-composer"]',
    )!;
    const editor = within(toolComposer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(editor, "Tool comment awaiting receipt");
    await userEvent.click(
      within(toolComposer).getByRole("button", { name: "Reply" }),
    );
    await expect(editor).toHaveTextContent("Tool comment awaiting receipt");
    await expect(
      within(toolComposer).getByRole("button", { name: "Reply" }),
    ).toBeDisabled();
    await userEvent.click(
      within(toolComposer).getByRole("button", {
        name: "Check comment status",
      }),
    );
    const posted = await within(tool).findByText(
      "Tool comment awaiting receipt",
      { exact: true },
    );
    await expect(editor).not.toHaveTextContent("Tool comment awaiting receipt");
    const comment = posted.closest<HTMLElement>(".community-message")!;
    await userEvent.hover(comment);
    await waitFor(() =>
      expect(
        getComputedStyle(
          comment.querySelector('[data-ui-part="message-actions"]')!,
        ).opacity,
      ).toBe("1"),
    );
    await userEvent.click(
      within(comment).getByRole("button", { name: "Reply" }),
    );
    const replyComposer = tool.querySelector<HTMLElement>(
      ".store-collaboration__composer",
    )!;
    const replyEditor = within(replyComposer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(replyEditor, "Nested reply awaiting receipt");
    await userEvent.click(
      within(replyComposer).getByRole("button", { name: "Comment" }),
    );
    await expect(replyEditor).toHaveTextContent(
      "Nested reply awaiting receipt",
    );
    await expect(
      within(replyComposer).getByRole("button", { name: "Comment" }),
    ).toBeDisabled();
    await userEvent.click(
      within(replyComposer).getByRole("button", {
        name: "Check comment status",
      }),
    );
    const reply = await within(tool).findByText(
      "Nested reply awaiting receipt",
      { exact: true },
    );
    await expect(reply.closest("[data-thread-depth]")).toHaveAttribute(
      "data-thread-depth",
      "2",
    );
  }}
/>
