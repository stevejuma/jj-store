import { expect, userEvent, waitFor, within } from "storybook/test";
import {
  reader,
  draft,
  addComment,
  expectAdjacentChevron,
  inlineComments,
} from "./acceptance-helpers.js";
export const patchFilesCollapse = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await reader(canvasElement, "Captured patch example.ts");
  await expect(
    c.queryByRole("textbox", { name: "Captured patch example.test.ts" }),
  ).toBeNull();
  await userEvent.click(
    c.getByRole("button", { name: "Show captured file example.test.ts" }),
  );
  await expect(
    await reader(canvasElement, "Captured patch example.test.ts"),
  ).toHaveTextContent("expect(answer)");
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Captured patch example.ts" }),
  );
  await userEvent.type(
    await draft(canvasElement, "Captured patch example.ts"),
    "Keep this file draft",
  );
  await userEvent.keyboard("{Escape}");
  await userEvent.click(
    c.getByRole("button", { name: "Hide captured file example.ts" }),
  );
  await waitFor(() =>
    expect(
      c.queryByRole("textbox", { name: "Captured patch example.ts" }),
    ).toBeNull(),
  );
  await expect(
    await reader(canvasElement, "Captured patch example.test.ts"),
  ).toBeVisible();
  await userEvent.click(
    c.getByRole("button", { name: "Show captured file example.ts" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Comment on Captured patch example.ts" }),
  );
  await expect(
    await draft(canvasElement, "Captured patch example.ts"),
  ).toHaveTextContent("Keep this file draft");
  await addComment(canvasElement);
  await userEvent.click(
    c.getByRole("button", { name: "Hide captured file example.ts" }),
  );
  await userEvent.click(
    c.getByRole("button", { name: "Show captured file example.ts" }),
  );
  await expect(
    await inlineComments(canvasElement).findByRole("article", {
      name: "Comment by You",
    }),
  ).toHaveTextContent("Keep this file draft");
  const file = c.getByRole("button", {
    name: "Hide captured file example.ts",
  });
  const next = c.getByRole("button", {
    name: "Hide captured file example.test.ts",
  });
  expectAdjacentChevron(file);
  expectAdjacentChevron(next);
  expect(
    next.getBoundingClientRect().top -
      file.closest(".chat-tool-file")!.getBoundingClientRect().bottom,
  ).toBeGreaterThanOrEqual(8);
};

export const shellAndTurnNavigation = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await reader(canvasElement);
  await expect(
    canvasElement.querySelector('[data-ui-part="body"][data-layout="regions"]'),
  ).not.toBeNull();
  const avatar = canvasElement.querySelector(
    '.chat-message-avatar [data-ui-component="community-member-avatar"]',
  )!;
  const text = canvasElement.querySelector(".chat-message-content .cm-line")!;
  expect(avatar.getBoundingClientRect().width).toBe(24);
  expect(
    text.getBoundingClientRect().left - avatar.getBoundingClientRect().right,
  ).toBeLessThanOrEqual(8);
  expect(
    Math.abs(
      text.getBoundingClientRect().top - avatar.getBoundingClientRect().top,
    ),
  ).toBeLessThanOrEqual(6);
  await userEvent.click(
    c.getByRole("button", { name: "Expand Read a file, ran a command" }),
  );
  const opener = c.getByRole("button", {
    name: "Show details for Read example.ts",
  });
  for (const header of canvasElement.querySelectorAll<HTMLElement>(
    ".chat-tool-trigger,.chat-file-trigger,.chat-context-trigger",
  )) {
    if (header.getBoundingClientRect().width) expectAdjacentChevron(header);
  }
  await userEvent.click(opener);
  const pane = await c.findByRole("complementary", {
    name: "Read tool details",
  });
  await expect(
    within(pane).getByRole("textbox", { name: "Captured example.ts" }),
  ).toHaveTextContent("export const answer");
  await userEvent.click(
    c.getByRole("button", { name: "Close read tool details" }),
  );
  await waitFor(() => expect(opener).toHaveFocus());
  expect(getComputedStyle(opener).backgroundColor).toBe("rgba(0, 0, 0, 0)");
  expect(getComputedStyle(opener).paddingLeft).toBe("0px");
  await userEvent.click(await reader(canvasElement));
  const raw = c.getByRole("button", {
    name: "Show raw output for Read example.ts",
  });
  expect(getComputedStyle(raw.closest(".chat-raw-toggle")!).opacity).toBe("0");
  opener.focus();
  expect(getComputedStyle(raw.closest(".chat-raw-toggle")!).opacity).toBe("1");
  await userEvent.click(c.getByRole("button", { name: "Append response" }));
  const markers = c.getAllByRole("button", { name: /Preview turn/ });
  expect(markers).toHaveLength(2);
  expect(
    markers[1].getBoundingClientRect().top -
      markers[0].getBoundingClientRect().top,
  ).toBe(10);
  const marker = c.getByRole("button", { name: "Preview turn 1" });
  await userEvent.hover(marker);
  const summary = await c.findByRole("group", { name: "Turn summary" });
  await expect(summary).toHaveTextContent("Turn Summary");
  await expect(summary).toHaveTextContent(
    "Use stable identities for inline discussions",
  );
  await userEvent.click(
    within(summary).getByRole("button", { name: "Bookmark turn 1" }),
  );
  await expect(
    within(summary).getByRole("button", { name: "Bookmark turn 1" }),
  ).toHaveAttribute("aria-pressed", "true");
  await userEvent.click(
    within(summary).getByRole("button", { name: "Scroll to turn 1" }),
  );
  await waitFor(() =>
    expect(marker).toHaveAttribute("aria-current", "location"),
  );
  const end = c.getByRole("button", { name: "Jump to end" });
  await userEvent.hover(end);
  await expect(c.getByRole("tooltip")).toHaveTextContent("Jump to End");
  await userEvent.click(end);
  await waitFor(() => expect(end).toHaveAttribute("aria-current", "location"));
  await expect(c.getByRole("textbox", { name: "Next message" })).toHaveFocus();
};
