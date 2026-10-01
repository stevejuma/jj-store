import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { mkdtemp, readFile, writeFile, mkdir, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import openapiTS, { astToString } from "openapi-typescript";
import SwaggerParser from "@apidevtools/swagger-parser";
import { format, resolveConfig } from "prettier";
import { responseValidator } from "./contracts-validation.mjs";

const root = fileURLToPath(new URL("..", import.meta.url));
const run = promisify(execFile);
const mode = process.argv[2];
if (!["--write", "--check"].includes(mode)) {
  throw new Error("usage: node scripts/contracts.mjs --write|--check");
}
const { stdout } = await run(
  "cargo",
  ["run", "--quiet", "--locked", "-p", "jj-store-http", "--example", "openapi"],
  { cwd: root, maxBuffer: 16 * 1024 * 1024 },
);
const document = JSON.parse(stdout);
// No remote reference retrieval is needed or allowed by generation.
await SwaggerParser.validate(structuredClone(document), {
  resolve: { external: false },
});
const options = await resolveConfig(join(root, "package.json"));
const artifacts = new Map([
  [
    "schemas/openapi.json",
    await format(JSON.stringify(document), { ...options, parser: "json" }),
  ],
  [
    "web/src/lib/api/generated.ts",
    await format(astToString(await openapiTS(document)), {
      ...options,
      parser: "typescript",
    }),
  ],
]);
for (const [path, text] of artifacts) {
  const target = join(root, path);
  if (mode === "--write") {
    await mkdir(dirname(target), { recursive: true });
    await writeFile(target, text);
  } else {
    const existing = await readFile(target, "utf8").catch(() => null);
    if (existing !== text) {
      throw new Error(
        `Generated contract drift: ${path}. Run pnpm contracts:generate.`,
      );
    }
  }
}
if (mode === "--check") {
  const directory = await mkdtemp(join(tmpdir(), "jj-store-contract-"));
  try {
    const samples = [];
    for (const [fixture, variable] of [
      [
        "content_acceptance::http_acceptance::local_management::local_workspace_management_requires_local_admin_and_captures_contracts",
        "JJ_STORE_LOCAL_MANAGEMENT_CONTRACT_CAPTURE",
      ],
      [
        "content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes",
        "JJ_STORE_CONTRACT_CAPTURE",
      ],
      [
        "http_mutation_acceptance::http_mutations_preview_apply_status_replay_and_contract_capture",
        "JJ_STORE_MUTATION_CONTRACT_CAPTURE",
      ],
      [
        "http_mutation_acceptance::http_abandon_previews_native_effects_and_reports_hidden_noop_without_publication",
        "JJ_STORE_ABANDON_CONTRACT_CAPTURE",
      ],
      [
        "content_acceptance::diff_acceptance::conflict_acceptance::resolve_acceptance::http_resolve_validates_atomic_fingerprints_permissions_and_selected_receipts",
        "JJ_STORE_RESOLVE_CONTRACT_CAPTURE",
      ],
      [
        "content_acceptance::diff_acceptance::conflict_acceptance::resolve_acceptance::uploads_acceptance::conflict_upload_http_stages_verified_binary_without_publication_then_resolves",
        "JJ_STORE_UPLOAD_CONTRACT_CAPTURE",
      ],
      [
        "content_acceptance::adapter_projection::search_paging::http_search::http_search_pins_results_across_writes_and_captures_contracts",
        "JJ_STORE_SEARCH_CONTRACT_CAPTURE",
      ],
      [
        "http_browsing_acceptance::http_browsing_windows_are_authorized_pinned_cursor_bound_and_contract_captured",
        "JJ_STORE_BROWSING_CONTRACT_CAPTURE",
      ],
      [
        "http_collaboration_acceptance::local_trusted_http_lists_root_pinned_agent_sessions_without_bearer_credentials",
        "JJ_STORE_COLLABORATION_CONTRACT_CAPTURE",
      ],
      [
        "http_collaboration_checkpoint_acceptance::local_trusted_http_pages_current_and_historical_agent_checkpoints",
        "JJ_STORE_CHECKPOINT_CONTRACT_CAPTURE",
      ],
      [
        "http_collaboration_review_acceptance::local_trusted_http_pages_root_pinned_review_threads_and_resolves_detail",
        "JJ_STORE_REVIEW_CONTRACT_CAPTURE",
      ],
      [
        "http_work_acceptance::http_workstore_pages_commands_permissions_and_contract_capture",
        "JJ_STORE_WORK_CONTRACT_CAPTURE",
      ],
    ]) {
      const capture = join(directory, `${variable}.json`);
      const { stdout } = await run(
        "cargo",
        [
          "test",
          "--quiet",
          "--locked",
          "-p",
          "jj-store-cli",
          "--test",
          "native_lifecycle",
          fixture,
          "--",
          "--exact",
        ],
        {
          cwd: root,
          env: { ...process.env, [variable]: capture },
          maxBuffer: 4 * 1024 * 1024,
        },
      );
      if (!stdout.includes("1 passed; 0 failed"))
        throw new Error(`Contract fixture did not run: ${stdout}`);
      samples.push(...JSON.parse(await readFile(capture, "utf8")));
    }
    const validate = responseValidator(document);
    for (const sample of samples) validate(sample);
    console.log(
      `Generated contracts match; ${samples.length} actual HTTP/SSE responses conform.`,
    );
  } finally {
    // This directory was created by this invocation and holds only disposable fixtures.
    await rm(directory, { recursive: true, force: true });
  }
} else {
  console.log(
    "Generated schemas/openapi.json and web/src/lib/api/generated.ts",
  );
}
