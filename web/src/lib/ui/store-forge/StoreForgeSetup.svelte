<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  import { Input } from "@lapismd/design-core/shadcn/input";
  import type { ForgeConnection } from "../../controller/forge/types.js";
  let provider = $state<ForgeConnection["provider"]>("github");
  let api = $state("https://api.github.com");
  let repository = $state("");
  let source = $state("");
  let remote = $state("origin");
  let target = $state("main");
  let targetRemote = $state("origin");
  let native = $state(false);
  function configure(event: SubmitEvent) {
    event.preventDefault();
    void controller.configure({
      provider,
      api_url: api,
      repository,
      source_repository: source || repository,
      remote,
      target_remote: targetRemote,
      target,
      native_stacks: native,
    });
  }
</script>

<form onsubmit={configure}>
  <h3>Connect a forge</h3>
  <label
    >Forge<select
      bind:value={provider}
      onchange={() =>
        (api =
          provider === "github"
            ? "https://api.github.com"
            : provider === "gitlab"
              ? "https://gitlab.com/api/v4"
              : "https://codeberg.org/api/v1")}
      ><option value="github">GitHub</option><option value="gitlab"
        >GitLab</option
      ><option value="forgejo">Forgejo</option></select
    ></label
  >
  <label>API URL<Input bind:value={api} required type="url" /></label>
  <label
    >Target repository<Input
      bind:value={repository}
      required
      placeholder="owner/project"
    /></label
  >
  <label
    >Source repository<Input
      bind:value={source}
      placeholder="Same as target, or your fork"
    /></label
  >
  <label>Push remote<Input bind:value={remote} required /></label>
  <label>Target remote<Input bind:value={targetRemote} required /></label>
  <label>Target branch<Input bind:value={target} required /></label>
  <label
    >Access token<Input
      type="password"
      autocomplete="off"
      bind:value={controller.token}
    /></label
  >
  <p>
    Token stays in this session. The backend can also use JJ_STORE_FORGE_TOKEN.
  </p>
  {#if provider === "github"}<label
      ><input type="checkbox" bind:checked={native} /> Use native GitHub stacks for
      eligible chains</label
    >{/if}
  <Button type="submit" disabled={blocked}>Connect forge</Button>
</form>
