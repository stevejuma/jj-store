<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Fixture from "../../fixtures/StoreRepositoryApplicationFixture.svelte";
  import { appearanceToolDiscussionPlay } from "./appearance-tool-discussion-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Appearance discussions",
    component: Fixture,
    tags: ["visual-pending", "application"],
    args: { initiallyCollapsed: true },
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      jjStoreShell: true,
      docs: { story: { inline: false, iframeHeight: 700 } },
    },
  });
</script>

<Story
  name="Focus and Split preserve tool discussion drafts"
  args={{ initialPage: "agents" }}
  play={appearanceToolDiscussionPlay}
/>
