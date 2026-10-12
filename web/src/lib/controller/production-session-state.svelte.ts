import { requireProductionSource } from "./production-store-lifecycle.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";

export function createProductionSessionState() {
  let source: StoreDataSource | null = null;
  let token = $state<string | null>(null);
  return {
    requireSource() {
      return requireProductionSource(source);
    },
    get source() {
      return source;
    },
    get token() {
      return token;
    },
    setToken(value: string | null) {
      token = value;
    },
    setSource(value: StoreDataSource | null) {
      source = value;
    },
    closeSource() {
      source?.close();
      source = null;
    },
    clearToken() {
      token = null;
    },
  };
}
