import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./http-store-data-source.js";
import { emptyWorkspaceLinks } from "./workspace-link-types.js";

export function createProductionWorkspaceLinks(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
}) {
  const { data } = options;
  let generation = 0;
  const available = () => Boolean(data.capabilities?.local_workspace_links);
  function clear() {
    generation++;
    Object.assign(data, emptyWorkspaceLinks());
  }
  async function refreshWorkspaceLinks() {
    if (data.workspaceLinking || data.workspacePicking) return;
    if (!available()) {
      generation++;
      data.workspaceLinks = null;
      data.workspaceLinksState = "unavailable";
      return;
    }
    const source = options.source(),
      ticket = options.version(),
      request = ++generation;
    data.workspaceLinksState = data.workspaceLinks ? "refreshing" : "loading";
    data.workspaceLinksError = "";
    try {
      if (!source.localWorkspaces)
        throw new Error("Local folder management unavailable");
      const result = await source.localWorkspaces.list();
      if (!options.current(ticket, source) || request !== generation) return;
      data.workspaceLinks = result;
      data.workspaceLinksState = "ready";
    } catch (cause) {
      if (!options.current(ticket, source) || request !== generation) return;
      data.workspaceLinksState = "failed";
      data.workspaceLinksError = storeErrorMessage(cause);
    }
  }
  function setWorkspacePath(path: string) {
    if (data.workspaceLinking || data.workspacePicking) return;
    data.workspacePickerError = "";
    data.workspacePath = path;
    if (!path) data.workspaceLinksError = "";
  }
  async function linkWorkspace() {
    if (
      !available() ||
      !data.workspaceLinks ||
      !data.workspacePath ||
      data.workspaceLinking ||
      data.workspacePicking
    )
      return;
    const source = options.source(),
      ticket = options.version(),
      request = ++generation;
    const path = data.workspacePath;
    const intent = {
      version: 1,
      path,
      expected_revision: data.workspaceLinks.revision,
    };
    data.workspaceLinking = true;
    data.workspaceLinksState = "ready";
    data.workspaceLinksError = "";
    try {
      if (!source.localWorkspaces)
        throw new Error("Local folder management unavailable");
      const result = await source.localWorkspaces.link(intent);
      if (!options.current(ticket, source) || request !== generation) return;
      data.workspaceLinks = result;
      data.workspaceLinksState = "ready";
      data.workspacePath = "";
    } catch (cause) {
      if (!options.current(ticket, source) || request !== generation) return;
      data.workspaceLinksError = `${(storeErrorMessage(cause) || "Folder link could not be confirmed").replace(/\.$/, "")}. Refresh folders to check the saved state before retrying.`;
    } finally {
      if (options.current(ticket, source) && request === generation)
        data.workspaceLinking = false;
    }
  }
  async function pickWorkspaceFolder() {
    if (
      !available() ||
      !data.capabilities?.local_folder_picker ||
      data.workspaceLinking ||
      data.workspacePicking ||
      ["loading", "refreshing"].includes(data.workspaceLinksState)
    )
      return;
    const source = options.source(),
      ticket = options.version(),
      request = ++generation;
    data.workspacePicking = true;
    data.workspacePickerError = "";
    try {
      if (!source.localWorkspaces) throw new Error("Folder picker unavailable");
      const result = await source.localWorkspaces.pick();
      if (!options.current(ticket, source) || request !== generation) return;
      if (result.path !== null) data.workspacePath = result.path;
    } catch (cause) {
      if (!options.current(ticket, source) || request !== generation) return;
      data.workspacePickerError = `${storeErrorMessage(cause)}. Enter the folder path manually or try Browse again.`;
    } finally {
      if (options.current(ticket, source) && request === generation)
        data.workspacePicking = false;
    }
  }
  return {
    clear,
    refreshWorkspaceLinks,
    setWorkspacePath,
    linkWorkspace,
    pickWorkspaceFolder,
  };
}
