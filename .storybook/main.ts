import type { StorybookConfig } from "@storybook/svelte-vite";
import { mergeConfig } from "vite";
const config: StorybookConfig = {
  stories: ["../catalog/**/*.stories.svelte"],
  staticDirs: ["../web/public"],
  addons: [
    "@storybook/addon-docs",
    "@storybook/addon-a11y",
    "@storybook/addon-svelte-csf",
    "@storybook/addon-themes",
    "@storybook/addon-vitest",
    {
      name: "@lapismd/storybook-addon-visual-delta",
      options: {
        visualDelta: {
          visualServerPort: Number(process.env.VISUAL_SERVER_PORT ?? 6047),
        },
      },
    },
  ],
  framework: {
    name: "@storybook/svelte-vite",
    options: { builder: { viteConfigPath: ".storybook/vite.config.ts" } },
  },
  core: { disableTelemetry: true },
  viteFinal: async (config) =>
    mergeConfig(config, {
      resolve: { dedupe: ["svelte"] },
      server: {
        host: "127.0.0.1",
        watch: { ignored: ["**/storybook-static/**", "**/target/**"] },
      },
    }),
};
export default config;
