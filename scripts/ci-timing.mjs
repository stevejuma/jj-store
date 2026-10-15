import { appendFileSync, mkdirSync } from "node:fs";
export function timed(label, operation) {
  const start = performance.now();
  let result = "failure";
  try {
    const value = operation();
    result = "success";
    return value;
  } finally {
    const seconds = ((performance.now() - start) / 1000).toFixed(2);
    console.log(`${label}: ${seconds}s (${result})`);
    if (process.env.JJ_STORE_CI_REPORTS === "1") {
      mkdirSync(".release/ci-results", { recursive: true });
      appendFileSync(
        ".release/ci-results/operations.jsonl",
        JSON.stringify({ label, seconds: Number(seconds), result }) + "\n",
      );
    }
    if (process.env.GITHUB_STEP_SUMMARY)
      appendFileSync(
        process.env.GITHUB_STEP_SUMMARY,
        `- ${label}: ${seconds}s (${result})\n`,
      );
  }
}
