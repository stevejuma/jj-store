import { readFile, writeFile } from "node:fs/promises";

export async function configureSearch(serverConfig, repositoryId) {
  const generated = await readFile(serverConfig, "utf8");
  const search = `
[[search]]
repository = "${repositoryId}"
poll_millis = 100
timeout_seconds = 120
snapshot_every = 32

[search.source.scopes.main]
kind = "bookmark"
name = "main"
`;
  await writeFile(
    serverConfig,
    `${generated.replace(/^search = \[\]\n/mu, "")}${search}`,
    { mode: 0o600 },
  );
}

function apiHeaders(origin) {
  return {
    Origin: new URL(origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
}

export async function repositoryPosition(origin, repositoryId) {
  const response = await fetch(`${origin}v1/repos/${repositoryId}/snapshot`, {
    headers: apiHeaders(origin),
  });
  if (!response.ok) {
    throw new Error(
      `could not read development repository position: ${response.status} ${await response.text()}`,
    );
  }
  return (await response.json()).published_seq;
}

export async function waitForSearch(
  origin,
  repositoryId,
  minimumPosition = 0,
  fetcher = fetch,
) {
  const capabilities = await fetcher(
    `${origin}v1/repos/${repositoryId}/capabilities`,
    { headers: apiHeaders(origin) },
  );
  if (!capabilities.ok)
    throw new Error(
      `could not read development search capabilities: ${capabilities.status}`,
    );
  if ((await capabilities.json()).search === false)
    return { configured: false };
  const deadline = performance.now() + 300_000;
  const url = `${origin}v1/repos/${repositoryId}/search/status`;
  let lastResponse = "no response";
  while (performance.now() < deadline) {
    try {
      const response = await fetcher(url, { headers: apiHeaders(origin) });
      const searchState = await response.json();
      lastResponse = `${response.status} ${JSON.stringify(searchState)}`;
      if (
        response.ok &&
        searchState.index_generation &&
        searchState.indexed_position >= minimumPosition &&
        searchState.is_stale === false &&
        searchState.degraded === false
      ) {
        return searchState;
      }
    } catch (error) {
      lastResponse = String(error.message ?? error);
    }
    await new Promise((resolveWait) => setTimeout(resolveWait, 100));
  }
  throw new Error(
    `native search did not become ready within 300 seconds: ${lastResponse}`,
  );
}
