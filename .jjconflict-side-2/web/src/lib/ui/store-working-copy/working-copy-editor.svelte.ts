import { onDestroy } from "svelte";
import type { StoreController } from "../../controller/contracts.js";
import type { WorkingFile } from "../../controller/working-copy-types.js";
export function createWorkingFileEdit(
  controller: () => StoreController,
  path: () => string | undefined,
  opened: () => void,
) {
  let file = $state<WorkingFile | null>(null);
  let pending = $state(false);
  let error = $state("");
  let ticket = 0;
  onDestroy(() => {
    ticket++;
  });
  function clear() {
    ticket++;
    file = null;
    pending = false;
    error = "";
  }
  let repository = "";
  $effect(() => {
    const current = controller().state.repository.id;
    if (current !== repository) {
      repository = current;
      clear();
    }
  });
  function available() {
    const caps = controller().state.settings.capabilities;
    return Boolean(
      caps && "working_copy_edit" in caps && caps.working_copy_edit,
    );
  }
  async function open() {
    const currentPath = path(),
      currentController = controller(),
      repositoryId = currentController.state.repository.id;
    if (
      !currentPath ||
      pending ||
      !available() ||
      !currentController.actions.openWorkingFile
    )
      return;
    const request = ++ticket;
    pending = true;
    error = "";
    try {
      const current =
        await currentController.actions.openWorkingFile(currentPath);
      if (
        request !== ticket ||
        path() !== currentPath ||
        controller().state.repository.id !== repositoryId
      )
        return;
      file = current;
      opened();
    } catch (cause) {
      if (request === ticket)
        error =
          cause instanceof Error ? cause.message : "Working file unavailable.";
    } finally {
      if (request === ticket) pending = false;
    }
  }
  return {
    get file() {
      return file;
    },
    get pending() {
      return pending;
    },
    get error() {
      return error;
    },
    available,
    clear,
    open,
  };
}
