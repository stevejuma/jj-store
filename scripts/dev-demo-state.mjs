import {
  mkdir,
  readFile,
  readdir,
  rename,
  stat,
  writeFile,
} from "node:fs/promises";
import { join, resolve } from "node:path";

const fixtureFormat = 1;

export function developmentLayout(repositoryRoot, environment = process.env) {
  const runtimeRoot = resolve(
    repositoryRoot,
    environment.JJ_STORE_DEV_ROOT ?? ".local/dev-demo",
  );
  const stateRoot = join(runtimeRoot, "local");
  return {
    runtimeRoot,
    runtimeFile: join(runtimeRoot, "runtime.json"),
    fixtureFile: join(runtimeRoot, "fixture.json"),
    logPath: join(runtimeRoot, "server.log"),
    stateRoot,
    serverConfig: join(stateRoot, "server.toml"),
    unixSocket: join(stateRoot, "native.sock"),
    rustfsVolume: join(stateRoot, "rustfs"),
    repository: join(runtimeRoot, "repository"),
  };
}

async function requiredPath(path, expected) {
  try {
    const details = await stat(path);
    if (
      (expected === "directory" && !details.isDirectory()) ||
      (expected === "file" && !details.isFile())
    ) {
      throw new Error(`${path} is not a ${expected}`);
    }
  } catch (error) {
    throw new Error(`development fixture is incomplete at ${path}`, {
      cause: error,
    });
  }
}

function resetGuidance(error) {
  return new Error(
    `development fixture is incomplete; run pnpm dev:reset to rebuild ${error.message}`,
    { cause: error },
  );
}

export async function loadFixture(layout) {
  let source;
  try {
    source = await readFile(layout.fixtureFile, "utf8");
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    try {
      const entries = await readdir(layout.runtimeRoot);
      if (entries.length) throw resetGuidance(error);
    } catch (directoryError) {
      if (directoryError.code !== "ENOENT") throw directoryError;
    }
    return null;
  }
  try {
    const fixture = JSON.parse(source);
    if (
      fixture.format !== fixtureFormat ||
      !/^[0-9a-f]{32}$/u.test(fixture.repositoryId) ||
      !Number.isInteger(fixture.firstStartupMillis) ||
      fixture.firstStartupMillis < 0
    ) {
      throw new Error("fixture.json has an unsupported shape");
    }
    await Promise.all([
      requiredPath(layout.repository, "directory"),
      requiredPath(layout.rustfsVolume, "directory"),
      requiredPath(layout.serverConfig, "file"),
    ]);
    return fixture;
  } catch (error) {
    throw resetGuidance(error);
  }
}

export async function writeFixture(layout, fixture) {
  const value = { format: fixtureFormat, ...fixture };
  await mkdir(layout.runtimeRoot, { recursive: true, mode: 0o700 });
  const temporary = `${layout.fixtureFile}.tmp`;
  await writeFile(temporary, `${JSON.stringify(value, null, 2)}\n`, {
    mode: 0o600,
  });
  await rename(temporary, layout.fixtureFile);
  return value;
}

export function rewriteServerConfiguration(
  source,
  { origin, webRoot, browserAnnotations = true },
) {
  let updated = source
    .replace(/^bind = ".*"$/mu, `bind = "${new URL(origin).host}"`)
    .replace(/^public_url = ".*"$/mu, `public_url = "${origin}"`);
  const webLine = `web_root = ${JSON.stringify(webRoot)}`;
  if (/^web_root = .+$/mu.test(updated)) {
    updated = updated.replace(/^web_root = .+$/mu, webLine);
  } else {
    updated = updated.replace(/^(operator_policy = .+)$/mu, `$1\n${webLine}`);
  }
  const annotationsLine = `development_browser_annotations = ${browserAnnotations}`;
  if (/^development_browser_annotations = .+$/mu.test(updated)) {
    updated = updated.replace(
      /^development_browser_annotations = .+$/mu,
      annotationsLine,
    );
  } else {
    updated = updated.replace(/^(web_root = .+)$/mu, `$1\n${annotationsLine}`);
  }
  return updated;
}
