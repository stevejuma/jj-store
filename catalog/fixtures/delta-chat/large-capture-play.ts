import { expect, userEvent, waitFor, within } from "storybook/test";
import { reader, draft, addComment } from "./acceptance-helpers.js";
export async function largeCaptures({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) {
  const c = within(canvasElement);
  const code = await reader(canvasElement, "Large captured code");
  const patch = await reader(canvasElement, "Large captured patch");
  for (const source of [code, patch]) {
    await expect(source).toHaveTextContent("line1 = 1");
    await expect(source).toHaveTextContent("line60 = 60");
    await expect(source).not.toHaveTextContent("line30 = 30");
    expect(
      getComputedStyle(source.closest(".chat-text-block")!).backgroundColor,
    ).not.toBe("rgba(0, 0, 0, 0)");
  }
  await userEvent.click(
    c.getByRole("button", {
      name: "Show 40 more lines in Large captured code",
    }),
  );
  await waitFor(() => expect(code).toHaveTextContent("line30 = 30"));
  await expect(patch).not.toHaveTextContent("line30 = 30");
  await userEvent.click(
    c.getByRole("button", { name: "Show less in Large captured code" }),
  );
  await waitFor(() => expect(code).not.toHaveTextContent("line30 = 30"));
  await expect(code).toHaveTextContent("line60 = 60");
  for (const label of ["Large captured patch", "Large captured code"]) {
    await userEvent.click(
      c.getByRole("button", { name: `Show 40 more lines in ${label}` }),
    );
  }
  await expect(patch).toHaveTextContent("line30 = 30");
  await userEvent.click(
    c.getByRole("button", { name: "Show less in Large captured patch" }),
  );
  await waitFor(() => expect(patch).not.toHaveTextContent("line30 = 30"));
  code.focus();
  await userEvent.keyboard("Review middle");
  await expect(
    await draft(canvasElement, "Large captured code"),
  ).toHaveTextContent("Review middle");
  await expect(
    c
      .getByRole("group", { name: "Pending comments" })
      .querySelector("blockquote"),
  ).toHaveTextContent("export const line13 = 13;");
  await addComment(canvasElement);
  await userEvent.click(c.getByRole("button", { name: "Hide body" }));
  await userEvent.click(c.getByRole("button", { name: "Show body" }));
  await expect(
    await reader(canvasElement, "Large captured code"),
  ).toHaveTextContent("line30 = 30");
  await expect(
    c.getByRole("article", { name: "Comment by You" }),
  ).toHaveTextContent("Review middle");
  await expect(
    c.getByRole("button", {
      name: "Show 40 more lines in Large captured patch",
    }),
  ).toBeVisible();
}
