import { execFileSync } from "node:child_process";
import { version } from "./release-metadata.mjs";
const repository = process.env.GITHUB_REPOSITORY;
if (
  repository !== "stevejuma/jj-store" ||
  process.env.GITHUB_REF !== "refs/heads/main"
)
  throw Error("release orchestration requires canonical main");
const sha = process.env.GITHUB_SHA;
if (!/^[a-f0-9]{40}$/.test(sha)) throw Error("invalid release source identity");
const tag = `v${version()}`;
const gh = (args) => execFileSync("gh", args, { encoding: "utf8" });
let existing;
try {
  existing = JSON.parse(gh(["api", `repos/${repository}/git/ref/tags/${tag}`]));
} catch (error) {
  if (!String(error.stderr).includes("404")) throw error;
}
if (existing) {
  console.log(
    "This version already has an immutable tag; use an explicit recovery run for incomplete publication.",
  );
  process.exit(0);
}
gh([
  "workflow",
  "run",
  "publish-release.yml",
  "--ref",
  "main",
  "-f",
  `tag=${tag}`,
  "-f",
  `source=${sha}`,
]);
console.log(
  `Dispatched full qualification and publication for ${tag} at ${sha}`,
);
