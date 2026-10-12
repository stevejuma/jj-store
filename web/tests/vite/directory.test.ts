import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createProductionConnect } from "../../src/lib/controller/production-store-connect.js";
import { expect, test, vi } from "vitest";
import { createDirectory } from "../../src/lib/controller/directory.svelte.js";
import type { DirectoryRequest } from "../../src/lib/api/directory-client.js";
import type { Model } from "../../src/lib/api/client.js";
function fixture() {
  let profile: Model<"UserProfile"> = {
      principal: "01".repeat(16),
      revision: null,
      details: { display_name: "", description: "", email: null, avatar: null },
    },
    unknown = false,
    token: string | null = null;
  let accepted: Model<"DirectoryReceipt"> | null = null;
  const intents: DirectoryRequest[] = [];
  const fetcher = vi.fn(async (path: RequestInfo | URL, init?: RequestInit) => {
    const url = String(path);
    let value: unknown = null;
    if (init?.method === "POST") {
      const intent = JSON.parse(String(init.body)) as DirectoryRequest;
      intents.push(intent);
      if (intent.command.type === "save_profile") {
        profile = {
          ...profile,
          revision: "ab".repeat(32),
          details: {
            ...intent.command.details,
            email: intent.command.details.email ?? null,
            avatar: intent.command.details.avatar ?? null,
          },
        };
        accepted = {
          request_id: intent.request_id,
          result: { type: "profile", profile },
        };
      }
      value = accepted;
      if (unknown) {
        unknown = false;
        throw Error("lost response");
      }
    } else if (url.endsWith("/profile")) value = profile;
    else if (url.endsWith("/emojis")) value = [];
    else if (url.includes("/profiles?")) value = [];
    else if (url.includes("/commands/")) value = accepted;
    return Response.json(value);
  });
  const manager = createDirectory({
    available: () => true,
    token: () => token,
    fetcher,
  });
  return {
    manager,
    fetcher,
    intents,
    set unknown(v: boolean) {
      unknown = v;
    },
    set token(v: string | null) {
      token = v;
    },
    set profile(v: Model<"UserProfile">) {
      profile = v;
    },
  };
}
test("profile drafts survive refresh and require explicit review of a changed revision", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.state.draft.display_name = "Draft";
  f.profile = {
    ...f.manager.state.profile!,
    revision: "cd".repeat(32),
    details: { ...f.manager.state.profile!.details, display_name: "External" },
  };
  await f.manager.observe();
  expect(f.manager.state.draft.display_name).toBe("Draft");
  expect(f.manager.state.conflict).toBe(true);
  await f.manager.save();
  expect(f.intents).toHaveLength(0);
  f.manager.review();
  await f.manager.save();
  expect(f.intents[0]?.command).toMatchObject({
    expected: "cd".repeat(32),
    details: { display_name: "Draft" },
  });
  f.manager.destroy();
});
test("lost save acknowledgement keeps the original request and recovers its receipt", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.state.draft.display_name = "Saved";
  f.unknown = true;
  await expect(f.manager.save()).rejects.toThrow("lost response");
  const original = structuredClone(f.intents[0]);
  expect(f.manager.state.pending?.request_id).toBe(original.request_id);
  await f.manager.status();
  expect(f.manager.state.pending).toBeNull();
  expect(f.manager.state.draft.display_name).toBe("Saved");
  expect(f.intents).toHaveLength(1);
  f.manager.destroy();
});
test("custom emoji drafts survive navigation and stale session media cannot repopulate cleared state", async () => {
  const f = fixture();
  await f.manager.observe();
  f.manager.state.emojiDraft = {
    shortcode: "party",
    asset: "blob:" + "ab".repeat(32),
    filename: "party.png",
    selected: undefined,
  };
  await f.manager.observe();
  expect(f.manager.state.emojiDraft.shortcode).toBe("party");
  let complete!: (value: Response) => void;
  f.fetcher.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        complete = resolve;
      }),
  );
  const image = f.manager.image("blob:" + "cd".repeat(32));
  await Promise.resolve();
  f.manager.clear();
  complete(
    new Response(new Uint8Array([1]), {
      headers: { "content-type": "image/png" },
    }),
  );
  await expect(image).rejects.toThrow();
  expect(f.manager.state.urls).toEqual({});
  expect(f.manager.state.emojiDraft.shortcode).toBe("");
  f.manager.destroy();
});

test("an enrolled token can authenticate directly from Settings without repository grants", async () => {
  const data = createProductionStoreState();
  data.accessMode = "authenticated";
  const history = createStoreRouteHistory();
  history.globalSettings.open("profile", "details");
  let credential: string | null = null;
  const connectRepository = vi.fn();
  const connect = createProductionConnect({
    data,
    route: () => history.route,
    projects:
      {} as import("../../src/lib/controller/projects-hub-types.js").ProjectsHubController,
    connectRepository,
    disconnect: () => {},
    version: () => 0,
    discovery: () => {},
    token: (value) => {
      credential = value;
    },
    fetcher: async (_input, init) => {
      expect(new Headers(init?.headers).get("authorization")).toBe(
        "Bearer enrolled",
      );
      return Response.json({ version: 1, generation: 0, repositories: [] });
    },
  });
  await connect("", "enrolled");
  expect(credential).toBe("enrolled");
  expect(data.status.repositories).toBe("empty");
  expect(data.status.navigation).toBe("ready");
  expect(connectRepository).not.toHaveBeenCalled();
  history.destroy();
});
