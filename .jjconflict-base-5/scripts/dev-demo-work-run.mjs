import { readFile } from "node:fs/promises";
import { loadFixture } from "./dev-demo-state.mjs";
import { seedRepositoryProfiles } from "./dev-demo-profiles.mjs";
import { seedWork } from "./dev-demo-work.mjs";

export async function seedSampleWork({
  layout,
  repositoryRoot,
  serverPort,
  run,
}) {
  const fixture = await loadFixture(layout);
  if (!fixture)
    throw new Error(
      "start the owned development demo with pnpm dev:restart first",
    );
  const runtime = JSON.parse(await readFile(layout.runtimeFile, "utf8"));
  if (
    runtime.runtimeRoot !== layout.runtimeRoot ||
    runtime.serverConfig !== layout.serverConfig ||
    runtime.repositoryId !== fixture.repositoryId ||
    runtime.origin !== `http://127.0.0.1:${serverPort}/`
  )
    throw new Error(
      "development work seed requires the matching owned LocalTrusted demo runtime",
    );
  const ownedProcess = await run("ps", [
    "-p",
    String(runtime.serverPid),
    "-o",
    "command=",
  ]);
  if (!ownedProcess.stdout.includes(layout.serverConfig))
    throw new Error("development work seed refuses an unowned process");
  const profiles = await seedRepositoryProfiles({
    repositoryRoot,
    runtimeRoot: layout.runtimeRoot,
    repositoryId: fixture.repositoryId,
    unixSocket: layout.unixSocket,
    run,
  });
  const result = await seedWork({
    repositoryLinks: profiles.repositoryLinks,
    origin: runtime.origin,
    repositoryId: fixture.repositoryId,
  });
  console.log(
    `Development work samples ready: ${result.projects} projects, ${result.tasks} tasks, ${result.comments} comments; ${result.publications} new publications. Existing records preserved. Refresh projects to load the samples.`,
  );
}
