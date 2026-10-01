<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import StoreRepositoryApplication from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { expectSelectionSurfaceCoversRow } from "./collaboration-story-assertions.js";

  const { Story } = defineMeta({
    title: "JJ Store/Collaboration",
    component: StoreRepositoryApplication,
    tags: ["visual-pending"],
    args: { initiallyCollapsed: true },
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: { story: { inline: false, iframeHeight: 700 } },
    },
  });
</script>

<Story
  name="Agent response activity"
  args={{ initialPage: "agents" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await waitFor(() =>
      expect(
        canvas.getByRole("list", { name: "Agent sessions" }),
      ).toBeVisible(),
    );
    const sessionSelection = canvas.getByRole("button", {
      name: /Codex ended/u,
    });
    const sessionCard = await expectSelectionSurfaceCoversRow(sessionSelection);
    const avatar = sessionCard.querySelector(
      '[data-ui-component="community-member-avatar"]',
    );
    if (!avatar) throw new Error("Agent session card avatar was not rendered");
    await expect(avatar).toBeVisible();
    await expect(avatar.querySelector("img")).toHaveAttribute(
      "src",
      "/agent-icons/codex.svg",
    );
    await expect(sessionCard).toHaveTextContent("Started");
    await expect(sessionCard).toHaveTextContent("5m");
    await expect(
      within(sessionCard).getByLabelText("1 transcripts"),
    ).toBeVisible();
    await expect(within(sessionCard).getByLabelText("3 tools")).toBeVisible();
    await userEvent.click(sessionSelection);
    await expect(sessionCard).toHaveAttribute("data-selected", "true");
    await expect(
      canvas.getByRole("log", { name: "Agent conversation activity" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: /Copy full agent root/u }),
    ).toBeNull();
    await expect(sessionCard).toHaveAttribute("data-selected", "true");
    await expect(
      canvas.getByText(
        "Update the native parser and keep its review anchors exact.",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByText("Updated the parser and verified the focused contract."),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Show details for apply_patch" }),
    );
    await expect(
      canvas.getByRole("region", { name: "Changed file src/lib.rs" }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: /Copy full tool call ID/u }),
    ).toBeNull();
    const transcript = canvas.getByRole("log", {
      name: "Agent conversation activity",
    });
    const expandedTool = canvas
      .getByRole("button", { name: "Hide details for apply_patch" })
      .closest<HTMLElement>('[data-ui-component="ai-chat-tool-calls"]');
    if (!expandedTool) throw new Error("Expanded tool row was not rendered");
    await expect(
      expandedTool.getBoundingClientRect().right,
    ).toBeLessThanOrEqual(transcript.getBoundingClientRect().right);
    const toolReview = expandedTool.closest<HTMLElement>(
      '[data-ui-part="agent-tool-review"]',
    );
    if (!toolReview) throw new Error("Tool review surface was not rendered");
    await fireEvent.pointerEnter(toolReview);
    const replyToTool = canvas.getByRole("button", {
      name: "Reply to apply_patch tool call",
    });
    const toolActions = canvas.getByRole("group", {
      name: "Actions for apply_patch tool call",
    });
    await expect(toolActions).toHaveAttribute("data-visible", "");
    await waitFor(() =>
      expect(getComputedStyle(toolActions).opacity).toBe("1"),
    );
    await fireEvent.click(replyToTool);
    let toolComposer: HTMLElement | null = null;
    await waitFor(() => {
      toolComposer = canvasElement.querySelector<HTMLElement>(
        '[data-ui-part="agent-tool-reply-composer"]',
      );
      expect(toolComposer).not.toBeNull();
    });
    if (!toolComposer) throw new Error("Tool reply composer was not rendered");
    const toolEditor = within(toolComposer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(toolEditor, "Keep this tool result pinned.");
    await expect(toolEditor).toHaveFocus();
    await expect(getComputedStyle(toolEditor).outlineStyle).toBe("none");
    await userEvent.click(
      within(toolComposer).getByRole("button", { name: "Reply" }),
    );
    await expect(
      canvas.getByText("Keep this tool result pinned."),
    ).toBeVisible();
    const resize = canvas.getByRole("separator", {
      name: "Resize Agent sessions and Agent response columns",
    });
    const width = resize.getAttribute("aria-valuenow");
    await fireEvent.keyDown(resize, { key: "ArrowRight" });
    await waitFor(() =>
      expect(resize).not.toHaveAttribute("aria-valuenow", width),
    );
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Revision collaboration context"
  args={{ initialPage: "log" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getAllByRole("group", { name: "Collaboration participants" })[0],
    ).toBeVisible();
    await expect(canvas.getAllByLabelText(/review replies/u)[0]).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Merge native storage and publication",
      }),
    );
    await expect(
      canvas.getByRole("button", { name: "Close Agent context column" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Close Review discussion column" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("link", { name: /Open related change/u }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    const details = canvas.getByRole("complementary", {
      name: "Repository details",
    });
    await expect(within(details).queryByText("Selected change")).toBeNull();
    await expect(within(details).getByText("JJ Log/Changes")).toBeVisible();
    await expect(
      within(details).getByRole("button", { name: "Add change" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await expect(
      canvas.getByText("src/lib.rs", {
        selector: ".store-collaboration__detail-title",
      }),
    ).toBeVisible();
    const lineComment = canvas.getAllByRole("button", {
      name: /Comment on src\/lib\.rs line/u,
    })[0]!;
    const diffRow = lineComment.closest<HTMLElement>(
      '[data-ui-part="diff-row"]',
    );
    if (!diffRow) throw new Error("Commentable diff row was not rendered");
    await userEvent.hover(diffRow);
    lineComment.focus();
    await fireEvent.click(lineComment);
    const lineComposer = canvasElement.querySelector<HTMLElement>(
      '[data-ui-part="diff-line-composer"]',
    );
    if (!lineComposer) throw new Error("Diff line composer was not rendered");
    const lineEditor = within(lineComposer).getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.type(lineEditor, "Keep this exact line covered.");
    await expect(lineEditor).toHaveFocus();
    await expect(getComputedStyle(lineEditor).outlineStyle).toBe("none");
    await userEvent.click(
      within(lineComposer).getByRole("button", {
        name: "Comment",
      }),
    );
    await expect(
      canvas.getAllByText("Keep this exact line covered.")[0],
    ).toBeVisible();
    await expect(
      diffRow.nextElementSibling?.querySelector(
        '[data-ui-part="diff-line-review"]',
      ) ?? canvasElement.querySelector('[data-ui-part="diff-line-review"]'),
    ).not.toBeNull();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Operation collaboration context"
  args={{ initialPage: "oplog" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getAllByRole("group", { name: "Collaboration participants" })[0],
    ).toBeVisible();
    await expect(canvas.getAllByLabelText(/review replies/u)[0]).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "describe: native object history" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Close Agent context column" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Close Review discussion column" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "src/lib.rs" }));
    await expect(
      canvas.getAllByRole("list", { name: "Review comments" })[0],
    ).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Agent activity loading"
  args={{ initialPage: "agents", collaborationState: "loading" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByRole("status", { name: "Loading agent sessions" }),
    ).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review discussion empty"
  args={{ initialPage: "reviews", collaborationState: "empty" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("No review threads")).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Agent activity unavailable"
  args={{ initialPage: "agents", collaborationState: "failed" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("Agent activity unavailable")).toBeVisible();
    await expect(
      canvas.getAllByText("The collaboration snapshot could not be loaded.")[0],
    ).toBeVisible();
    await expect(canvas.getByText("No agent sessions")).toBeVisible();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review discussion actions"
  args={{ initialPage: "reviews" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await waitFor(() =>
      expect(
        canvas.getByRole("list", { name: "Review threads" }),
      ).toBeVisible(),
    );
    const agentReviewSelection = canvas.getByRole("button", {
      name: /Open Agent session open review with 2 comments/u,
    });
    const agentReviewCard = agentReviewSelection.closest<HTMLElement>(
      "article.community-projects__review-row",
    );
    if (!agentReviewCard)
      throw new Error("Community review row was not rendered");
    await expect(agentReviewCard.querySelector("img")).toHaveAttribute(
      "src",
      "/agent-icons/codex.svg",
    );
    await expect(
      within(agentReviewCard).getByLabelText("2 comments"),
    ).toBeVisible();
    await userEvent.click(agentReviewSelection);
    await expect(
      canvasElement.querySelector(".community-projects__review-meta img"),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to reviews" }),
    );
    const reviewSelection = canvas.getAllByRole("button", {
      name: /Revision open/u,
    })[0]!;
    await expect(
      reviewSelection.closest("article.community-projects__review-row"),
    ).not.toBeNull();
    await userEvent.click(reviewSelection);
    await expect(
      canvas.getByRole("article", { name: "Review details" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("list", { name: "Review comments" }),
    ).toBeVisible();
    await expect(
      within(canvas.getByRole("list", { name: "Review comments" })).getByText(
        "Can we preserve the exact operation when this retry races?",
      ),
    ).toBeVisible();
    await expect(
      canvas.getByText(
        "The nested follow-up stays attached to the reply branch.",
      ),
    ).toBeVisible();
    const reviewComments = canvas.getByRole("list", {
      name: "Review comments",
    });
    const commentRows = Array.from(
      reviewComments.querySelectorAll<HTMLElement>(
        ".store-collaboration__message",
      ),
    );
    await expect(commentRows).toHaveLength(3);
    await expect(getComputedStyle(commentRows[0]!).borderBottomWidth).toBe(
      "0px",
    );
    await expect(commentRows[2]!.dataset.threadDepth).toBe("2");
    await expect(
      commentRows[2]!.querySelector('[data-ui-part="review-reply-connector"]'),
    ).not.toBeNull();
    await expect(
      canvas.queryByRole("separator", {
        name: "Resize Review threads and Discussion columns",
      }),
    ).toBeNull();
    const editor = canvas.getByRole("textbox", {
      name: "Markdown editor",
    });
    await userEvent.click(editor);
    await userEvent.type(editor, "The pinned root remains exact.");
    await expect(editor).toHaveFocus();
    await expect(getComputedStyle(editor).outlineStyle).toBe("none");
    await userEvent.click(
      within(
        canvas.getByRole("group", { name: "Reply to review thread" }),
      ).getByRole("button", { name: "Reply" }),
    );
    await expect(
      canvas.getByText("The pinned root remains exact."),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Resolve" }));
    await expect(
      canvas.getAllByText("resolved", { exact: true }).at(-1),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("textbox", { name: "Markdown editor" }),
    ).not.toBeInTheDocument();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>

<Story
  name="Review outcome unknown"
  args={{ initialPage: "reviews", reviewOutcome: "unknown" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getAllByRole("button", { name: /Revision open/u })[0]!,
    );
    await userEvent.click(canvas.getByRole("button", { name: "Resolve" }));
    await expect(canvas.getByText("Review outcome unknown")).toBeVisible();
    await expect(canvas.getByText(/outcome is unknown/u)).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Check status" }));
    await expect(canvas.getByText("Review updated")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Check status" }),
    ).not.toBeInTheDocument();
  }}
>
  {#snippet template(args)}<StoreRepositoryApplication {...args} />{/snippet}
</Story>
