import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
const hash = async (path) =>
  createHash("sha256")
    .update(await readFile(path))
    .digest("hex");
export const developmentFixtureIdentity = async (layout) => ({
  fixture: await hash(layout.fixtureFile),
  installation: await hash(join(layout.stateRoot, "installation.json")),
});
