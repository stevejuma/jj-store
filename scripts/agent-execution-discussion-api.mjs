import { expect } from "@playwright/test";
export function discussionApi(page, origin, admission, repository, profile) {
  const local = profile === "local_jj_git";
  const root = `/local/repos/${repository}/collaboration`;
  const get = async (path) => {
    const r = await page.request.get(`${origin}/v1${path}`, {
      headers: admission,
    });
    expect(r.ok(), await r.text()).toBeTruthy();
    return r.json();
  };
  const post = (path, data) =>
    page.request.post(`${origin}/v1${path}`, { headers: admission, data });
  return {
    get,
    commandPath: (kind) =>
      local ? `${root}/commands` : `/repos/${repository}/${kind}/commands`,
    command(kind, command) {
      return post(
        local ? `${root}/commands` : `/repos/${repository}/${kind}/commands`,
        {
          version: 1,
          request_id: crypto.randomUUID().replaceAll("-", ""),
          command: local
            ? {
                domain: kind === "threads" ? "thread" : "reaction",
                intent: command,
              }
            : command,
        },
      );
    },
    receipt: (kind, id) =>
      get(
        local
          ? `${root}/commands/${id}`
          : `/repos/${repository}/${kind}/commands/${id}`,
      ),
    async thread(id) {
      if (!local) return get(`/repos/${repository}/threads/${id}`);
      const selection = await get(`${root}/snapshot`);
      const r = await post(`${root}/thread`, {
        metadata_commit: selection.snapshot.metadata_commit,
        thread_id: id,
      });
      expect(r.ok(), await r.text()).toBeTruthy();
      return r.json();
    },
  };
}
export async function qualifyNativeFeedback(
  page,
  workspace,
  origin,
  admission,
  repository,
  original,
  feedback,
) {
  expect(original.write_unavailable).toContain("collaboration replication");
  const api = discussionApi(page, origin, admission, repository, "native");
  const refused = await api.command("threads", {
    type: "create_agent_discussion",
    thread_id: crypto.randomUUID().replaceAll("-", ""),
    title: "Refused code repository feedback",
    target: original.messages[0].target,
  });
  expect(refused.status()).toBe(409);
  expect(await refused.text()).toContain(
    "collaboration_replication_unsupported",
  );
  const message = workspace.locator(
    `[data-message-id="${original.messages[0].row_id}"]`,
  );
  await message.hover();
  await message.getByRole("button", { name: "Reply", exact: true }).click();
  const panel = workspace.getByRole("region", { name: /Replies to/ });
  await expect(
    panel.getByRole("button", { name: "Save reply" }),
  ).toBeDisabled();
  await expect(
    panel.getByText(/until collaboration replication is qualified/),
  ).toBeVisible();
  await message.getByRole("button", { name: "Reply", exact: true }).click();
  // The positive native fixture uses the existing supported WorkStore-home profile.
  const home = await page.request.post(
    `${origin}/v1/repos/${repository}/work/commands`,
    {
      headers: admission,
      data: {
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        command: {
          command: "create_project",
          id: crypto.randomUUID().replaceAll("-", ""),
          title: "Feedback qualification home",
          body: "Disposable supported WorkStore fixture",
        },
      },
    },
  );
  expect(home.ok()).toBeTruthy();
  expect((await home.json()).status).toBe("committed");
  await expect
    .poll(async () => (await feedback()).write_unavailable)
    .toBeUndefined();
  await page.reload();
}
