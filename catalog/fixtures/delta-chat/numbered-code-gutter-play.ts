import { expect, userEvent, waitFor, within } from "storybook/test";
import { EditorView } from "@codemirror/view";
import { reader, draft } from "./acceptance-helpers.js";

export async function numberedCodeGutter({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  await reader(canvasElement, "User message");
  await userEvent.click(
    c.getByRole("button", {
      name: "Expand Listed a directory, ran a command, read 7 files, ran a search, thought",
    }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Show details for Read README.md" }),
  );
  const source = await reader(canvasElement, "README.md");
  const toolbar = canvasElement.querySelector<HTMLElement>(
    '[data-ui-part="toolbar"]',
  )!;
  const heading = canvasElement.querySelector<HTMLElement>(
    '[data-ui-part="sidebar-header"]',
  )!;
  expect(toolbar.getBoundingClientRect().height).toBe(
    heading.getBoundingClientRect().height,
  );
  expect(toolbar.getBoundingClientRect().bottom).toBe(
    heading.getBoundingClientRect().bottom,
  );
  expect(getComputedStyle(toolbar).borderBottomWidth).toBe("1px");
  const editor = source.closest(".cm-editor")!;
  const gutter = editor.querySelector(".chat-capture-gutter");
  expect(gutter).not.toBeNull();
  await waitFor(() =>
    expect(gutter!.querySelector('[data-captured-line="1"]')).toHaveTextContent(
      "1",
    ),
  );
  expect(source.querySelector(".cm-line")!.textContent).toBe(
    "# Bedtime for macOS",
  );
  const wrapped = source.querySelectorAll<HTMLElement>(".cm-line")[2];
  const range = canvasElement.ownerDocument.createRange();
  range.selectNodeContents(wrapped);
  const segments = Array.from(range.getClientRects()).filter(
    (rect) => rect.width > 0,
  );
  expect(segments.length).toBeGreaterThan(1);
  for (const segment of segments)
    expect(segment.left).toBeCloseTo(wrapped.getBoundingClientRect().left, 0);
  const lineLabel = gutter!
    .querySelector('[data-captured-line="3"]')!
    .closest(".cm-gutterElement")!;
  expect(lineLabel.getBoundingClientRect().right).toBeLessThan(
    wrapped.getBoundingClientRect().left,
  );
  expect(lineLabel.getBoundingClientRect().top).toBeCloseTo(
    wrapped.getBoundingClientRect().top,
    0,
  );
  const view = EditorView.findFromDOM(source)!;
  const original = view.state.doc.toString();
  expect(original).toMatch(/^\s+1\t# Bedtime/);
  expect(gutter!.querySelector('[data-captured-line="68"]')).not.toBeNull();
  expect(gutter!.querySelector('[data-captured-line="30"]')).toBeNull();
  await userEvent.click(
    c.getByRole("button", { name: "Show 55 more lines in README.md" }),
  );
  await waitFor(() =>
    expect(gutter!.querySelector('[data-captured-line="30"]')).not.toBeNull(),
  );
  const from = original.indexOf("native SwiftUI/AppKit");
  view.dispatch({
    selection: { anchor: from, head: from + "native SwiftUI/AppKit".length },
  });
  view.focus();
  await userEvent.keyboard("Review wrapped source");
  await expect(await draft(canvasElement, "README.md")).toHaveTextContent(
    "Review wrapped source",
  );
  const pending = c.getByRole("group", { name: "Pending comments" });
  expect(pending.querySelector("blockquote")).toHaveTextContent(
    "native SwiftUI/AppKit",
  );
  expect(view.state.doc.toString()).toBe(original);
}
