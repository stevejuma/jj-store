import type { Preview } from "@storybook/svelte-vite";
import { withThemeByDataAttribute } from "@storybook/addon-themes";
import { expect, waitFor } from "storybook/test";
import { syncCatalogStoryLayout } from "@lapismd/design-core/storybook/catalog-layout";
import { preparePortableApplicationCanvas } from "./portable-canvas.js";
import "../web/src/utilities.css";
import "@lapismd/design-core/storybook.css";
import "@lapismd/design-core/themes/lapis.css";
import "./preview.css";
const preview: Preview = {
  beforeEach: async () => {
    // Portable stories dispose the previous canvas immediately before this hook.
    // Let an overlay outro release its modal lease before mounting the next one.
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  },
  tags: ["autodocs", "test"],
  globalTypes: {
    theme: {
      toolbar: {
        icon: "paintbrush",
        items: ["default", "lapis"],
        dynamicTitle: true,
      },
    },
    colorMode: {
      toolbar: {
        icon: "circlehollow",
        items: ["system", "light", "dark"],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: { theme: "default", colorMode: "system" },
  parameters: {
    layout: "padded",
    viewport: {
      options: {
        mobile390: {
          name: "Mobile 390",
          styles: { width: "390px", height: "844px" },
          type: "mobile",
        },
      },
    },
    docs: { toc: true },
    a11y: { test: "error" },
    backgrounds: { disable: true },
    themes: { disable: true },
  },
  decorators: [
    withThemeByDataAttribute({
      themes: { default: "default", lapis: "lapis" },
      defaultTheme: "default",
      attributeName: "data-ui-theme",
    }),
    (story, context) => {
      if (typeof document !== "undefined") {
        if (context.parameters.jjStoreStory?.kind === "application") {
          context.parameters.layout = "fullscreen";
          document.documentElement.dataset.storyColorMode =
            context.globals.colorMode;
          window.dispatchEvent(new Event("jj-store:story-appearance"));
        }
        syncCatalogStoryLayout(document, context);
        if (
          context.viewMode === "story" &&
          (context.parameters.jjStoreStory?.kind === "application" ||
            context.parameters.jjStoreShell === true)
        )
          document.body.setAttribute("data-ui-catalog-full-viewport", "");
        else document.body.removeAttribute("data-ui-catalog-full-viewport");
        preparePortableApplicationCanvas(
          document,
          context.canvasElement,
          document.body.hasAttribute("data-ui-catalog-full-viewport"),
        );
        document.documentElement.classList.toggle(
          "dark",
          context.globals.colorMode === "dark",
        );
      }
      return story();
    },
  ],
};
export default preview;
