import { expect, waitFor, within } from "storybook/test";
export async function assertCanvasInsets(canvasElement: HTMLElement) {
  const body = within(canvasElement).getByRole("region", {
    name: "Local stack workspace",
  });
  const area = body.querySelector<HTMLElement>(
    ".stack-workspace__canvas-scroll",
  )!;
  const viewport = area.querySelector<HTMLElement>(
    '[data-ui-part="scroll-area-viewport"]',
  )!;
  const columns = area.querySelector<HTMLElement>(".stack-workspace__canvas")!;
  const content = columns.parentElement!;
  expect(content).toHaveAttribute("data-scroll-area-content");
  expect(Number.parseFloat(getComputedStyle(columns).paddingInlineStart)).toBe(
    0,
  );
  expect(Number.parseFloat(getComputedStyle(area).paddingInlineStart)).toBe(12);
  expect(Number.parseFloat(getComputedStyle(content).paddingInlineStart)).toBe(
    0,
  );
  expect(
    Math.abs(
      area.getBoundingClientRect().width - body.getBoundingClientRect().width,
    ),
  ).toBeLessThanOrEqual(1);
  expect(
    Math.abs(
      area.getBoundingClientRect().left - body.getBoundingClientRect().left,
    ),
  ).toBeLessThanOrEqual(1);
  viewport.scrollTo({ left: 0, behavior: "auto" });
  const firstColumn = columns.querySelector<HTMLElement>(
    '[data-ui-part="column"]',
  )!;
  await waitFor(() =>
    expect(
      Math.abs(
        firstColumn.getBoundingClientRect().left -
          area.getBoundingClientRect().left -
          12,
      ),
    ).toBeLessThanOrEqual(1),
  );
}
