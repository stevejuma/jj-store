/** Register only the owned native checkout through the production management API. */
export async function linkDemoFolder({
  origin,
  repositoryId,
  path,
  fetcher = fetch,
}) {
  const endpoint = `${origin}v1/repos/${repositoryId}/local/workspaces`;
  const headers = {
    Origin: new URL(origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const request = async (url, options = {}) => {
    const response = await fetcher(url, {
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.timeout(30000),
      ...options,
      headers: { ...headers, ...options.headers },
    });
    if (!response.ok)
      throw Error(
        `development folder link failed: ${response.status} ${await response.text()}`,
      );
    return response.json();
  };
  const current = await request(endpoint);
  if (current.links.some((link) => link.path === path)) return;
  await request(`${endpoint}/link`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({
      version: 1,
      expected_revision: current.revision,
      path,
    }),
  });
}
