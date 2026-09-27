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

export async function expectRepositoryBreadcrumbLayout(
  breadcrumb: HTMLElement,
) {
  const root = breadcrumb.closest<HTMLElement>('[data-ui-part="breadcrumb"]')!;
  const items = breadcrumb.querySelectorAll<HTMLElement>(
    '[data-ui-part="breadcrumb-item"]',
  );
  const separator = breadcrumb.querySelector<HTMLElement>(
    '[data-ui-part="breadcrumb-separator"]',
  )!;
  const bounds = breadcrumb.getBoundingClientRect();
  const first = items[0]!.getBoundingClientRect();
  const last = items[items.length - 1]!.getBoundingClientRect();
  await expect(getComputedStyle(breadcrumb).display).toBe("flex");
  await expect(getComputedStyle(breadcrumb).flexWrap).toBe("nowrap");
  await expect(bounds.width).toBeCloseTo(root.getBoundingClientRect().width, 0);
  await expect(last.right).toBeLessThanOrEqual(bounds.right + 1);
  await expect(last.width).toBeGreaterThan(0);
  await expect(first.top).toBeCloseTo(last.top, 0);
  await expect(
    Math.abs((first.left + last.right) / 2 - (bounds.left + bounds.right) / 2),
  ).toBeLessThan(2);
  await expect(getComputedStyle(separator).flexShrink).toBe("0");
}
