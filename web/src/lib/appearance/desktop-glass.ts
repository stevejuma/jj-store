import type { HostEnvironment } from "../host/contracts.js";

/** Serialize native effects; never expose CSS for an obsolete or failed enable. */
export function createGlassController(
  apply: (enabled: boolean) => Promise<unknown>,
  expose: (enabled: boolean) => void,
  failed: (error: unknown) => void = (error) =>
    console.warn("Native glass unavailable", error),
) {
  let wanted = false;
  let applied: boolean | undefined = false;
  let disposed = false;
  let running: Promise<void> | undefined;
  async function drain() {
    while (wanted !== applied) {
      const target = wanted;
      try {
        await apply(target);
        applied = target;
      } catch (error) {
        applied = undefined;
        if (!disposed) expose(false);
        failed(error);
        return;
      }
      if (!disposed) expose(wanted && applied);
    }
  }
  function set(enabled: boolean) {
    wanted = enabled && !disposed;
    if (!wanted) expose(false);
    else if (!running && applied === true) expose(true);
    if (!running && wanted !== applied) {
      running = drain().finally(() => {
        running = undefined;
      });
    }
    return running ?? Promise.resolve();
  }
  return {
    set,
    dispose() {
      disposed = true;
      return set(false);
    },
  };
}

/** Capability absence leaves desktop browser surfaces opaque. */
export function attachNativeGlass(
  root: HTMLElement,
  refresh: () => void,
  host: HostEnvironment,
) {
  if (!host.setGlassBackground) return;
  const background = root.style.getPropertyValue("background");
  const priority = root.style.getPropertyPriority("background");
  const controller = createGlassController(
    host.setGlassBackground,
    (enabled) => {
      root.dataset.storeNativeGlass = String(enabled);
      if (enabled) root.style.setProperty("background", "transparent");
      else if (background)
        root.style.setProperty("background", background, priority);
      else root.style.removeProperty("background");
      refresh();
    },
  );
  root.dataset.storeNativeGlass = "false";
  return controller;
}
