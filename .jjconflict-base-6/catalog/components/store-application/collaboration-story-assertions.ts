import { expect } from "storybook/test";

export async function expectSelectionSurfaceCoversRow(selection: HTMLElement) {
  const row = selection.closest<HTMLElement>(".store-collaboration__row");
  if (!row) throw new Error("Collaboration row was not rendered");
  const rowRect = row.getBoundingClientRect();
  const selectionRect = selection.getBoundingClientRect();
  await expect(Math.abs(selectionRect.left - rowRect.left)).toBeLessThanOrEqual(
    1,
  );
  await expect(Math.abs(selectionRect.top - rowRect.top)).toBeLessThanOrEqual(
    1,
  );
  await expect(
    Math.abs(selectionRect.right - rowRect.right),
  ).toBeLessThanOrEqual(1);
  await expect(
    Math.abs(selectionRect.bottom - rowRect.bottom),
  ).toBeLessThanOrEqual(1);
  return row;
}
