const rawPlan = JSON.parse(process.env.CI_PLAN);
const plan = Object.fromEntries(
  Object.entries(rawPlan).map(([key, value]) => [
    key,
    value === true || value === "true",
  ]),
);
const jobs = JSON.parse(process.env.CI_JOBS);
const stage = process.env.CI_STAGE || "root";
if (stage === "root") {
  let selection;
  try {
    selection = JSON.parse(rawPlan.selection);
  } catch {
    throw Error("Missing or invalid active selection schema");
  }
  if (selection.version !== 2 || selection.mode !== "active")
    throw Error("Missing or invalid active selection schema");
  for (const field of [
    "rust",
    "web",
    "backend",
    "ui_source",
    "ui_browser",
    "help",
    "catalog",
    "contracts",
  ]) {
    if (![true, false, "true", "false"].includes(rawPlan[field]))
      throw Error(`Missing or invalid planned selection ${field}`);
    if (
      field !== "rust" &&
      field !== "web" &&
      field !== "backend" &&
      selection.domains?.[field] !== plan[field]
    )
      throw Error(`Active selection domain mismatch ${field}`);
  }
  const selectedWeb =
    plan.ui_source ||
    plan.ui_browser ||
    plan.help ||
    plan.catalog ||
    plan.contracts;
  if (plan.web !== selectedWeb)
    throw Error("Planned web selection disagrees with active domains");
}
if (!["root", "heavy", "early"].includes(stage))
  throw Error("Unknown CI stage");
const conditional =
  stage === "early"
    ? { plan: true, format: true, tooling: true, governance: true }
    : stage === "heavy"
      ? {
          resolve: true,
          standard: true,
          build: true,
          backend: true,
          runtime: true,
          contracts: true,
          external: true,
          embedded: true,
          assets: true,
          cold: true,
          native: true,
        }
      : {
          plan: true,
          format: true,
          tooling: true,
          governance: true,
          fast: true,
          build: plan.rust,
          rust: plan.rust,
          targeted_build: plan.backend,
          targeted: plan.backend,
          web: plan.ui_source || plan.ui_browser || plan.help || plan.contracts,
          catalog_unit: plan.catalog,
          ui: plan.ui_source || plan.ui_browser || plan.help || plan.catalog,
        };
for (const [name, job] of Object.entries(jobs)) {
  const required = conditional[name] ?? true;
  if (
    required
      ? job.result !== "success"
      : !["success", "skipped"].includes(job.result)
  )
    throw Error(`required CI lane ${name}: ${job.result}`);
}
for (const name of Object.keys(conditional))
  if (!jobs[name]) throw Error(`missing planned CI lane ${name}`);
console.log("All planned CI lanes completed; conditional skips checked.");
