import { storyControl } from "../story-controls.js";
import { expect, userEvent, waitFor, within } from "storybook/test";
import {
  reader,
  draft,
  addComment,
  inlineComments,
} from "./acceptance-helpers.js";
export const commentLifecycle = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement),
    source = await reader(canvasElement);
  await userEvent.click(source);
  await userEvent.keyboard("Please clarify");
  const inline = await draft(canvasElement);
  const pending = c.getByRole("group", { name: "Pending comments" });
  const queued = within(pending).getByRole("textbox", {
    name: "Comment on Assistant explanation",
  });
  await expect(queued).toHaveTextContent("Please clarify");
  await userEvent.click(queued);
  await userEvent.keyboard("{End} more");
  await expect(inline).toHaveTextContent("Please clarify more");
  await expect(queued).toHaveFocus();
  await userEvent.keyboard("{Enter}Second line");
  await expect(inline.querySelectorAll(".cm-line")).toHaveLength(2);
  await storyControl(document.body, "Fail next send");
  await userEvent.click(c.getByRole("button", { name: "Send message" }));
  await expect(await c.findByRole("alert")).toHaveTextContent("retained");
  await expect(queued).toHaveTextContent("Second line");
  await userEvent.click(queued);
  await userEvent.keyboard(
    navigator.platform.includes("Mac")
      ? "{Meta>}{Enter}{/Meta}"
      : "{Control>}{Enter}{/Control}",
  );
  await waitFor(() =>
    expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull(),
  );
  await expect(c.getByRole("textbox", { name: "Next message" })).toHaveFocus();
  await expect(
    inlineComments(canvasElement).getByRole("article", {
      name: "Comment by You",
    }),
  ).toHaveTextContent("Please clarify more");
  await expect(c.queryByRole("button", { name: "Go to reply" })).toBeNull();
  await userEvent.click(
    c.getByRole("button", { name: "Expand 1 sent comments" }),
  );
  const history = c.getByRole("group", { name: "Sent comments" });
  await expect(within(history).queryByRole("textbox")).toBeNull();
  await expect(history).toHaveTextContent("Second line");
  await userEvent.click(
    c.getByRole("button", { name: "Collapse 1 sent comments" }),
  );
  await storyControl(document.body, "Simulate replies");
  const sourceArea = canvasElement.querySelector(
    '[data-placement="inline"]',
  )! as HTMLElement;
  await expect(
    within(sourceArea).getByRole("button", { name: "Go to reply" }),
  ).toBeVisible();
  await userEvent.click(
    within(sourceArea).getByRole("button", { name: "Go to reply" }),
  );
  await expect(
    c.getByRole("textbox", { name: "Simulated reply" }).closest("li"),
  ).toHaveFocus();
  await userEvent.click(
    within(sourceArea).getByRole("button", { name: "Reply" }),
  );
  await userEvent.keyboard("Follow up");
  await expect(await draft(canvasElement)).toHaveTextContent("Follow up");
  await expect(
    within(sourceArea).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
  await userEvent.keyboard("{Escape}");
  await expect(source).toHaveFocus();
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Assistant explanation" }),
  );
  await expect(await draft(canvasElement)).toHaveTextContent("Follow up");
  await expect(
    within(sourceArea).getAllByRole("article", { name: "Comment by You" }),
  ).toHaveLength(2);
};
export const unavailableDraft = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await userEvent.click(await reader(canvasElement));
  await userEvent.keyboard("Keep this draft");
  await storyControl(document.body, "Replace content");
  await expect(
    c.getByRole("complementary", { name: "Comments on unavailable content" }),
  ).toHaveTextContent("unavailable or changed");
  const pending = c.getByRole("group", { name: "Pending comments" });
  await expect(within(pending).getByRole("textbox")).toHaveTextContent(
    "Keep this draft",
  );
  await expect(
    within(pending).getByRole("button", { name: /Go to source/ }),
  ).toBeDisabled();
  await userEvent.click(
    within(pending).getByRole("button", { name: "Remove pending comment" }),
  );
  await expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull();
};
export const retainedDraft = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement),
    source = await reader(canvasElement);
  source.focus();
  await userEvent.paste("Pasted comment");
  const editor = await draft(canvasElement);
  await expect(editor).toHaveTextContent("Pasted comment");
  await userEvent.keyboard("{Escape}");
  await expect(source).toHaveFocus();
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Assistant explanation" }),
  );
  await expect(await draft(canvasElement)).toHaveTextContent("Pasted comment");
  await userEvent.click(
    within(c.getByRole("group", { name: "Pending comments" })).getByRole(
      "button",
      { name: "Remove pending comment" },
    ),
  );
  await expect(c.queryByRole("group", { name: "Pending comments" })).toBeNull();
  source.focus();
  await userEvent.keyboard("New comment");
  await addComment(canvasElement);
  await expect(
    inlineComments(canvasElement).getByRole("article", {
      name: "Comment by You",
    }),
  ).toHaveTextContent("New comment");
};
