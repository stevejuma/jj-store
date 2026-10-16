import { expect, userEvent, within, fireEvent } from "storybook/test";
import { WORKSPACE_DRAG_TYPE } from "../../../web/src/lib/controller/stack-workspace/drag.js";
import {
  workspaceCanvas,
  workspacePortal,
  unassignedFile,
  applyPreview,
} from "./story-actions.js";
export const verifyDraggableHunks = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const canvas = workspaceCanvas(canvasElement);
  await unassignedFile(canvasElement, "src/settings.ts");
  const first = canvas.getByRole("group", {
    name: "Draggable hunk add keyboard shortcut",
  });
  const second = canvas.getByRole("group", {
    name: "Draggable hunk remember preference",
  });
  const firstHeader = first.querySelector<HTMLElement>(
    '[data-ui-part="card-header"]',
  )!;
  const secondHeader = second.querySelector<HTMLElement>(
    '[data-ui-part="card-header"]',
  )!;
  expect(within(firstHeader).getByText("@@ -1,1 +1,1 @@")).toBeVisible();
  expect(within(secondHeader).getByText("@@ -11,1 +11,1 @@")).toBeVisible();
  expect(getComputedStyle(firstHeader.querySelector("code")!).fontSize).toBe(
    "11px",
  );
  expect(within(first).getByText("new_add_keyboard_shortcut")).toBeVisible();
  expect(within(first).queryByText("new_remember_preference")).toBeNull();
  expect(within(second).getByText("new_remember_preference")).toBeVisible();
  expect(first).toHaveAttribute("draggable", "true");
  await userEvent.click(canvas.getByRole("radio", { name: "Show split diff" }));
  expect(first.querySelector('[data-view-mode="split"]')).not.toBeNull();
  expect(second.querySelector('[data-view-mode="split"]')).not.toBeNull();
  const transfer = new DataTransfer();
  await fireEvent(
    second,
    new DragEvent("dragstart", { bubbles: true, dataTransfer: transfer }),
  );
  expect(
    JSON.parse(transfer.getData(WORKSPACE_DRAG_TYPE)).selection.hunks,
  ).toEqual(["src/settings.ts:remember preference"]);
  await fireEvent(
    canvas.getByRole("button", { name: "Add workspace toolbar" }),
    new DragEvent("drop", {
      bubbles: true,
      cancelable: true,
      dataTransfer: transfer,
    }),
  );
  await userEvent.click(
    workspacePortal(canvasElement).getByRole("button", {
      name: "Preview command",
    }),
  );
  await applyPreview(canvasElement);
  await unassignedFile(canvasElement, "src/settings.ts");
  expect(
    canvas.queryByRole("group", {
      name: "Draggable hunk remember preference",
    }),
  ).toBeNull();
  expect(
    canvas.getByRole("group", {
      name: "Draggable hunk add keyboard shortcut",
    }),
  ).toBeVisible();
};
