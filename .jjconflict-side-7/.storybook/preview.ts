import type { Preview } from "@storybook/svelte-vite";
import { withThemeByDataAttribute } from "@storybook/addon-themes";
import { syncCatalogStoryLayout } from "@lapismd/design-core/storybook/catalog-layout";
import { preparePortableApplicationCanvas } from "./portable-canvas.js";
import "@lapismd/design-core/storybook.css";
import "@lapismd/design-core/themes/lapis.css";
import "./preview.css";
const preview: Preview = {
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
        items: ["light", "dark"],
        dynamicTitle: true,
      },
    },
  },
  initialGlobals: { theme: "default", colorMode: "light" },
  parameters: {
    layout: "padded",
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
        syncCatalogStoryLayout(document, context);
        if (
          context.viewMode === "story" &&
          context.parameters.jjStoreShell === true
        )
          document.body.setAttribute("data-ui-catalog-full-viewport", "");
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
