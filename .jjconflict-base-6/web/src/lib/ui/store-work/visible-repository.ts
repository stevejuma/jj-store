/** Viewport observation owns a cancelable directory-read lease, not a snapshot. */
export function visibleRepository(
  node: HTMLElement,
  options: { id: string; watch?: (id: string) => () => void },
) {
  let release: (() => void) | undefined;
  let visible = false;
  const clear = () => {
    release?.();
    release = undefined;
  };
  const observer = new IntersectionObserver(
    (entries) => {
      visible = entries.some(
        (entry) => entry.target === node && entry.isIntersecting,
      );
      if (visible) release ??= options.watch?.(options.id);
      else clear();
    },
    { rootMargin: "200px 0px" },
  );
  observer.observe(node);
  return {
    update(next: typeof options) {
      if (options.id === next.id && options.watch === next.watch) return;
      clear();
      options = next;
      if (visible) release = options.watch?.(options.id);
    },
    destroy() {
      observer.disconnect();
      clear();
    },
  };
}
