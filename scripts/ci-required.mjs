const rawPlan = JSON.parse(process.env.CI_PLAN);
const plan = Object.fromEntries(
  Object.entries(rawPlan).map(([key, value]) => [
    key,
    value === true || value === "true",
  ]),
);
const jobs = JSON.parse(process.env.CI_JOBS);
const stage = process.env.CI_STAGE || "root";
if (stage === "root")
  for (const field of ["rust", "web"]) {
    if (![true, false, "true", "false"].includes(rawPlan[field]))
      throw Error(`Missing or invalid planned selection ${field}`);
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
          web: plan.web,
          ui: plan.web,
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
