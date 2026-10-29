import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";
import { helperNames, nativeTarget } from "../../lapis-ai/scripts/bundle.mjs";

export async function stubAiExecutable(path, versions = "3", context = true) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(
    path,
    `#!/bin/sh\nif [ "$1" = "--help" ]; then\nprintf '%s\\n' '--control-version <${versions}>'\nelse\nread -r first\nread -r second\nprintf '%s\\n' '${JSON.stringify({ version: 3, type: "error", requestId: "preflight-text-context", code: context ? "session_not_open" : "invalid_request" })}' '${JSON.stringify({ version: 3, type: "error", requestId: "preflight-context-only", code: context ? "session_not_open" : "invalid_request" })}'\ncat >/dev/null\nfi\n`,
    { mode: 0o755 },
  );
}

/** Complete synthetic bundle for launcher tests; it performs no agent work. */
export async function stubAiBundle(root) {
  const bundle = join(root, "desktop/resources/lapis-ai");
  const version = JSON.parse(
    await readFile(
      new URL("../../lapis-ai/package.json", import.meta.url),
      "utf8",
    ),
  ).version;
  let manifest = `version=1\npackage_version='${version}'\ntarget='${nativeTarget()}'\n`;
  for (const name of helperNames) {
    const path = join(bundle, name);
    await stubAiExecutable(path);
    manifest += `[[binary]]\nname='${name}'\npath='${name}'\nsha256='${createHash(
      "sha256",
    )
      .update(await readFile(path))
      .digest("hex")}'\n`;
  }
  await writeFile(join(bundle, "runtime-manifest.toml"), manifest);
  return { bundle, acp: join(bundle, "lapis-acp") };
}
