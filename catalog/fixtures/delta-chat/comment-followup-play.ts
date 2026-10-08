import { expect, userEvent, waitFor, within } from "storybook/test";
import { draft } from "./acceptance-helpers.js";
export async function groupedCommentEntries({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Assistant explanation" }),
  );
  await userEvent.type(await draft(canvasElement), "First comment");
  const source = canvasElement.querySelector(
    '[data-placement="inline"]',
  )! as HTMLElement;
  const inline = within(source);
  await userEvent.click(
    inline.getByRole("button", { name: "Add another comment" }),
  );
  const editor = source.querySelector<HTMLElement>(
    '[data-editing="true"] [role="textbox"]',
  )!;
  await userEvent.type(editor, "Second comment");
  expect(source.querySelectorAll(".chat-comment-list")).toHaveLength(1);
  expect(source.querySelectorAll(".chat-comment-card")).toHaveLength(2);
  const list = source.querySelector(".chat-comment-list")!;
  expect(getComputedStyle(list).borderTopWidth).toBe("1px");
  for (const card of source.querySelectorAll(".chat-comment-card"))
    expect(getComputedStyle(card).borderTopWidth).toBe("0px");
  const pending = c.getByRole("group", { name: "Pending comments" });
  const queued = within(pending).getAllByRole("textbox");
  expect(queued).toHaveLength(2);
  await userEvent.click(
    inline.getAllByRole("button", { name: "Go to queued comment" })[1],
  );
  await waitFor(() => expect(queued[1]).toHaveFocus());
  await userEvent.keyboard("{End} continued");
  await expect(editor).toHaveTextContent("Second comment continued");
  await userEvent.keyboard("{Escape}");
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Assistant explanation" }),
  );
  await expect(editor).toHaveFocus();
  expect(source.querySelectorAll(".chat-comment-card")).toHaveLength(2);
  await userEvent.click(c.getByRole("button", { name: "Send message" }));
  await waitFor(() =>
    expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull(),
  );
  expect(inline.getAllByRole("img", { name: "Sent" })).toHaveLength(2);
  expect(inline.queryByText("Sent", { exact: true })).toBeNull();
  const reply = inline.getByRole("button", { name: "Reply" });
  expect(reply.textContent).toBe("");
  inline.getAllByRole("button", { name: "View message" })[0].focus();
  expect(
    getComputedStyle(source.querySelector(".chat-comment-links")!).opacity,
  ).toBe("1");
  await userEvent.click(
    inline.getAllByRole("button", { name: "View message" })[1],
  );
  const history = c.getByRole("group", { name: "Sent comments" });
  await waitFor(() =>
    expect(
      within(history).getAllByRole("article", { name: "Comment by You" })[1],
    ).toHaveFocus(),
  );
  await userEvent.click(reply);
  await waitFor(() =>
    expect(
      source.querySelector('[data-editing="true"] [role="textbox"]'),
    ).toHaveFocus(),
  );
  await userEvent.keyboard("Third comment");
  expect(source.querySelectorAll(".chat-comment-list")).toHaveLength(1);
  expect(source.querySelectorAll(".chat-comment-card")).toHaveLength(3);
}
export async function commentRail({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  const delivered = c.getByRole("button", { name: "Preview turn 2" });
  await userEvent.hover(delivered);
  let popover = await c.findByRole("group", { name: "Comment summary" });
  expect(popover).toHaveTextContent("Comment: show me contents of this file");
  expect(popover).toHaveTextContent("1 comment");
  expect(popover).toHaveTextContent("Codex");
  expect(getComputedStyle(delivered).outlineStyle).toBe("none");
  await userEvent.click(
    within(popover).getByRole("button", { name: "Bookmark turn 2" }),
  );
  expect(
    within(popover).getByRole("button", { name: "Bookmark turn 2" }),
  ).toHaveAttribute("aria-pressed", "true");
  await userEvent.hover(c.getByRole("button", { name: "Preview turn 3" }));
  popover = c.getByRole("group", { name: "Comment summary" });
  expect(popover).toHaveTextContent(
    "Comments: what are the cargo dependencies; how many files in src?",
  );
  expect(popover).toHaveTextContent("2 comments");
  await userEvent.hover(
    c.getByRole("button", { name: "Preview pending comments" }),
  );
  popover = c.getByRole("group", { name: "Comment summary" });
  expect(popover).toHaveTextContent("Comment: is this correct?");
  expect(popover).toHaveTextContent("1 comment · Pending");
  await userEvent.click(
    within(popover).getByRole("button", { name: "Go to pending comments" }),
  );
  await waitFor(() =>
    expect(
      within(c.getByRole("group", { name: "Pending comments" })).getByRole(
        "textbox",
      ),
    ).toHaveFocus(),
  );
  await userEvent.click(delivered);
  await waitFor(() =>
    expect(
      canvasElement.querySelector(
        '[data-placement="history"] [data-comment-id="discussion-0/user"]',
      ),
    ).toHaveFocus(),
  );
}
