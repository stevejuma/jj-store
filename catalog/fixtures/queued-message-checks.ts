import { expect, userEvent, within, waitFor } from "storybook/test";
const input = (c: ReturnType<typeof within>) => {
  const document = c.queryByRole("group", { name: "Next message draft" });
  return document
    ? within(document).getByRole("textbox")
    : c.getByRole("textbox", { name: "Markdown editor" });
};
export function expectPromptTypography(canvasElement: HTMLElement) {
  const response = canvasElement.querySelector(
    ".chat-text-reader .cm-content",
  )!;
  const queued = canvasElement.querySelector(".prompt-queue__text")!;
  expect(response).not.toBeNull();
  expect(queued).not.toBeNull();
  const normal = getComputedStyle(response);
  const pending = getComputedStyle(queued);
  expect(pending.fontSize).toBe(normal.fontSize);
  expect(pending.lineHeight).toBe(normal.lineHeight);
  expect(pending.fontFamily).toBe(normal.fontFamily);
  const card = queued.closest(".chat-comment-list")!;
  const cardStyle = getComputedStyle(card);
  expect(cardStyle.paddingTop).toBe(cardStyle.paddingBottom);
  const text = queued.getBoundingClientRect();
  const avatar = card
    .querySelector(".community-member-avatar")!
    .getBoundingClientRect();
  if (text.height <= avatar.height) {
    expect(
      Math.abs(text.top + text.height / 2 - avatar.top - avatar.height / 2),
    ).toBeLessThan(1);
  }
}
export async function preview(canvasElement: HTMLElement) {
  const c = within(canvasElement);
  await c.findByText("Ready to help.");
  await userEvent.click(input(c));
  await userEvent.keyboard("Review the retry handling after this turn.");
  await userEvent.click(c.getByRole("button", { name: "Queue message" }));
  const pending = await c.findByRole("region", { name: "Queued messages" });
  const collapse = within(pending).getByRole("button", {
    name: "Collapse 1 queued message",
  });
  expect(collapse.querySelector("svg")).toHaveClass("lucide-chevron-right");
  await userEvent.click(collapse);
  await waitFor(() =>
    expect(
      within(pending).getByText("Review the retry handling after this turn."),
    ).not.toBeVisible(),
  );
  const expand = within(pending).getByRole("button", {
    name: "Expand 1 queued message",
  });
  expand.focus();
  await userEvent.keyboard("{Enter}");
  await waitFor(() =>
    expect(
      within(pending).getByText("Review the retry handling after this turn."),
    ).toBeVisible(),
  );
  expect(
    within(pending).getByRole("img", { name: "You avatar" }),
  ).toBeVisible();
  const text = within(pending).getByText(
    "Review the retry handling after this turn.",
  );
  expect(Number(getComputedStyle(text.closest("li")!).opacity)).toBeLessThan(1);
  expect(
    getComputedStyle(text.closest(".chat-comment-list")!).backgroundColor,
  ).not.toBe("rgba(0, 0, 0, 0)");
  await userEvent.click(input(c));
  await userEvent.keyboard("Check the error message too.");
  const row = text.closest("li")!;
  const metadata = row.querySelector(".prompt-queue__metadata")!;
  expect(metadata).toHaveTextContent("Repository helper · Queued");
  expect(text.closest(".chat-comment-list")!.contains(metadata)).toBe(false);
  const card = text.closest("article")!;
  const actions = card.querySelector(".chat-comment-links")!;
  expect(getComputedStyle(actions).opacity).toBe("0");
  within(card)
    .getByRole("button", {
      name: "Cancel queued message for Repository helper",
    })
    .focus();
  expect(getComputedStyle(actions).opacity).toBe("1");
  await userEvent.click(input(c));
  expectPromptTypography(canvasElement);
}

export async function longPreview(canvasElement: HTMLElement) {
  const c = within(canvasElement);
  await c.findByText("Ready to help.");
  const text =
    "Review the retry handling after this turn. " +
    "Keep the captured request and draft intact while checking each recipient. ".repeat(
      12,
    ) +
    "Finish with an exact cancellation check.";
  await userEvent.click(input(c));
  await userEvent.keyboard(text);
  await userEvent.click(c.getByRole("button", { name: "Queue message" }));
  const pending = await c.findByRole("region", { name: "Queued messages" });
  const paragraph = pending.querySelector(".prompt-queue__text")!;
  expect(paragraph.textContent!.trim().length).toBeLessThan(text.length);
  expect(paragraph).not.toHaveTextContent(
    "Finish with an exact cancellation check.",
  );
  const more = within(pending).getByRole("button", { name: "See More" });
  expect(more).toHaveAttribute("aria-expanded", "false");
  more.focus();
  await userEvent.keyboard("{Enter}");
  expect(
    within(pending).getByRole("button", { name: "See Less" }),
  ).toHaveAttribute("aria-expanded", "true");
  expect(paragraph.textContent!.trim()).toBe(text);
  await userEvent.click(c.getByRole("button", { name: "Switch view" }));
  expect(c.getByRole("button", { name: "See Less" })).toBeVisible();
  await userEvent.click(c.getByRole("button", { name: "Toggle chat" }));
  await userEvent.click(c.getByRole("button", { name: "Toggle chat" }));
  expect(c.getByRole("button", { name: "See Less" })).toBeVisible();
  await userEvent.click(c.getByRole("button", { name: "See Less" }));
  expect(c.getByRole("button", { name: "See More" })).toHaveAttribute(
    "aria-expanded",
    "false",
  );
  expectPromptTypography(canvasElement);
  const submitted = JSON.parse(
    c.getByLabelText("Submitted queue commands").textContent ?? "[]",
  );
  expect(submitted).toHaveLength(1);
  expect(submitted[0].action.text).toBe(text);
  await userEvent.click(input(c));
  await userEvent.keyboard("Keep the next draft");
  await userEvent.click(c.getByRole("button", { name: "See More" }));
  expect(input(c)).toHaveTextContent("Keep the next draft");
  await userEvent.click(c.getByRole("button", { name: "See Less" }));
  await userEvent.click(c.getByRole("button", { name: "Switch view" }));
}
