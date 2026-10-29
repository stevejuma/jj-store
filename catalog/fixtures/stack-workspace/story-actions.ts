import { fileRowName as workspaceFileName } from "../file-row-name.js";
import { expect, fireEvent, userEvent, waitFor, within } from "storybook/test";

export function workspaceCanvas(element: HTMLElement) {
  return within(element);
}
export function workspacePortal(element: HTMLElement) {
  return within(element.ownerDocument.body);
}
export async function applyPreview(element: HTMLElement) {
  const portal = workspacePortal(element);
  await userEvent.click(
    await portal.findByRole("button", { name: "Apply command" }),
  );
  await waitFor(() =>
    expect(
      portal.queryByRole("button", { name: "Apply command" }),
    ).not.toBeInTheDocument(),
  );
}
export async function commandMenu(
  element: HTMLElement,
  button: string,
  item: string,
) {
  await userEvent.click(
    await workspaceCanvas(element).findByRole("button", {
      name: button,
    }),
  );
  await userEvent.click(
    await workspacePortal(element).findByRole("menuitem", {
      name: item,
    }),
  );
}
export async function chooseWorkspace(element: HTMLElement, name: string) {
  await userEvent.click(
    workspaceCanvas(element).getByRole("combobox", {
      name: "Active stack workspace",
    }),
  );
  await userEvent.click(
    await workspacePortal(element).findByRole("option", {
      name: new RegExp(name),
    }),
  );
  await applyPreview(element);
}
export async function unassignedFile(element: HTMLElement, path: string) {
  const canvas = workspaceCanvas(element);
  await userEvent.click(
    await canvas.findByRole("button", { name: workspaceFileName(path) }),
  );
}
export async function messagePreview(element: HTMLElement, message: string) {
  const portal = workspacePortal(element);
  const inline = portal.queryByLabelText("Commit title");
  const field = inline ?? (await portal.findByLabelText("Commit message"));
  await userEvent.clear(field);
  await userEvent.type(field, message);
  await userEvent.click(
    portal.getByRole("button", {
      name: inline ? "Create commit" : "Preview command",
    }),
  );
}

export async function dropWorkspaceChange(
  source: HTMLElement,
  target: HTMLElement,
) {
  const dataTransfer = new DataTransfer();
  await fireEvent(
    source,
    new DragEvent("dragstart", { bubbles: true, dataTransfer }),
  );
  await fireEvent(
    target,
    new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer }),
  );
  return dataTransfer;
}

export async function assertCompactColumnToggle(element: HTMLElement) {
  const canvas = workspaceCanvas(element);
  await canvas.findByRole("region", { name: "Stack feature/api" });
  const toggle = canvas.getByRole("button", { name: "Collapse Unstaged" });
  const icon = toggle.querySelector(
    '[data-ui-component="panel-toggle"][data-ui-part="icon"]',
  );
  expect(icon).not.toBeNull();
  expect(toggle.getBoundingClientRect().width).toBe(28);
  expect(toggle.getBoundingClientRect().height).toBe(24);
  expect(icon!.getBoundingClientRect().width).toBe(18);
  expect(icon!.getBoundingClientRect().height).toBe(12);
  await userEvent.click(toggle);
  const expand = canvas.getByRole("button", {
    name: "Expand Unstaged column",
  });
  expect(
    expand.querySelector('[data-ui-component="panel-toggle"]'),
  ).not.toBeNull();
  await userEvent.click(expand);
  await expect(
    canvas.getByRole("button", { name: "Collapse Unstaged" }),
  ).toHaveAttribute("aria-expanded", "true");
}

export { fileRowName as workspaceFileName } from "../file-row-name.js";
