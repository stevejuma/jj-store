import { defineVisualPlaywrightConfig } from "@lapismd/storybook-addon-visual-delta/playwright";

export default defineVisualPlaywrightConfig({
  port: Number(process.env.VISUAL_SERVER_PORT ?? 6047),
});
