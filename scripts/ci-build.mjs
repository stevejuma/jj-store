import { execFileSync } from "node:child_process";
import { selectedPackages } from "./ci-packages.mjs";
import { timed } from "./ci-timing.mjs";
import { cargoCommand } from "./managed-cargo.mjs";
const cohort = process.argv[2];
if (!["ordinary", "heavy"].includes(cohort)) throw Error("unknown cohort");
const packages = selectedPackages(cohort);
const selectors = packages.flatMap((name) => ["-p", name]);
process.env.CI_PACKAGES = JSON.stringify(packages);
const cargo = (args) =>
  timed(`Cargo ${args[0]} (${cohort})`, () =>
    execFileSync(...cargoCommand(args), { stdio: "inherit" }),
  );
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
  cargo([
    "build",
    "-p",
    "jj-store-http",
    "--example",
    "browser_preview",
    "--locked",
  ]);
  execFileSync(process.execPath, ["scripts/browser-fixtures.mjs"], {
    stdio: "inherit",
  });
}
