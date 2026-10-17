import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { repositoryRoot } from "./lapis-ai.mjs";
import { ensurePinnedSource } from "./ci-pinned-source.mjs";

export function validateAiSourcePin(pin) {
  if (
    pin.repository !== "lapismd/lapis-ai" ||
    !/^[a-f0-9]{40}$/.test(pin.revision) ||
    !/^\d+\.\d+\.\d+$/.test(pin.version)
  )
    throw Error("Invalid Lapis AI source pin");
  return pin;
}
export async function ensureAiSource(
  root = repositoryRoot,
  environment = process.env,
) {
  const pin = validateAiSourcePin(
    JSON.parse(
      await readFile(join(root, "scripts/lapis-ai-source.json"), "utf8"),
    ),
  );
  return ensurePinnedSource(pin, {
    root,
    environment,
    name: "lapis-ai",
    packageName: "@lapismd/lapis-ai",
  });
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await ensureAiSource();
