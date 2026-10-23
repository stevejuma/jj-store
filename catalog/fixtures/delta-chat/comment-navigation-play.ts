import { storyControl } from "../story-controls.js";
import { expect, userEvent, waitFor, within } from "storybook/test";
import { reader, draft, addComment } from "./acceptance-helpers.js";
export const collapsedComments = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Expand Read a file, ran a command" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Show details for Read example.ts" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Captured example.ts" }),
  );
  await userEvent.type(
    await draft(canvasElement, "Captured example.ts"),
    "Read comment",
  );
  await userEvent.keyboard("{Escape}");
  await userEvent.click(
    c.getByRole("button", { name: "Close read tool details" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Collapse Read a file, ran a command" }),
  );
  const summaries = () =>
    canvasElement.querySelectorAll('.chat-document [data-placement="summary"]');
  await expect(summaries()).toHaveLength(1);
  await expect(summaries()[0]).toHaveTextContent("Read comment");
  const pending = c.getByRole("group", { name: "Pending comments" });
  await userEvent.click(
    within(pending).getByRole("button", {
      name: "Go to source: Captured example.ts",
    }),
  );
  await waitFor(() =>
    expect(
      c.getByRole("textbox", { name: "Captured example.ts" }),
    ).toHaveFocus(),
  );
  await expect(
    c.getByRole("button", { name: "Collapse Read a file, ran a command" }),
  ).toBeVisible();
  await userEvent.click(
    c.getByRole("button", { name: "Show raw output for Read example.ts" }),
  );
  await expect(
    await reader(canvasElement, "Raw captured data for Read example.ts"),
  ).toHaveTextContent("answer");
  await userEvent.click(
    within(pending).getByRole("button", {
      name: "Go to source: Captured example.ts",
    }),
  );
  await waitFor(() =>
    expect(
      c.getByRole("textbox", { name: "Captured example.ts" }),
    ).toHaveFocus(),
  );
  await addComment(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Close read tool details" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Captured patch example.ts" }),
  );
  await userEvent.type(
    await draft(canvasElement, "Captured patch example.ts"),
    "Patch comment",
  );
  await userEvent.keyboard("{Escape}");
  await userEvent.click(
    c.getByRole("button", { name: "Hide captured file example.ts" }),
  );
  await expect(summaries()).toHaveLength(2);
  await expect(summaries()[1]).toHaveTextContent("Patch comment");
  await userEvent.click(
    c.getByRole("button", { name: "Hide details for Apply patch example.ts" }),
  );
  await expect(summaries()).toHaveLength(2);
  const queue = c.getByRole("group", { name: "Pending comments" });
  await userEvent.click(
    within(queue).getByRole("button", {
      name: "Go to source: Captured patch example.ts",
    }),
  );
  await expect(
    await reader(canvasElement, "Captured patch example.ts"),
  ).toHaveFocus();
  await storyControl(document.body, "Hide body");
  await storyControl(document.body, "Show body");
  await expect(
    c.getByRole("group", { name: "Pending comments" }),
  ).toHaveTextContent("Patch comment");
};
export const deltaCommentReference = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  const pending = await c.findByRole("group", { name: "Pending comments" });
  await expect(pending).toHaveTextContent(
    "This ignores Rust’s build output directory",
  );
  for (const id of ["reply-0", "reply-1", "reply-2"]) {
    const reply = canvasElement.querySelector(`[data-chat-item="${id}"]`)!;
    await waitFor(() =>
      expect(reply.querySelectorAll(".chat-reply-indicator")).toHaveLength(1),
    );
    expect(
      reply.querySelector(".cm-line .chat-reply-indicator"),
    ).not.toBeNull();
  }

  await expect(within(pending).getByRole("textbox")).toHaveTextContent(
    "is this correct?",
  );
  await expect(
    c.getByRole("textbox", { name: "Next message" }),
  ).toHaveTextContent("Normal chat input");
  await expect(c.getAllByRole("button", { name: /Preview turn/ })).toHaveLength(
    3,
  );
  await userEvent.click(
    within(pending).getByRole("button", { name: /Go to source/ }),
  );
  await waitFor(() =>
    expect(document.activeElement?.getAttribute("aria-label")).toBe(
      "Agent reply",
    ),
  );
};
