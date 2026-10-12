import type { StoreController } from "../controller/contracts.js";
import { createProductionStoreController } from "../controller/production-store-controller.svelte.js";
import type { StoreRouteBrowser } from "../controller/store-route-browser.js";
import { connectionBrowser } from "./browser.js";
import { connectionStorage } from "./preferences.js";
import { readConnectionHash } from "./transport.js";
export interface ConnectionWorkspace {
  id: string;
  controller: StoreController;
  layoutStorage?: Storage;
}
/** A controller and its effects outlive ordinary server selection. */
export class ConnectionWorkspaces {
  values = $state<ConnectionWorkspace[]>([]);
  private owners = new Map<string, () => void>();
  private routes = new Map<string, ReturnType<typeof connectionBrowser>>();
  constructor(
    private options: {
      base?: StoreRouteBrowser;
      storage?: Storage;
      fetcher?: typeof fetch;
      selected(): string;
      route(id: string): string;
      remember(id: string, hash: string): void;
      expired(id: string): void;
    },
  ) {}
  get(id: string, fetcher?: typeof fetch) {
    const existing = this.values.find((w) => w.id === id);
    if (existing) return existing.controller;
    const { base, storage, selected, remember, expired } = this.options;
    let current: { id: string; hash: string };
    try {
      current = readConnectionHash(base?.hash() ?? "");
    } catch {
      current = { id: "invalid", hash: "" };
    }
    const initial =
      selected() === id && current.id === id
        ? current.hash
        : this.options.route(id);
    const route = base
      ? connectionBrowser(
          base,
          id,
          () => selected() === id,
          initial,
          (hash) => remember(id, hash),
        )
      : undefined;
    if (route) this.routes.set(id, route);
    let controller!: StoreController;
    this.owners.set(
      id,
      $effect.root(() => {
        controller = createProductionStoreController({
          browser: route,
          fetcher: fetcher ?? this.options.fetcher,
          active: () => selected() === id,
          authorizationLost: () => {
            if (id !== "local") queueMicrotask(() => expired(id));
          },
        });
      }),
    );
    this.values = [
      ...this.values,
      { id, controller, layoutStorage: connectionStorage(storage, id) },
    ];
    return controller;
  }
  activate(id: string) {
    this.routes.get(id)?.activate();
  }
  selected() {
    return this.values.find(
      (workspace) => workspace.id === this.options.selected(),
    )?.controller;
  }
  remove(id: string) {
    const owner = this.values.find((w) => w.id === id);
    owner?.controller.actions.destroy();
    this.owners.get(id)?.();
    this.owners.delete(id);
    this.routes.delete(id);
    this.values = this.values.filter((w) => w !== owner);
  }
  destroy() {
    for (const value of [...this.values]) this.remove(value.id);
  }
}
