import { readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

/** Explicit intentional examples, independently of captured agent conversations. */
export async function seedThreads({
  origin,
  runtimeRoot,
  repositories,
  fetcher = fetch,
}) {
  const marker = join(runtimeRoot, "intentional-threads-v1.json");
  try {
    return JSON.parse(await readFile(marker, "utf8"));
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
  }
  const samples = [
    { title: "Ideas without a repository", repositories: [] },
    {
      title: "Review the sample codebase",
      repositories: repositories.slice(0, 1),
    },
    {
      title: "Coordinate two codebases",
      repositories: repositories.slice(0, 2),
    },
  ];
  const outcomes = [];
  for (const [index, command] of samples.entries()) {
    const response = await fetcher(new URL("v1/local/threads", origin), {
      method: "POST",
      credentials: "omit",
      redirect: "error",
      cache: "no-store",
      signal: AbortSignal.timeout(35000),
      headers: {
        "content-type": "application/json",
        "x-jj-store-protocol": "1",
        Origin: new URL(origin).origin,
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-origin",
      },
      body: JSON.stringify({
        version: 1,
        request_id: ("e" + (index + 1)).repeat(16),
        command: { type: "create", ...command },
      }),
    });
    if (!response.ok)
      throw new Error(
        `Intentional Thread seed failed: ${response.status} ${await response.text()}`,
      );
    outcomes.push(await response.json());
  }
  await writeFile(marker, JSON.stringify(outcomes, null, 2) + "\n");
  return outcomes;
}
