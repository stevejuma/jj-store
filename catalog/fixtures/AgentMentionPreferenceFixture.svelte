<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onDestroy, untrack } from "svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { provideAgentExecution } from "../../web/src/lib/ui/store-agent-execution/context.js";
  import CommunityChatComposer from "../../web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import type { CommunityMemberModel } from "../../web/src/lib/community/workspace-model.js";
  const saved = new AgentSettingsFixture();
  let requests = $state<string[]>([]),
    loseResponse = true;
  const manager = new AgentExecution(
    () => true,
    async (url, init) => {
      if (init?.body) requests.push(String(init.body));
      if (init?.body && loseResponse) {
        loseResponse = false;
        saved.respond(String(url), init);
        throw new TypeError("Preference response lost");
      }
      // Replay of the fixture's exact unknown request returns its accepted receipt.
      return init?.body
        ? Response.json(saved.value)
        : saved.respond(String(url), init)!;
    },
  );
  provideAgentExecution(() => manager);
  const scope = crypto.randomUUID();
  const members: CommunityMemberModel[] = [
    {
      id: "helper",
      acpAgentId: "helper",
      name: "Helper",
      handle: "helper",
      initials: "H",
      kind: "agent",
    },
  ];
  let first = $state("Draft stays here"),
    second = $state("");
  $effect(() => {
    untrack(() => void manager.settings.refresh());
  });
  onDestroy(() => manager.destroy());
</script>

<section aria-label="First conversation">
  <CommunityChatComposer
    bind:value={first}
    audienceKey={`${scope}-first`}
    {members}
  />
</section>
<section aria-label="Second conversation">
  <CommunityChatComposer
    bind:value={second}
    audienceKey={`${scope}-second`}
    {members}
  />
</section>
<FixtureControls floating={false}
  ><button
    onclick={() => {
      saved.value = {
        ...saved.value,
        revision: Number(saved.value.revision) + 1,
        automatically_mention: false,
      };
      void manager.settings.refresh();
    }}>External preference off</button
  ></FixtureControls
>
<output aria-label="Preference requests">{JSON.stringify(requests)}</output>
