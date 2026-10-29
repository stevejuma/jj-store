import { storyControl } from "../story-controls.js";
import { expect, userEvent, within } from "storybook/test";
import { reader } from "./acceptance-helpers.js";
export async function messageActions({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement),
    source = await reader(canvasElement);
  source.focus();
  const actions = c.getByRole("group", {
    name: "Actions for Assistant explanation",
  });
  expect(getComputedStyle(actions).borderTopWidth).toBe("1px");
  expect(getComputedStyle(actions).backgroundColor).not.toBe(
    "rgba(0, 0, 0, 0)",
  );
  await userEvent.click(
    c.getByRole("button", { name: "Add reaction to Assistant explanation" }),
  );
  await userEvent.click(
    await within(document.body).findByRole("button", { name: "React 👍" }),
  );
  const reaction = await c.findByRole("button", {
    name: "1 reaction with 👍: 1",
  });
  await expect(reaction).toHaveAttribute("aria-pressed", "true");
  await storyControl(document.body, "Hide body");
  await storyControl(document.body, "Show body");
  await expect(
    c.getByRole("button", { name: "1 reaction with 👍: 1" }),
  ).toHaveAttribute("aria-pressed", "true");
  await userEvent.click(
    c.getByRole("button", { name: "1 reaction with 👍: 1" }),
  );
  await expect(
    c.queryByRole("button", { name: "1 reaction with 👍: 1" }),
  ).toBeNull();
  await userEvent.click(
    c.getByRole("button", { name: "Agent response · Synthetic review" }),
  );
  const tag = await c.findByRole("button", { name: /<subagent-completion/ });
  await userEvent.click(tag);
  await expect(
    c.getByRole("group", { name: "Returned synthetic response" }),
  ).toHaveTextContent("No regressions found.");
  await expect(
    c.queryByRole("form", { name: "Inline comment composer" }),
  ).toBeNull();
}
