import { expect, userEvent, waitFor, within } from "storybook/test";

export async function referenceDiscussionLinks(canvasElement: HTMLElement) {
  delete canvasElement.dataset.chatReferenceReady;
  const c = within(canvasElement);
  const pane = c.getByRole("complementary", { name: "Read tool details" });
  const readDiscussion = within(pane).getByRole("group", {
    name: "Comments on README.md",
  });
  expect(
    within(readDiscussion).getByRole("img", { name: "Sent" }),
  ).toBeVisible();
  expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull();
  expect(c.getByRole("button", { name: "Send message" })).toBeDisabled();
  await userEvent.click(
    within(readDiscussion).getByRole("button", { name: "View message" }),
  );
  const history = c.getByRole("group", { name: "Sent comments" });
  const cards = within(history).getAllByRole("article", {
    name: "Comment by You",
  });
  expect(cards).toHaveLength(4);
  expect(
    within(history).getAllByText("case screenBlock", { selector: "code" }),
  ).toHaveLength(1);
  for (const area of history.querySelectorAll(".chat-comment-area")) {
    const quote = getComputedStyle(
      area.querySelector(".chat-comment-quote > code, .chat-comment-quote p")!,
    );
    const body = getComputedStyle(
      area.querySelector(".chat-comment-content p")!,
    );
    expect(quote.fontFamily).toBe(body.fontFamily);
    expect(quote.fontSize).toBe(body.fontSize);
    expect(quote.lineHeight).toBe(body.lineHeight);
  }
  await waitFor(() => expect(cards[2]).toHaveFocus());
  await userEvent.click(
    within(cards[0]).getByRole("button", { name: "Go to reply" }),
  );
  const reply = canvasElement.querySelector<HTMLElement>(
    '[data-chat-item="demo-review:action/reply"]',
  )!;
  await expect(reply).toHaveFocus();
  expect(reply).toHaveTextContent("Simulated reply:");
  expect(reply.querySelectorAll(".chat-reply-indicator")).toHaveLength(1);
  expect(
    reply.querySelector(".chat-reply-indicator .lucide-corner-up-left"),
  ).not.toBeNull();
  const ordinary = canvasElement.querySelector(
    '[data-chat-item="assistant-63803"]',
  )!;
  expect(ordinary).not.toBeNull();
  expect(ordinary.querySelector(".chat-reply-indicator")).toBeNull();
  expect(
    within(reply).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
  expect(reply.querySelector(".chat-comment-quote")).toHaveTextContent(
    "case screenBlock",
  );
  await userEvent.click(
    c.getByRole("button", { name: "Collapse 4 sent comments" }),
  );
  expect(
    within(reply).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
  await userEvent.click(
    c.getByRole("button", { name: "Expand 4 sent comments" }),
  );
  expect(
    within(reply).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
  await userEvent.click(
    within(reply).getAllByRole("button", { name: /Go to source/ })[0],
  );
  const source = c.getByRole("textbox", {
    name: "Captured patch · Sources/BedtimeCore/Schedule.swift",
  });
  await waitFor(() => expect(source).toHaveFocus());
  const discussion = source
    .closest(".chat-text-block")!
    .querySelector<HTMLElement>(
      '[data-placement="inline"][data-thread-id="demo-review:action"]',
    )!;
  expect(discussion.querySelectorAll(".chat-comment-list")).toHaveLength(1);
  expect(
    within(discussion).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
  expect(
    within(discussion).getAllByRole("button", { name: "Go to reply" }),
  ).toHaveLength(2);
  expect(within(discussion).getAllByRole("article")).toHaveLength(2);
  expect(discussion).not.toHaveTextContent("Simulated reply:");
  const jump = within(discussion).getAllByRole("button", {
    name: "Go to reply",
  })[1];
  within(discussion)
    .getByRole("button", { name: /^Reply$/ })
    .focus();
  await userEvent.tab();
  await expect(jump).toHaveFocus();
  await waitFor(() =>
    expect(within(document.body).getByRole("tooltip")).toHaveTextContent(
      "Go to reply",
    ),
  );
  expect(
    getComputedStyle(jump.querySelector(".chat-comment-delivered-icon")!)
      .display,
  ).toBe("none");
  expect(
    getComputedStyle(jump.querySelector(".chat-comment-jump-icon")!).display,
  ).not.toBe("none");
  await userEvent.keyboard("{Enter}");
  await expect(reply).toHaveFocus();
  // Restore the captured-read view with its seeded inline discussion visible.
  within(pane).getByRole("textbox", { name: "README.md" }).focus();
  await waitFor(() =>
    expect(
      within(pane).getByRole("textbox", { name: "README.md" }),
    ).toHaveFocus(),
  );
  canvasElement.dataset.chatReferenceReady = "true";
}
