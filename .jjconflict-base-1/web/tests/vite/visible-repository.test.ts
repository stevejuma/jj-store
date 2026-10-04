import { afterEach, expect, test, vi } from "vitest";
import { visibleRepository } from "../../src/lib/ui/store-work/visible-repository.js";

afterEach(() => vi.unstubAllGlobals());

test("viewport entry, identity changes, exit and unmount own exactly one directory lease", () => {
  let notify!: (entries: Partial<IntersectionObserverEntry>[]) => void;
  const observe = vi.fn(),
    disconnect = vi.fn();
  vi.stubGlobal(
    "IntersectionObserver",
    class {
      constructor(callback: typeof notify, options: IntersectionObserverInit) {
        notify = callback;
        expect(options.rootMargin).toBe("200px 0px");
      }
      observe = observe;
      disconnect = disconnect;
    },
  );
  const node = {} as HTMLElement,
    release = vi.fn();
  const watch = vi.fn(() => release);
  const action = visibleRepository(node, { id: "one", watch });
  expect(observe).toHaveBeenCalledWith(node);
  expect(watch).not.toHaveBeenCalled();
  notify([{ target: node, isIntersecting: true }]);
  notify([{ target: node, isIntersecting: true }]);
  action.update({ id: "one", watch });
  expect(watch).toHaveBeenCalledTimes(1);
  action.update({ id: "two", watch });
  expect(release).toHaveBeenCalledTimes(1);
  expect(watch).toHaveBeenLastCalledWith("two");
  notify([{ target: node, isIntersecting: false }]);
  expect(release).toHaveBeenCalledTimes(2);
  notify([{ target: node, isIntersecting: true }]);
  action.destroy();
  expect(release).toHaveBeenCalledTimes(3);
  expect(disconnect).toHaveBeenCalledTimes(1);
});

test("rows requiring separate access do not start directory reads", () => {
  let notify!: (entries: Partial<IntersectionObserverEntry>[]) => void;
  vi.stubGlobal(
    "IntersectionObserver",
    class {
      constructor(callback: typeof notify) {
        notify = callback;
      }
      observe() {}
      disconnect() {}
    },
  );
  const node = {} as HTMLElement;
  const action = visibleRepository(node, { id: "hidden" });
  notify([{ target: node, isIntersecting: true }]);
  const watch = vi.fn(() => vi.fn());
  action.update({ id: "hidden", watch });
  expect(watch).toHaveBeenCalledTimes(1);
  action.update({ id: "hidden" });
  action.destroy();
});
