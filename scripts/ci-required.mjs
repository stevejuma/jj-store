const plan=Object.fromEntries(Object.entries(JSON.parse(process.env.CI_PLAN)).map(([key,value])=>[key,value===true || value==="true"]));
const jobs=JSON.parse(process.env.CI_JOBS);
const conditional={build:plan.rust || plan.web,rust:plan.rust,heavy:plan.heavy,storage:plan.heavy,assets:plan.web,browser:plan.web,native:plan.full};
for (const [name,job] of Object.entries(jobs)) {
  const required=conditional[name] ?? true;
  if (required ? job.result !== "success" : !["success","skipped"].includes(job.result)) throw Error(`required CI lane ${name}: ${job.result}`);
}
for (const name of Object.keys(conditional)) if (!jobs[name]) throw Error(`missing planned CI lane ${name}`);
console.log("All planned CI lanes completed; conditional skips checked.");
