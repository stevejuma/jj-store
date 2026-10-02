import { expect, fireEvent, userEvent, waitFor, within } from "storybook/test";
import { expectSelectionSurfaceCoversRow } from "./collaboration-story-assertions.js";

export async function agentSidebarPlay({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const canvas = within(canvasElement);
  await waitFor(() =>
    expect(canvas.getByRole("list", { name: "Agent sessions" })).toBeVisible(),
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
  const response = canvas.getByRole("complementary", {
    name: "Agent response",
  });
  await expect(response).toHaveAttribute("data-ui-part", "sidebar");
  await expect(
    response.querySelector(".store-collaboration__detail-header"),
  ).toBeNull();
  const firstMessage = within(response).getAllByRole("article")[0]!;
  const viewport = response.querySelector(
    '[data-ui-part="scroll-area-viewport"]',
  )!;
  await waitFor(() =>
    expect(
      firstMessage.getBoundingClientRect().top -
        viewport.getBoundingClientRect().top,
    ).toBeLessThan(48),
  );
  const body = canvasElement.querySelector('[data-ui-part="main"]')!;
  await waitFor(() => {
    expect(
      Math.abs(
        response.getBoundingClientRect().top - body.getBoundingClientRect().top,
      ),
    ).toBeLessThan(1);
    expect(
      Math.abs(
        response.getBoundingClientRect().bottom -
          body.getBoundingClientRect().bottom,
      ),
    ).toBeLessThan(1);
  });
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
  const toolCalls = response.querySelector(
    '[data-ui-component="ai-chat-tool-calls"]',
  )!;
  await expect(
    Math.abs(
      toolCalls.getBoundingClientRect().left -
        firstMessage
          .querySelector('[data-ui-component="community-member-avatar"]')!
          .getBoundingClientRect().left,
    ),
  ).toBeLessThan(1);
  const transcript = canvas.getByRole("log", {
    name: "Agent conversation activity",
  });
  const expandedTool = canvas
    .getByRole("button", { name: "Hide details for apply_patch" })
    .closest<HTMLElement>('[data-ui-component="ai-chat-tool-calls"]');
  if (!expandedTool) throw new Error("Expanded tool row was not rendered");
  await expect(expandedTool.getBoundingClientRect().right).toBeLessThanOrEqual(
    transcript.getBoundingClientRect().right,
  );
  const toolReview = canvas
    .getByRole("button", { name: "Hide details for apply_patch" })
    .closest<HTMLElement>('[data-ui-part="agent-tool-review"]');
  if (!toolReview) throw new Error("Tool review surface was not rendered");
  await fireEvent.pointerEnter(toolReview);
  const replyToTool = canvas.getByRole("button", {
    name: "Reply to apply_patch tool call",
  });
  const toolActions = canvas.getByRole("group", {
    name: "Actions for apply_patch tool call",
  });
  await expect(toolActions).toHaveAttribute("data-visible", "");
  await waitFor(() => expect(getComputedStyle(toolActions).opacity).toBe("1"));
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
  await expect(canvas.getByText("Keep this tool result pinned.")).toBeVisible();
  const resize = canvas.getByRole("slider", {
    name: "Resize agent response sidebar",
  });
  const width = resize.getAttribute("aria-valuenow");
  await fireEvent.keyDown(resize, { key: "ArrowRight" });
  await waitFor(() =>
    expect(resize).not.toHaveAttribute("aria-valuenow", width),
  );
  await userEvent.click(canvas.getByRole("button", { name: "Toggle details" }));
  await expect(
    canvas.getByRole("complementary", { name: "Repository details" }),
  ).toBeVisible();
  await expect(response).toBeVisible();
  await userEvent.click(
    canvas.getByRole("button", { name: "Close agent response" }),
  );
  await waitFor(() =>
    expect(
      canvas.queryByRole("complementary", { name: "Agent response" }),
    ).toBeNull(),
  );
  await expect(
    canvas.getByRole("complementary", { name: "Repository details" }),
  ).toBeVisible();
  await expect(sessionSelection).toHaveFocus();
}
