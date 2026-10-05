import { expect, userEvent, waitFor, within } from "storybook/test";
import { threadTreeIndentRem } from "../../../web/src/lib/ui/community-thread-panel/thread-tree-layout.js";

export async function agentCommentReplies({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await userEvent.click(
    await c.findByRole("button", { name: /Open Codex ended/ }),
  );
  await userEvent.click(c.getByRole("button", { name: "Expand 3 tool calls" }));
  const tool = c.getByRole("group", { name: "apply_patch tool call" });
  await within(tool).findByText(
    "The nested follow-up stays attached to the reply branch.",
    { exact: true },
  );
  const rows = Array.from(
    tool.querySelectorAll<HTMLElement>(
      ".store-collaboration__message[data-thread-depth]",
    ),
  );
  await expect(rows.map((row) => Number(row.dataset.threadDepth))).toEqual([
    1, 2, 3,
  ]);
  const left = rows[0]!
    .querySelector(".community-message")!
    .getBoundingClientRect().left;
  for (const row of rows) {
    const depth = Number(row.dataset.threadDepth);
    const indent =
      threadTreeIndentRem(depth) *
      parseFloat(getComputedStyle(document.documentElement).fontSize);
    await expect(
      Math.abs(
        row.querySelector(".community-message")!.getBoundingClientRect().left -
          left -
          indent,
      ),
    ).toBeLessThan(1);
    if (depth > 1) {
      const connector = row.querySelector<HTMLElement>(
        '[data-ui-part="review-reply-connector"]',
      )!;
      await expect(getComputedStyle(connector).borderInlineStartWidth).toBe(
        "1px",
      );
      await expect(getComputedStyle(connector).borderEndStartRadius).toBe(
        "16px",
      );
    }
  }
  const comment = rows[0]!.querySelector<HTMLElement>(".community-message")!;
  await userEvent.hover(comment);
  await waitFor(() =>
    expect(
      getComputedStyle(
        comment.querySelector('[data-ui-part="message-actions"]')!,
      ).opacity,
    ).toBe("1"),
  );
  await userEvent.click(within(comment).getByRole("button", { name: "Reply" }));
  const composer = tool.querySelector<HTMLElement>(
    ".store-collaboration__composer",
  )!;
  await userEvent.type(
    within(composer).getByRole("textbox", { name: "Markdown editor" }),
    "New reply to the tool comment",
  );
  await userEvent.click(
    within(composer).getByRole("button", { name: "Comment" }),
  );
  const reply = await within(tool).findByText("New reply to the tool comment", {
    exact: true,
  });
  await expect(reply.closest("[data-thread-depth]")).toHaveAttribute(
    "data-thread-depth",
    "2",
  );
}
