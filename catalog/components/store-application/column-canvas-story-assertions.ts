import { expect } from "storybook/test";

export async function expectColumnCanvasFillsBody(canvasElement: HTMLElement) {
  const root = canvasElement.querySelector<HTMLElement>(
    '[data-ui-component="column-canvas"][data-ui-part="root"]',
  )!;
  const body = root.closest<HTMLElement>(
    '[data-ui-component="app-shell"][data-ui-part="body"]',
  )!;
  await expect(root.getBoundingClientRect().height).toBeCloseTo(
    body.getBoundingClientRect().height,
    0,
  );
  await expect(body).toHaveAttribute("data-layout", "regions");
  await expect(getComputedStyle(body).overflow).toBe("hidden");
  await expect(
    body.querySelector(".ui-minimal-app-shell__body-scroll-area"),
  ).toBeNull();
  await expect(
    root.querySelector('[data-ui-part="column-body"]'),
  ).not.toBeNull();
}
