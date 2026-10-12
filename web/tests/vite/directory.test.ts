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
    unknownCommand: DirectoryRequest["command"]["type"] | null = null,
    token: string | null = null;
  let accepted: Model<"DirectoryReceipt"> | null = null;
  const intents: DirectoryRequest[] = [];
  const png = Uint8Array.from(
    Buffer.from(
      "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGP4z8DwHwAFAAH/iZk9HQAAAABJRU5ErkJggg==",
      "base64",
    ),
  );
  const fetcher = vi.fn(async (path: RequestInfo | URL, init?: RequestInit) => {
    const url = String(path);
    let value: unknown = null;
    if (init?.method === "POST") {
      const intent: DirectoryRequest = url.includes("/image-uploads/")
        ? {
            request_id: url.split("/").at(-1)!,
            command: {
              type: "upload_image",
              bytes: [
                ...new Uint8Array(await new Response(init.body).arrayBuffer()),
              ],
            },
          }
        : (JSON.parse(String(init.body)) as DirectoryRequest);
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
      if (intent.command.type === "upload_image") {
        accepted = {
          request_id: intent.request_id,
          result: {
            type: "media",
            asset: "cd".repeat(32),
            preview: "cd".repeat(32),
          },
        };
      }
      if (intent.command.type === "save_emoji") {
        accepted = {
          request_id: intent.request_id,
          result: {
            type: "emoji",
            emoji: {
              id: intent.command.id,
              owner: profile.principal,
              revision: "ab".repeat(32),
              shortcode: intent.command.shortcode,
              asset: intent.command.asset,
              preview: intent.command.asset,
              retired: false,
            },
          },
        };
      }
      value = accepted;
      if (unknown || unknownCommand === intent.command.type) {
        unknown = false;
        unknownCommand = null;
        throw Error("lost response");
      }
    } else if (url.includes("/media/"))
      return new Response(png, { headers: { "content-type": "image/png" } });
    else if (url.endsWith("/profile")) value = profile;
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
    set unknownCommand(v: DirectoryRequest["command"]["type"] | null) {
      unknownCommand = v;
    },
    set token(v: string | null) {
      token = v;
    },
    set profile(v: Model<"UserProfile">) {
      profile = v;
    },
  };
}
test.each(["profile", "emoji"] as const)(
  "%s image uploads use binary transport and retain source bytes after an unknown response",
  async (target) => {
    const f = fixture();
    await f.manager.observe();
    f.unknown = true;
    const bytes = new Uint8Array([1, 2, 3, 4]);
    const source = new File([bytes], "headshot.jpg", {
      type: "image/jpeg",
    });
    await expect(f.manager.upload(source, target)).rejects.toThrow(
      "lost response",
    );
    expect(f.manager.state.pending?.command.type).toBe("upload_image");
    const original = f.intents[0]!.request_id;
    await f.manager.retry();
    expect(f.intents[1]?.request_id).toBe(original);
    for (const intent of f.intents) {
      if (intent.command.type !== "upload_image")
        throw Error("Expected image upload");
      expect(
        Buffer.from(intent.command.bytes.map(Number)).equals(
          Buffer.from(bytes),
        ),
      ).toBe(true);
    }
    expect(
      target === "profile"
        ? f.manager.state.draft.avatar
        : f.manager.state.emojiDraft.asset,
    ).toBe(`blob:${"cd".repeat(32)}`);
    const post = f.fetcher.mock.calls.find(
      ([, init]) => init?.method === "POST",
    )!;
    expect(post[1]?.headers).toMatchObject({
      "content-type": "application/octet-stream",
    });
    expect(f.manager.state.pending).toBeNull();
    f.manager.destroy();
  },
);
test("custom emoji accepts sources larger than two MiB through the normalization transport", async () => {
  const f = fixture();
  await f.manager.observe();
  const bytes = new Uint8Array(2 * 1024 * 1024 + 1);
  bytes.set([1, 2, 3, 4]);
  await f.manager.upload(
    new File([bytes], "large.jpg", { type: "image/jpeg" }),
    "emoji",
  );
  expect(f.manager.state.emojiDraft.asset).toBe(`blob:${"cd".repeat(32)}`);
  const intent = f.intents[0]!;
  if (intent.command.type !== "upload_image")
    throw Error("Expected normalization upload");
  expect(
    Buffer.from(intent.command.bytes.map(Number)).equals(Buffer.from(bytes)),
  ).toBe(true);
  f.manager.destroy();
});
test("emoji selection stays local through refresh and cancel without server writes", async () => {
  const f = fixture();
  await f.manager.observe();
  const file = new File([new Uint8Array([1, 2, 3, 4])], "party.jpg", {
    type: "image/jpeg",
  });
  f.manager.selectEmoji(file);
  const preview = f.manager.state.emojiDraft.preview!;
  await f.manager.observe();
  expect(f.intents).toHaveLength(0);
  expect(f.manager.state.emojiDraft.source).toBe(file);
  expect(f.manager.state.emojiDraft.preview).toBe(preview);
  const revoke = vi.spyOn(URL, "revokeObjectURL");
  f.manager.cancelEmoji();
  expect(revoke).toHaveBeenCalledWith(preview);
  expect(f.manager.state.emojiDraft.source).toBeUndefined();
  expect(f.intents).toHaveLength(0);
  f.manager.destroy();
  revoke.mockRestore();
});
test.each(["upload_image", "save_emoji"] as const)(
  "Save emoji recovers an unknown %s and keeps its original definition intent",
  async (phase) => {
    const f = fixture();
    await f.manager.observe();
    f.manager.selectEmoji(
      new File([new Uint8Array([1, 2, 3, 4])], "party.jpg", {
        type: "image/jpeg",
      }),
    );
    f.unknownCommand = phase;
    await expect(f.manager.saveEmoji("party", "")).rejects.toThrow(
      "lost response",
    );
    const pending = structuredClone(f.manager.state.pending);
    f.manager.state.emojiDraft.shortcode = "changed";
    await f.manager.retry();
    expect(f.manager.state.pending).toBeNull();
    expect(f.manager.state.emojiSave).toBeNull();
    const writes = f.intents.filter(
      (intent) => intent.command.type === "save_emoji",
    );
    expect(writes.at(-1)?.command).toMatchObject({
      shortcode: "party",
      asset: "cd".repeat(32),
    });
    expect(f.intents[phase === "upload_image" ? 1 : 2]).toEqual(pending);
    expect(
      f.intents.filter((intent) => intent.command.type === "upload_image"),
    ).toHaveLength(phase === "upload_image" ? 2 : 1);
    expect(f.manager.state.emojiDraft.source).toBeUndefined();
    f.manager.destroy();
  },
);
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
