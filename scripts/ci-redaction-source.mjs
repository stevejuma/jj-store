import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { repositoryRoot } from "./lapis-ai.mjs";
import { ensurePinnedSource } from "./ci-pinned-source.mjs";
export function validateRedactionSourcePin(pin) {
  if (
    pin.repository !== "lapismd/agent-redaction" ||
    !/^[a-f0-9]{40}$/.test(pin.revision) ||
    !/^\d+\.\d+\.\d+$/.test(pin.version)
  )
    throw Error("Invalid native redaction source pin");
  return pin;
}
export async function ensureRedactionSource(
  root = repositoryRoot,
  environment = process.env,
) {
  const pin = validateRedactionSourcePin(
    JSON.parse(
      await readFile(join(root, "scripts/agent-redaction-source.json"), "utf8"),
    ),
  );
  return ensurePinnedSource(pin, {
    root,
    environment,
    name: "agent-redaction",
    packageName: "@lapismd/agent-redaction",
  });
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await ensureRedactionSource();
