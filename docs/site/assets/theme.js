// Use the same generated palette sources as the app. Embedded appearance is
// read-only: no preference store, parent writes or new message payloads.
(() => {
  const root = document.documentElement;
  const attributes = [
    "data-store-theme",
    "data-store-accent",
    "data-store-font",
    "data-store-density",
    "data-store-glass",
    "data-store-prominent",
  ];
  let host;
  try {
    if (
      window.parent !== window &&
      window.parent.location.origin === location.origin
    ) {
      const candidate = window.parent.document.documentElement;
      if (candidate.hasAttribute("data-store-theme")) host = candidate;
    }
  } catch {
    // Independently hosted docs remain usable outside the application origin.
  }
  const diagrams = (dark) => {
    for (const image of document.querySelectorAll(
      ".architecture-diagram img[data-light-src]",
    )) {
      const source = dark ? image.dataset.darkSrc : image.dataset.lightSrc;
      if (image.getAttribute("src") !== source)
        image.setAttribute("src", source);
    }
  };
  const sync = () => {
    let dark;
    if (host) {
      root.dataset.helpEmbedded = "true";
      for (const attribute of attributes) {
        const value = host.getAttribute(attribute);
        if (value === null) root.removeAttribute(attribute);
        else root.setAttribute(attribute, value);
      }
      root.style.setProperty(
        "--store-glass-opacity",
        host.style.getPropertyValue("--store-glass-opacity"),
      );
      dark = host.classList.contains("dark");
    } else {
      dark = document.body.getAttribute("data-md-color-scheme") === "slate";
      root.dataset.storeTheme = dark ? "original-dark" : "original";
      root.dataset.storeFont = "default";
    }
    root.classList.toggle("dark", dark);
    root.style.colorScheme = dark ? "dark" : "light";
    diagrams(dark);
  };
  const observer = new MutationObserver(sync);
  const observe = () => {
    sync();
    observer.observe(host ?? document.body, {
      attributes: true,
      attributeFilter: host
        ? [...attributes, "class", "style"]
        : ["data-md-color-scheme"],
    });
  };
  observe();
  window.addEventListener("pagehide", () => observer.disconnect());
  window.addEventListener("pageshow", observe);
  window.addEventListener("jj-store-help-page", sync);
})();
