export const navigationStorageKey = "jj-store/layout/v1/navigation";
export interface NavigationLayout {
  sections: Record<string, boolean>;
  items: Record<string, boolean>;
}
function flags(value: unknown): value is Record<string, boolean> {
  return (
    typeof value === "object" &&
    value !== null &&
    !Array.isArray(value) &&
    Object.values(value).every((flag) => typeof flag === "boolean")
  );
}
export function loadNavigationLayout(
  storage: Storage | undefined,
): NavigationLayout {
  try {
    const value = JSON.parse(storage?.getItem(navigationStorageKey) ?? "null");
    if (value?.version === 1 && flags(value.sections) && flags(value.items))
      return { sections: value.sections, items: value.items };
  } catch {
    /* Storage availability does not gate navigation. */
  }
  return { sections: {}, items: {} };
}
export function saveNavigationLayout(
  storage: Storage | undefined,
  layout: NavigationLayout,
): void {
  try {
    storage?.setItem(
      navigationStorageKey,
      JSON.stringify({
        version: 1,
        sections: layout.sections,
        items: layout.items,
      }),
    );
  } catch {
    /* Navigation remains usable when storage is unavailable. */
  }
}
