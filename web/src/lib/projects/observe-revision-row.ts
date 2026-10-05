/** Measure row and inline-file heights without remounting graph content. */
export function observeRevisionRow(
  node: HTMLDivElement,
  publish: (height: number, itemHeight: number) => void,
) {
  let frame: number | null = null;
  const observer = new ResizeObserver(() => {
    if (frame !== null) return;
    frame = requestAnimationFrame(() => {
      frame = null;
      const height = node.getBoundingClientRect().height;
      const itemHeight =
        node.parentElement?.getBoundingClientRect().height ?? height;
      publish(height, itemHeight);
    });
  });
  observer.observe(node);
  if (node.parentElement) observer.observe(node.parentElement);
  return {
    destroy() {
      observer.disconnect();
      if (frame !== null) cancelAnimationFrame(frame);
    },
  };
}
