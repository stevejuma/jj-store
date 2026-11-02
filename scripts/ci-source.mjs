import { appendFileSync } from "node:fs";
import { sourceSha } from "./ci-inventory.mjs";
import { cacheWritable, sourceOnMain } from "./ci-plan.mjs";
const source = sourceSha();
if (
  process.env.GITHUB_EVENT_NAME === "schedule" &&
  (process.env.GITHUB_REPOSITORY !== "stevejuma/jj-store" ||
    process.env.GITHUB_REF !== "refs/heads/main")
)
  throw Error("Scheduled qualification requires canonical main");
const cache_write = cacheWritable({
  event: process.env.GITHUB_EVENT_NAME,
  ref: process.env.GITHUB_REF,
  repository: process.env.GITHUB_REPOSITORY,
  sourceOnMain: sourceOnMain(source),
});
appendFileSync(
  process.env.GITHUB_OUTPUT,
  `source=${source}\ncache_write=${cache_write}\n`,
);
if (process.env.GITHUB_STEP_SUMMARY)
  appendFileSync(
    process.env.GITHUB_STEP_SUMMARY,
    `Qualification source: \`${source}\`. Shared cache writes: ${cache_write}.\n`,
  );
