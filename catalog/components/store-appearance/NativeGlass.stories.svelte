<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within } from "storybook/test";
  import Fixture from "../../fixtures/AppearanceShellFixture.svelte";
  import "../../../web/src/lib/appearance/native-glass.css";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance/Native glass composition",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
    },
  });
  async function glass({ canvasElement }: { canvasElement: HTMLElement }) {
    const root = canvasElement.ownerDocument.documentElement;
    const shell = canvasElement.querySelector<HTMLElement>(
      ".community-app-shell",
    )!;
    const header = within(canvasElement).getByRole("navigation", {
      name: "Application navigation",
    });
    const native = root.getAttribute("data-store-native-glass");
    const opacity = root.style.getPropertyValue("--store-glass-opacity");
    try {
      // Simulate confirmed native readiness; actual material is qualified in WKWebView.
      root.dataset.storeNativeGlass = "true";
      const style = getComputedStyle(shell);
      expect(style.backgroundImage).toContain("linear-gradient");
      expect(style.backgroundImage).toContain("0.65");
      expect(getComputedStyle(header).backgroundColor).toBe("rgba(0, 0, 0, 0)");
      expect(getComputedStyle(header).backdropFilter).toBe("none");
      const main = shell.querySelector(
        '[data-ui-component="app-shell"][data-ui-part="main"]',
      )!;
      expect(getComputedStyle(main).backgroundColor).not.toBe(
        "rgba(0, 0, 0, 0)",
      );
      root.style.setProperty("--store-glass-opacity", "0.3");
      expect(getComputedStyle(shell).backgroundImage).toContain("0.3");
      root.dataset.storeNativeGlass = "false";
      expect(getComputedStyle(header).backdropFilter).toBe("blur(24px)");
      expect(getComputedStyle(shell).backgroundImage).not.toContain(
        "color(srgb",
      );
    } finally {
      if (native === null) root.removeAttribute("data-store-native-glass");
      else root.setAttribute("data-store-native-glass", native);
      root.style.setProperty("--store-glass-opacity", opacity);
    }
  }
</script>

<Story name="Light" args={{ theme: "buzz", glass: true }} play={glass} />
<Story name="Dark" args={{ theme: "buzz-dark", glass: true }} play={glass} />
