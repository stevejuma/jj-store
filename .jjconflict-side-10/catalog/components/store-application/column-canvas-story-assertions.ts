import { expect } from "storybook/test";

export async function expectColumnCanvasFillsBody(canvasElement: HTMLElement) {
  const root = canvasElement.querySelector<HTMLElement>(
    '[data-ui-component="column-canvas"][data-ui-part="root"]',
  )!;
  const body = root.closest<HTMLElement>(".store-application__body-regions")!;
  await expect(root.getBoundingClientRect().height).toBeCloseTo(
    body.getBoundingClientRect().height,
    0,
  );
  await expect(getComputedStyle(body).overflow).toBe("hidden");
  for (const header of root.querySelectorAll<HTMLElement>(
    '[data-ui-part="column-header"]',
  )) {
    await expect(
      Number.parseFloat(getComputedStyle(header).paddingInlineStart),
    ).toBeLessThanOrEqual(8);
  }
  await expect(
    body
      .closest('[data-ui-part="body"]')
      ?.querySelector(".ui-minimal-app-shell__body-scroll-area"),
  ).toBeNull();
  await expect(
    root.querySelector('[data-ui-part="column-body"]'),
  ).not.toBeNull();
}
