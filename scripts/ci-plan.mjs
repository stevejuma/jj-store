import { execFileSync } from "node:child_process";
import { appendFileSync } from "node:fs";
export function impact(files, full = false) {
  const shared =
    full ||
    files.some((f) =>
      /^(?:Cargo\.|rust-toolchain|pnpm-lock|package.json|\.github\/|\.config\/|scripts\/ci-|scripts\/managed-|scripts\/build-storage)/.test(
        f,
      ),
    );
  const rust =
    shared ||
    files.some((f) =>
      /^(?:crates\/|runtime\/embedded-rustfs\/|scripts\/(?:contracts|docs\/references|.*acceptance))/.test(
        f,
      ),
    );
  const web =
    shared ||
    files.some((f) =>
      /^(?:web\/|catalog\/|\.storybook\/|scripts\/.*browser|docs\/site\/assets\/)/.test(
        f,
      ),
    );
  return { rust, web };
}
if (process.argv[1]?.endsWith("ci-plan.mjs")) {
  const files = execFileSync(
    "git",
    [
      "diff",
      "--name-only",
      process.env.JJ_STORE_CHECK_BASE,
      process.env.JJ_STORE_CHECK_REVISION,
    ],
    { encoding: "utf8" },
  )
    .trim()
    .split("\n")
    .filter(Boolean);
  const result = impact(
    files,
    process.env.CI_ALL_CHECKS === "true" ||
      (process.env.CI_EVENT === "push" &&
        process.env.CI_REF === "refs/heads/main"),
  );
  for (const [key, value] of Object.entries(result))
    appendFileSync(process.env.GITHUB_OUTPUT, `${key}=${value}\n`);
  appendFileSync(
    process.env.GITHUB_OUTPUT,
    `source=${process.env.JJ_STORE_CHECK_REVISION}\n`,
  );
  console.log(JSON.stringify({ files, ...result }));
}
