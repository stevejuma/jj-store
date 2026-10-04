import { workSeedId } from "./dev-demo-work-samples.mjs";
import { realpath } from "node:fs/promises";
/** Admit the existing fixture without reseeding code or changing its native project home. */
export async function registerDemoLocalFolder({
  origin,
  repositoryId,
  path,
  fetcher = fetch,
}) {
  path = await realpath(path);
  const headers = {
    Origin: new URL(origin).origin,
    "Sec-Fetch-Dest": "empty",
    "Sec-Fetch-Mode": "cors",
    "Sec-Fetch-Site": "same-origin",
    "X-JJ-Store-Protocol": "1",
  };
  const request = async (route, body) => {
    const response = await fetcher(`${origin}v1/local/repositories${route}`, {
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.timeout(35000),
      method: body ? "POST" : "GET",
      headers: {
        ...headers,
        ...(body ? { "content-type": "application/json" } : {}),
      },
      ...(body ? { body: JSON.stringify(body) } : {}),
    });
    if (!response.ok)
      throw Error(
        `local fixture registration failed: ${response.status} ${await response.text()}`,
      );
    return response.json();
  };
  const current = await request("");
  const binding = current.bindings.find((b) => b.repository === repositoryId);
  if (binding) {
    if (binding.path !== path || binding.availability !== "ready")
      throw Error("local fixture binding changed; reconnect it explicitly");
    return binding;
  }
  const requestId = workSeedId(
    repositoryId,
    "request",
    "browser-folder-registration",
  );
  const prior = await request(`/requests/${requestId}`);
  const outcome =
    prior.status === "committed"
      ? prior
      : await request("", {
          version: 1,
          request_id: requestId,
          expected_revision: current.revision,
          path,
          name: "Local Git-backed JJ",
        });
  if (
    outcome.status !== "committed" ||
    outcome.binding.repository !== repositoryId
  )
    throw Error(
      "local fixture registration outcome unknown or identity changed; retain the request and retry",
    );
  return outcome.binding;
}
