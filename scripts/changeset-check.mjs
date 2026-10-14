import { execFileSync } from "node:child_process";
import { version } from "./release-metadata.mjs";
if (process.env.CI_EVENT !== "pull_request") process.exit(0);
if (
  process.env.GITHUB_HEAD_REF === "changeset-release/main" ||
  (process.env.CI_PR_AUTHOR === "dependabot[bot]" &&
    process.env.GITHUB_HEAD_REF?.startsWith("dependabot/"))
) {
  console.log(
    "Generated release/dependency PR: no authored spec evidence or Changesets note required; qualification checks remain enabled.",
  );
  process.exit(0);
}
const git = (args) => execFileSync("git", args, { encoding: "utf8" }).trim();
try {
  git(["rev-parse", "--verify", `refs/tags/v${version()}`]);
} catch {
  console.log(
    "Initial publication or version PR: release evidence and coordinated metadata govern this revision.",
  );
  process.exit(0);
}
const files = git([
  "diff",
  "--name-only",
  process.env.JJ_STORE_CHECK_BASE,
  process.env.JJ_STORE_CHECK_REVISION,
]).split("\n");
const published = files.some((p) =>
  /^(?:crates\/|runtime\/embedded-rustfs\/|Cargo\.|Dockerfile|\.github\/workflows\/publish-release|scripts\/release)/.test(
    p,
  ),
);
if (
  published &&
  !files.some(
    (p) => /^\.changeset\/[^/]+\.md$/.test(p) && p !== ".changeset/README.md",
  )
)
  throw Error(
    "Published behavior or distribution changes require a Changesets note for @lapismd/jj-store.",
  );
