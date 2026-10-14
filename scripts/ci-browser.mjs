import { execFileSync } from "node:child_process";
const pnpm = (...args) => execFileSync("pnpm", args, { stdio: "inherit" });
const lane = process.argv[2];
if (lane === "storybook") pnpm("test:storybook");
else if (lane === "browser") {
  pnpm("catalog:test:browser");
  pnpm("web:test:browser");
  pnpm("docs:test:browser");
} else if (lane === "cold") pnpm("web:test:real");
else throw Error("unknown browser lane");
