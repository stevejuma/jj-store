import { cp } from "node:fs/promises";
import { constants } from "node:fs";
import { join } from "node:path";
import { verifyAiBundle } from "./lapis-ai.mjs";

export async function stageAgentResources(destination, source) {
  await verifyAiBundle(source);
  const target = join(destination, "lapis-ai");
  await cp(source, target, {
    recursive: true,
    mode: constants.COPYFILE_FICLONE,
  });
  await verifyAiBundle(target);
}
