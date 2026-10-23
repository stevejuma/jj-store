<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Fixture from "../../fixtures/ServerConnectionsFixture.svelte";
  import {
    retainedOwners,
    guardedOwners,
    restoredOwners,
  } from "../../fixtures/server-connections/owner.js";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-connections/StoreServerSwitcher.svelte",
      },
    },
    title: "Store/Server connection owners",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
</script>

<Story name="Retained controllers drafts and history" play={retainedOwners} />
<Story name="Pending guards and protected sign-out" play={guardedOwners} />
<Story name="Restart and explicit links" play={restoredOwners} />
