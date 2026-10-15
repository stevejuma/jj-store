import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { storeErrorMessage } from "./store-error.js";
import {
  emptySearchSettings,
  searchDraft,
  searchConfiguration,
  type SearchDraft,
  type SearchAction,
} from "./search-settings.js";
export const searchOperating = (operation: string | undefined) =>
  ["applying", "rebuilding", "retrying"].includes(operation ?? "");
export function createProductionSearchSettings(options: {
  data: ProductionStoreState;
  source: () => StoreDataSource;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
  freshness?: import("./application-freshness.js").ApplicationFreshness;
  signal?(): AbortSignal;
}) {
  const { data } = options;
  let generation = 0;
  let draftRevision: string | undefined;
  let saveError = "";
  function clear() {
    generation++;
    draftRevision = undefined;
    saveError = "";
    data.searchSettings = emptySearchSettings();
  }
  function setSearchDraft(patch: Partial<SearchDraft>) {
    const state = data.searchSettings;
    if (
      state.draft &&
      !state.changing &&
      !searchOperating(state.value?.operation)
    ) {
      if (!state.dirty) draftRevision = state.value?.revision;
      state.draft = { ...state.draft, ...patch };
      state.dirty = true;
      state.validation = "";
    }
  }
  function cancelSearchDraft() {
    const state = data.searchSettings;
    if (state.value && !state.changing) {
      state.draft = searchDraft(state.value.desired);
      draftRevision = state.value.revision;
      state.dirty = false;
      state.validation = "";
      if (state.error === saveError) state.error = "";
      saveError = "";
    }
  }
  async function request(action?: SearchAction | "save") {
    const state = data.searchSettings;
    if (state.changing) return;
    if (!data.capabilities?.local_search_management) {
      state.state = "unavailable";
      return;
    }
    if (
      action &&
      (!state.value ||
        state.state !== "ready" ||
        searchOperating(state.value.operation))
    )
      return;
    if (action && action !== "save" && state.dirty) return;
    if (action === "save") state.validation = "";
    let configuration;
    try {
      if (action === "save" && state.draft)
        configuration = searchConfiguration(state.draft);
    } catch (cause) {
      state.validation =
        cause instanceof Error ? cause.message : "Check the Search settings.";
      return;
    }
    const source = options.source(),
      ticket = options.version(),
      token = ++generation;
    const current = () =>
      options.current(ticket, source) && token === generation;
    state.changing = true;
    state.error = saveError;
    state.state = state.value ? "refreshing" : "loading";
    try {
      if (!source.searchSettings)
        throw new Error("Search settings unavailable");
      let observed = options.freshness?.journal.revision ?? 0;
      let result =
        action === "save"
          ? await source.searchSettings.save(
              draftRevision ?? state.value!.revision,
              configuration!,
            )
          : action
            ? await source.searchSettings.control(state.value!.revision, action)
            : await source.searchSettings.status();
      if (!current()) return;
      state.value = result;
      if (action === "save" || !state.dirty) {
        state.draft = searchDraft(result.desired);
        draftRevision = result.revision;
        state.dirty = false;
      }
      // Explicit action completion is allowed to refresh management state, never the selected repository snapshot.
      while (action && action !== "save" && searchOperating(result.operation)) {
        if (options.freshness) {
          const freshness = options.freshness;
          await freshness.journal.wait({
            after: observed,
            matches: (change) =>
              change.domain === "search" &&
              (change.repository === null ||
                change.repository === source.repository),
            healthy: () => freshness.healthy,
            listen: (listener) => freshness.listen(listener),
            signal: options.signal?.() ?? new AbortController().signal,
            delay: 5000,
          });
        } else await new Promise((resolve) => setTimeout(resolve, 500));
        if (!current()) return;
        observed = options.freshness?.journal.revision ?? observed;
        result = await source.searchSettings.status();
        if (!current()) return;
        state.value = result;
      }
      if (data.capabilities)
        data.capabilities.search = result.effective.enabled;
      if (action === "save") saveError = "";
      // Observing current settings does not resolve a rejected draft save.
      state.error = saveError;
      state.state = "ready";
    } catch (cause) {
      if (!current()) return;
      state.state = "failed";
      if (action === "save") {
        saveError = `${storeErrorMessage(cause)}. Your draft is retained. Refresh settings before retrying.`;
        state.error = saveError;
      } else {
        state.error = `${storeErrorMessage(cause)}. Settings will retry automatically.`;
      }
    } finally {
      if (current()) state.changing = false;
    }
  }
  return {
    clear,
    setSearchDraft,
    cancelSearchDraft,
    refreshSearchSettings: () => request(),
    saveSearchSettings: () => request("save"),
    controlSearch: (action: SearchAction) => request(action),
  };
}
