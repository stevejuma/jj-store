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

// Keep the document, header, search worker, palette and fonts mounted while
// navigating the portable manual. Material's sitemap navigation assumes a
// deployment URL; bundled Help has no fixed origin or port.
(() => {
  let controller;
  let failed;
  let renderedPath = location.pathname;
  const notify = (type, message) => {
    if (window.parent !== window)
      window.parent.postMessage({ type, message }, location.origin);
  };
  const internal = (url) =>
    url.origin === location.origin && url.pathname.startsWith("/help/");
  const remember = () =>
    history.replaceState({ ...history.state, helpScrollY: scrollY }, "");
  const scroll = (url, offset = 0) => {
    const target = document.getElementById(
      decodeURIComponent(url.hash.slice(1)),
    );
    if (target) target.scrollIntoView();
    else window.scrollTo(0, offset);
  };
  const navigate = async (url, push = true, offset = 0) => {
    controller?.abort();
    const request = new AbortController();
    controller = request;
    failed = { url, push, offset };
    notify("jj-store-help-loading");
    document.body.dataset.helpLoading = "true";
    document.querySelector(".help-page-skeleton")?.remove();
    const skeleton = document.createElement("div");
    skeleton.className = "help-page-skeleton";
    skeleton.setAttribute("role", "status");
    skeleton.setAttribute("aria-label", "Loading documentation");
    skeleton.setAttribute("aria-busy", "true");
    document.querySelector(".md-content")?.appendChild(skeleton);
    try {
      const response = await fetch(url, { signal: request.signal });
      if (!response.ok)
        throw new Error(
          (await response.text()).trim().slice(0, 500) ||
            `Documentation request failed (${response.status}).`,
        );
      if (!internal(new URL(response.url)))
        throw new Error("Documentation redirected outside bundled Help.");
      const next = new DOMParser().parseFromString(
        await response.text(),
        "text/html",
      );
      if (
        !next.querySelector("#__config") ||
        !next.querySelector('[data-md-component="container"]')
      )
        throw new Error("The documentation response is not a valid Help page.");
      // Resolve relative links against the requested page before adopting nodes
      // into the retained document. Diagram variants have their own URL fields.
      for (const element of next.querySelectorAll(
        "[href], [src], [data-light-src], [data-dark-src]",
      )) {
        for (const attribute of [
          "href",
          "src",
          "data-light-src",
          "data-dark-src",
        ]) {
          const value = element.getAttribute(attribute);
          if (value) element.setAttribute(attribute, new URL(value, url).href);
        }
      }
      if (request.signal.aborted) return;
      if (push) {
        remember();
        history.pushState(null, "", url);
      }
      for (const name of ["container", "header-topic", "skip"]) {
        const selector = `[data-md-component="${name}"]`;
        const current = document.querySelector(selector),
          replacement = next.querySelector(selector);
        if (current && replacement) {
          // Bootstrap scripts were used on the initial document; Material's
          // document lifecycle initializes new code blocks and navigation.
          replacement
            .querySelectorAll("script")
            .forEach((script) => script.remove());
          current.replaceWith(document.importNode(replacement, true));
        }
      }
      document.title = next.title;
      for (const relation of ["prev", "next"]) {
        document.querySelector(`link[rel="${relation}"]`)?.remove();
        const link = next.querySelector(`link[rel="${relation}"]`);
        if (link) document.head.appendChild(document.importNode(link, true));
      }
      renderedPath = url.pathname;
      window.location$?.next(url);
      window.document$?.next(document);
      window.dispatchEvent(new Event("jj-store-help-page"));
      delete document.body.dataset.helpLoading;
      failed = undefined;
      scroll(url, offset);
      const heading = document.querySelector(".md-content h1");
      heading?.setAttribute("tabindex", "-1");
      heading?.focus({ preventScroll: true });
      notify("jj-store-help-ready");
    } catch (error) {
      if (request.signal.aborted) return;
      delete document.body.dataset.helpLoading;
      document.querySelector(".help-page-skeleton")?.remove();
      notify(
        "jj-store-help-error",
        error instanceof Error
          ? error.message.slice(0, 500)
          : "Documentation could not be loaded.",
      );
    }
  };
  document.addEventListener("click", (event) => {
    if (
      event.defaultPrevented ||
      event.button !== 0 ||
      event.ctrlKey ||
      event.metaKey ||
      event.shiftKey ||
      event.altKey
    )
      return;
    const link =
      event.target instanceof Element ? event.target.closest("a[href]") : null;
    if (!link || link.target || link.hasAttribute("download")) return;
    const url = new URL(link.href);
    if (!internal(url)) return;
    event.preventDefault();
    if (url.pathname === renderedPath && url.search === location.search) {
      controller?.abort();
      delete document.body.dataset.helpLoading;
      document.querySelector(".help-page-skeleton")?.remove();
      failed = undefined;
      if (url.href !== location.href) {
        remember();
        history.pushState(null, "", url);
        window.location$?.next(url);
        scroll(url);
      }
      notify("jj-store-help-ready");
      return;
    }
    void navigate(url);
  });
  window.addEventListener("popstate", () => {
    const url = new URL(location.href);
    if (!internal(url)) return;
    if (url.pathname !== renderedPath)
      void navigate(url, false, history.state?.helpScrollY ?? 0);
    else scroll(url, history.state?.helpScrollY ?? 0);
  });
  window.addEventListener("message", (event) => {
    if (
      event.source !== window.parent ||
      event.origin !== location.origin ||
      event.data?.type !== "jj-store-help-retry" ||
      !failed
    )
      return;
    void navigate(failed.url, failed.push, failed.offset);
  });
  window.addEventListener("pagehide", () => controller?.abort());
})();
