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
    const capture = join(directory, "responses.json");
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
        "content_acceptance::http_acceptance::http_contract_capture_matches_registered_routes",
        "--",
        "--exact",
      ],
      {
        cwd: root,
        env: { ...process.env, JJ_STORE_CONTRACT_CAPTURE: capture },
        maxBuffer: 4 * 1024 * 1024,
      },
    );
    if (!stdout.includes("1 passed; 0 failed"))
      throw new Error(`Contract fixture did not run: ${stdout}`);
    const samples = JSON.parse(await readFile(capture, "utf8"));
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
