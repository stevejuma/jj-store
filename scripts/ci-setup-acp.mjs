import { createHash } from "node:crypto";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { execFileSync } from "node:child_process";
import { pathToFileURL } from "node:url";
import { probePreparedContext } from "./dev-agent-probe.mjs";

const pins = {
  "darwin-arm64": [
    "aarch64-apple-darwin",
    "a86b4918a28c80ae7dd3a1254bcdbe4181dd0121596b2eefa1989d12e2503598",
  ],
  "darwin-x64": [
    "x86_64-apple-darwin",
    "492804d385f093f0d020beedd82b2fe07660436cf0bb3a2e58c290c4ce38f98e",
  ],
  "linux-arm64": [
    "aarch64-unknown-linux-gnu",
    "8d553e6ae371f421818f67a6a2913519650729b4c4b21fda9539900e0f0ddaa3",
  ],
  "linux-x64": [
    "x86_64-unknown-linux-gnu",
    "469a6729b98ab9305fcc2fe15b748bc643fdb3a73042c11a1afd2796bf8e2069",
  ],
};
export function acpArtifact(platform, arch) {
  const pin = pins[`${platform}-${arch}`];
  if (!pin) throw Error("Unsupported ACP CI platform");
  const root = `jj-acp-0.1.0-${pin[0]}`;
  return { name: `${root}.tar.gz`, member: `${root}/jj-acp`, digest: pin[1] };
}
export function verifyAcp(bytes, digest) {
  if (createHash("sha256").update(bytes).digest("hex") !== digest)
    throw Error("ACP archive checksum mismatch");
}
async function install() {
  if (!process.env.RUNNER_TEMP || !process.env.GITHUB_ENV)
    throw Error("ACP installation requires runner-owned temporary paths");
  const selected = acpArtifact(process.platform, process.arch);
  const response = await fetch(
    `https://github.com/stevejuma/jj-acp/releases/download/v0.1.0/${selected.name}`,
  );
  if (!response.ok) throw Error(`ACP download failed: HTTP ${response.status}`);
  const bytes = Buffer.from(await response.arrayBuffer());
  verifyAcp(bytes, selected.digest);
  const directory = join(process.env.RUNNER_TEMP, "jj-store-ci-acp");
  mkdirSync(directory, { recursive: true });
  const archive = join(directory, selected.name);
  writeFileSync(archive, bytes);
  execFileSync("tar", [
    "-xzf",
    archive,
    "-C",
    directory,
    "--strip-components=1",
    selected.member,
  ]);
  const executable = join(directory, "jj-acp");
  const version = execFileSync(executable, ["--version"], {
    encoding: "utf8",
    timeout: 5000,
  }).trim();
  if (version !== "jj-acp 0.1.0")
    throw Error(`Unexpected ACP version: ${version}`);
  if (!(await probePreparedContext(executable, process.cwd(), process.env)))
    throw Error("Published ACP lacks prepared control-version-3 context");
  appendFileSync(
    process.env.GITHUB_ENV,
    `JJ_STORE_ACP_EXECUTABLE=${executable}\n`,
  );
  console.log(`Installed checksum-verified ${version} for cold CI acceptance`);
}
if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
)
  await install();
