// Only the bundled documentation may participate in the host's dismissal protocol.
if (window.parent !== window) {
  window.parent.postMessage(
    { type: "jj-store-help-ready" },
    window.location.origin,
  );
  window.addEventListener(
    "keydown",
    (event) => {
      if (event.key !== "Escape" || event.defaultPrevented) return;
      const search = document.querySelector("#__search");
      if (search?.checked) {
        search.checked = false;
        event.preventDefault();
        event.stopImmediatePropagation();
        document.querySelector("[data-md-component=search-query]")?.blur();
        return;
      }
      event.preventDefault();
      window.parent.postMessage(
        { type: "jj-store-help-close" },
        window.location.origin,
      );
    },
    { capture: true },
  );
}

// Complete the dialog's keyboard loop across the iframe document boundary.
if (window.parent !== window) {
  window.addEventListener("keydown", (event) => {
    if (event.key !== "Tab" || event.defaultPrevented) return;
    const items = [
      ...document.querySelectorAll(
        "a[href], button, input, select, textarea, [tabindex]",
      ),
    ].filter(
      (element) =>
        element.tabIndex >= 0 &&
        !element.hasAttribute("disabled") &&
        element.getClientRects().length > 0 &&
        getComputedStyle(element).visibility !== "hidden",
    );
    if (
      (!event.shiftKey && document.activeElement === items.at(-1)) ||
      (event.shiftKey && document.activeElement === items[0])
    ) {
      event.preventDefault();
      window.parent.postMessage(
        { type: "jj-store-help-focus-close" },
        window.location.origin,
      );
    }
  });
}
