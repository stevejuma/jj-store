import { restorePinnedSources } from "./pins.mjs";
import { execute } from "./process.mjs";
import { ensureAiSource } from "../ci-lapis-source.mjs";
import { ensureRedactionSource } from "../ci-redaction-source.mjs";
export async function prepareFull(root, input, source, lane) {
  await restorePinnedSources(root, input, source.pins);
  await ensureAiSource(root, { CI: "true" });
  await ensureRedactionSource(root, { CI: "true" });
  await execute("pnpm", ["install", "--frozen-lockfile"], {
    cwd: "/work/lapis-ai",
  });
  if (lane === "test:governance" || lane === "test") {
    // The aggregate test lane nests the governance suite, so both need the
    // offline clean-install regressions provisioned with registry metadata
    // as well as locked tarballs. Hosted CI mirrors this via
    // scripts/ci-provision-offline-governance.mjs in .github/actions/setup.
    await execute("node", ["scripts/ci-provision-offline-governance.mjs"], {
      cwd: root,
    });
    await execute("cargo", ["fetch", "--locked"], {
      cwd: root,
    });
  }
}
