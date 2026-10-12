import assert from "node:assert/strict";

export function developmentApi(origin) {
  const headers = {
    "x-jj-store-protocol": "1",
    Origin: new URL(origin).origin,
    "Sec-Fetch-Site": "same-origin",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Dest": "empty",
  };
  const api = async (path, options = {}) => {
    const response = await fetch(`${origin}v1/${path}`, {
      ...options,
      headers: { ...headers, ...options.headers },
      signal: AbortSignal.timeout(10000),
    });
    assert.equal(
      response.status,
      200,
      `${path}: ${await response.clone().text()}`,
    );
    return response.json();
  };
  const agentsAvailable = async () => {
    assert.equal(
      (await api("bootstrap")).capabilities.local_agent_execution,
      true,
    );
    const status = await api("local/agents");
    assert.ok(Array.isArray(status.setups));
  };
  return { api, agentsAvailable, headers };
}
