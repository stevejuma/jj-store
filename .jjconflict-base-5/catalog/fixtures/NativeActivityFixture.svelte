<script lang="ts">
  import { untrack } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import StoreRepositoryActivity from "../../web/src/lib/ui/store-collaboration/StoreRepositoryActivity.svelte";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import { activityCommit } from "./native-activity.js";
  const controller = untrack(() => createInMemoryStoreController("projects"));
  const template = untrack(() => activityCommit("ab".repeat(16)));
  let discoveries = $state(0),
    sessions = $state(0),
    continuations = $state(0);
  let loading = $state(false);
  function make(index: number) {
    let discovery: Promise<typeof template.agents> | undefined;
    let detail: Promise<typeof template.agents> | undefined;
    return {
      ...template,
      revision: {
        ...template.revision,
        eventId: index.toString(16).padStart(128, "0"),
        message: `Native activity ${index}`,
        createdAt: template.revision.createdAt - index,
      },
      agents: {
        sessions: [],
        error: "",
        discover: () =>
          (discovery ??= (async () => {
            discoveries++;
            return {
              sessions: [],
              error: "",
              relatedSessionIds: [
                template.agents.sessions[0]!.session.session_id,
              ],
              load: () =>
                (detail ??= (async () => {
                  sessions++;
                  return template.agents;
                })()),
            };
          })()),
      },
    };
  }
  let rows = $state(Array.from({ length: 120 }, (_, index) => make(index)));
  async function more() {
    if (loading || rows.length >= 160) return;
    loading = true;
    continuations++;
    await new Promise((resolve) => setTimeout(resolve, 20));
    rows = [
      ...rows,
      ...Array.from({ length: 40 }, (_, index) => make(rows.length + index)),
    ];
    loading = false;
  }
</script>

<section
  aria-label="Virtual activity fixture"
  data-discoveries={discoveries}
  data-sessions={sessions}
  data-continuations={continuations}
  data-loaded={rows.length}
>
  <ScrollArea type="hover" class="activity-fixture-scroll">
    <StoreRepositoryActivity
      commits={rows}
      {controller}
      {loading}
      hasMore={rows.length < 160}
      onLoadMore={() => void more()}
    />
  </ScrollArea>
</section>

<style>
  section {
    height: 420px;
  }
  :global(.activity-fixture-scroll) {
    height: 100%;
  }
</style>
