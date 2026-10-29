import { expect, test } from "vitest";
import { applicationController } from "./application-controller.js";
import type { StoreController } from "../../web/src/lib/controller/contracts.js";
import type { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
test("late workspace discovery preserves live owner getters", () => {
  class Owner {
    #loading = false;
    get stackWorkspaceLoading() {
      return this.#loading;
    }
    discover() {
      this.#loading = true;
    }
  }
  const owner = new Owner();
  let binding: StackWorkspaceController | undefined;
  const adapted = applicationController(
    owner as unknown as StoreController,
    () => binding,
  );
  expect(adapted.stackWorkspace).toBeUndefined();
  expect(adapted.stackWorkspaceLoading).toBe(false);
  owner.discover();
  binding = {} as StackWorkspaceController;
  expect(adapted.stackWorkspaceLoading).toBe(true);
  expect(adapted.stackWorkspace).toBe(binding);
});
