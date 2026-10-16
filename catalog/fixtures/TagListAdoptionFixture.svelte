<script lang="ts">
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ForumTopicList from "../../web/src/lib/ui/forum-topic-list/ForumTopicList.svelte";
  import CommunityDetails from "../../web/src/lib/ui/community-details/CommunityDetails.svelte";
  import StoreMutationCommitPicker from "../../web/src/lib/ui/store-mutations/StoreMutationCommitPicker.svelte";
  import { NOW, revisions } from "./repository.js";
  let width = $state(480);
  let extra = $state(false);
  let opened = $state(0);
  let target = $state(revisions[0]!.eventId);
  const labels = $derived([
    "priority",
    "native objects",
    "retained source labels",
    "review",
    "ready",
    ...(extra ? ["new live tag"] : []),
  ]);
  const choices = $derived(
    revisions.map((revision) => ({
      ...revision,
      bookmarks: [
        ...labels,
        ...Array.from(
          { length: 8 },
          (_, index) => `additional bookmark ${index + 1}`,
        ),
      ].map((name) => ({
        name,
        added: true,
        removed: false,
        conflicted: false,
      })),
      workspaces: ["default"],
    })),
  );
</script>

<div class="tag-adoption">
  <label for="tag-width">Container width</label>
  <Input id="tag-width" type="number" min={240} max={1200} bind:value={width} />
  <Button variant="outline" onclick={() => (extra = !extra)}
    >Toggle live tag</Button
  >
  <output aria-label="Threads opened">{opened}</output>
  <div class="tag-adoption__surface" style:width="{width}px">
    <h2>Thread tags</h2>
    <ForumTopicList
      members={[]}
      topics={[
        {
          id: "review",
          title: "Review the responsive tags",
          excerpt: "All tags retain their original order.",
          authorId: "author",
          tags: labels,
          replyCount: 0,
          participantIds: [],
          lastActivityLabel: "just now",
        },
      ]}
      onOpenTopic={() => (opened += 1)}
    />
    <h2>Commit tags</h2>
    <StoreMutationCommitPicker
      label="Choose tagged commit"
      bind:target
      revisions={choices}
      now={NOW}
      onChange={() => {}}
    />
    <h2>Room facts</h2>
    <CommunityDetails
      room={{ id: "room", name: "Review", kind: "channel", private: true }}
      details={{
        description: "Shared tag presentation with application-owned data.",
        memberIds: [],
      }}
      members={[]}
    />
  </div>
</div>

<style>
  .tag-adoption {
    display: grid;
    gap: 0.75rem;
    max-width: 100%;
  }
  .tag-adoption__surface {
    max-width: 100%;
    min-width: 0;
  }
  h2 {
    font-size: 1rem;
    margin-block: 1rem 0.5rem;
  }
</style>
