import type { StoreRoute } from "./store-route-types.js";
export function createStoreNavigationGuards(route: () => StoreRoute) {
  const guards = new Map<
    string,
    (current: StoreRoute, next: StoreRoute) => string
  >();
  return {
    setNavigationGuard(
      key: string,
      guard?: (current: StoreRoute, next: StoreRoute) => string,
    ) {
      if (guard) guards.set(key, guard);
      else guards.delete(key);
    },
    reason(next: StoreRoute) {
      return (
        [...guards.values()]
          .map((guard) => guard(route(), next))
          .find(Boolean) ?? ""
      );
    },
  };
}
