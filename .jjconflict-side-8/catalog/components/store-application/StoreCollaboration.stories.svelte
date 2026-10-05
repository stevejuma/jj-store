<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import { agentSidebarPlay } from "./agent-sidebar-play.js";
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
  play={agentSidebarPlay}
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
    const status = canvas.getByRole("status", {
      name: "Loading agent sessions",
    });
    await expect(status).toBeVisible();
    const heading = status
      .closest(".community-projects__collection-group")!
      .querySelector<HTMLElement>(".community-projects__list-group-heading")!;
    const row = status.querySelector<HTMLElement>(
      ".store-collaboration-skeleton__row",
    )!;
    await expect(
      Math.abs(
        status.getBoundingClientRect().left -
          heading.getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    await expect(
      Math.abs(
        status.getBoundingClientRect().right -
          heading.getBoundingClientRect().right,
      ),
    ).toBeLessThan(1);
    await expect(
      row
        .querySelector("[data-ui-component='skeleton']")!
        .getBoundingClientRect().left - heading.getBoundingClientRect().left,
    ).toBeCloseTo(10.4, 0);
    await expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
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
      within(canvas.getByRole("group", { name: "Review comment" })).getByRole(
        "button",
        { name: "Add comment" },
      ),
    );
    const newComment = canvas.getByText("The pinned root remains exact.");
    await expect(newComment).toBeVisible();
    await expect(
      newComment.closest(".store-collaboration__message"),
    ).toHaveAttribute("data-thread-depth", "0");
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

<Story
  name="Mobile agent response preserves its directory"
  args={{ initialPage: "agents", displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const row = await c.findByRole("button", { name: /Open Codex ended/ });
    await userEvent.click(row);
    const close = await c.findByRole("button", {
      name: "Close agent response",
    });
    const response = c.getByRole("complementary", { name: "Agent response" });
    await expect(response).toHaveAttribute("data-ui-part", "sidebar");
    await expect(
      within(response).getByRole("log", {
        name: "Agent conversation activity",
      }),
    ).toBeVisible();
    await userEvent.click(close);
    await waitFor(() => expect(row).toBeVisible());
    await expect(row).toHaveFocus();
  }}
/>
