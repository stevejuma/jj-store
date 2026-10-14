import { execFileSync } from "node:child_process";
const pnpm = (...args) => execFileSync("pnpm", args, { stdio: "inherit" });
const lane = process.argv[2];
const prebuilt =
  process.env.CI === "true" && process.env.JJ_STORE_CI_PREBUILT === "1";
if (lane === "storybook") pnpm("test:storybook");
else if (lane === "browser" && prebuilt) {
  pnpm("exec", "node", "--test", "scripts/catalog-browser.test.mjs");
  pnpm("exec", "playwright", "test", "--config", "web/playwright.config.ts");
  pnpm(
    "exec",
    "playwright",
    "test",
    "--config",
    "web/playwright.docs.config.ts",
  );
} else if (lane === "browser") {
  pnpm("catalog:test:browser");
  pnpm("web:test:browser");
  pnpm("docs:test:browser");
} else if (lane === "cold") pnpm("web:test:real");
else throw Error("unknown browser lane");
