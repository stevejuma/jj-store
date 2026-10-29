<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onMount, untrack } from "svelte";
  import StoreSettingsPage from "../../web/src/lib/ui/store-settings/StoreSettingsPage.svelte";
  import StoreProfileSettings from "../../web/src/lib/ui/store-settings/StoreProfileSettings.svelte";
  import StoreCustomEmojiSettings from "../../web/src/lib/ui/store-settings/StoreCustomEmojiSettings.svelte";
  import { createDirectory } from "../../web/src/lib/controller/directory.svelte.js";
  import type { DirectoryManagement } from "../../web/src/lib/controller/directory.svelte.js";
  type Profile = NonNullable<DirectoryManagement["state"]["profile"]>;
  type Emoji = DirectoryManagement["state"]["emojis"][number];
  import type { DirectoryRequest } from "../../web/src/lib/api/directory-client.js";
  let {
    section = "profile",
    unknown = false,
    supported = true,
    withAvatar = false,
  }: {
    section?: "profile" | "custom-emoji";
    unknown?: boolean;
    supported?: boolean;
    withAvatar?: boolean;
  } = $props();
  let profile: Profile = untrack(() => ({
    principal: "01".repeat(16),
    revision: null,
    details: {
      display_name: "Local admin",
      description: "A profile",
      email: null,
      avatar: withAvatar
        ? `data:image/svg+xml,${encodeURIComponent('<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><rect width="256" height="256" rx="128" fill="#6e56cf"/><text x="128" y="166" font-size="128" text-anchor="middle">😀</text></svg>')}`
        : null,
    },
  }));
  let emojis: Emoji[] = [];
  const management = createDirectory({
    available: () => supported,
    token: () => null,
    fetcher: async (path, init) => {
      const url = String(path);
      let value: unknown = null;
      if (init?.method === "POST") {
        const request = JSON.parse(String(init.body)) as DirectoryRequest;
        if (unknown) throw Error("lost response");
        if (request.command.type === "save_profile") {
          profile = {
            ...profile,
            revision: "ab".repeat(32),
            details: {
              ...request.command.details,
              email: request.command.details.email ?? null,
              avatar: request.command.details.avatar ?? null,
            },
          };
          value = {
            request_id: request.request_id,
            result: { type: "profile", profile },
          };
        }
        if (request.command.type === "save_emoji") {
          const emoji = {
            id: request.command.id,
            owner: profile.principal,
            revision: "ab".repeat(32),
            shortcode: request.command.shortcode,
            asset: request.command.asset,
            preview: request.command.asset,
            retired: false,
          };
          emojis.push(emoji);
          value = {
            request_id: request.request_id,
            result: { type: "emoji", emoji },
          };
        }
        if (request.command.type === "retire_emoji") {
          const id = request.command.id;
          const emoji = emojis.find((e) => e.id === id)!;
          emoji.retired = true;
          value = {
            request_id: request.request_id,
            result: { type: "emoji", emoji },
          };
        }
      } else if (url.endsWith("/profile")) value = profile;
      else if (url.endsWith("/emojis")) value = emojis;
      else if (url.includes("/profiles?")) value = [];
      return Response.json(value);
    },
  });
  onMount(() => {
    void management.observe();
    return () => management.destroy();
  });
  function preview() {
    const digest = "cd".repeat(32);
    management.state.urls[digest] =
      "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGP4z8DwHwAFgAI/ScLttAAAAABJRU5ErkJggg==";
    management.state.emojiDraft = {
      shortcode: "",
      asset: `blob:${digest}`,
      filename: "party.png",
      selected: undefined,
    };
  }
</script>

<StoreSettingsPage {section}>
  {#if section === "profile"}<StoreProfileSettings
      {management}
    />{:else}<FixtureControls floating={false}
      ><button onclick={preview}>Select fixture image</button></FixtureControls
    ><StoreCustomEmojiSettings {management} />{/if}
</StoreSettingsPage>
