<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import CaptureSettingsFixture from "../../fixtures/CaptureSettingsFixture.svelte";
  import { sourceRepositoryCommand } from "./capture-repository-command-play.js";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreAgentSettings.svelte",
      },
    },
    title: "JJ Store/Settings/Capture sources",
    component: CaptureSettingsFixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story
  name="Source repository command combines repositories statuses and unknown bindings"
  args={{ multipleHarnesses: true, multipleRepositories: true }}
  play={sourceRepositoryCommand}
/>
