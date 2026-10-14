import { execFileSync } from "node:child_process";
import { metadata } from "./release-metadata.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
const cohort = process.argv[2];
if (!["ordinary", "heavy"].includes(cohort)) throw Error("unknown cohort");
const packages = metadata().packages.filter(
  (p) =>
    p.name !== "jj-store-embedded-rustfs" &&
    (cohort === "heavy") ===
      ["jj-store", "jj-store-server", "jj-store-capture"].includes(p.name),
);
const selectors = packages.flatMap((p) => ["-p", p.name]);
const cargo = (args) =>
  execFileSync(...cargoCommand(args), { stdio: "inherit" });
cargo([
  "clippy",
  ...selectors,
  "--all-targets",
  "--locked",
  "--",
  "-D",
  "warnings",
]);
execFileSync(process.execPath, ["scripts/ci-tests.mjs", "build", cohort], {
  stdio: "inherit",
});
cargo(["test", ...selectors, "--doc", "--locked"]);
if (cohort === "heavy") {
  cargo(["build", "-p", "jj-store", "--bins", "--examples", "--locked"]);
}
