<script lang="ts">
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const settings = $derived(state.settings);
  const capabilities = $derived(settings.capabilities);
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
  const words = (values: readonly string[]) =>
    values.length
      ? values.map((value) => value.replaceAll("_", " ")).join(", ")
      : "None";
  const featureList = (value: Model<"Capabilities">) =>
    [
      [value.read_api, "Read API"],
      [value.browsing_windows, "Windowed browsing"],
      [value.events, "Live events"],
      [value.search, "Search"],
      [value.index_tasks, "Index tasks"],
    ]
      .filter(([enabled]) => enabled)
      .map(([, label]) => label)
      .join(", ");
</script>

<section class="store-settings__section" aria-labelledby="repository-identity">
  <header><h2 id="repository-identity">Identity and snapshot</h2></header>
  <dl class="store-settings__rows">
    <div>
      <dt>Name</dt>
      <dd>{state.repository.name}</dd>
    </div>
    <div>
      <dt>Repository ID</dt>
      <dd><code>{state.repository.id}</code></dd>
    </div>
    <div>
      <dt>Permission</dt>
      <dd>{state.repository.permission ?? "Unavailable"}</dd>
    </div>
    <div>
      <dt>Selected operation</dt>
      <dd>
        <code
          >{state.route.snapshotOperationId ?? "Choose an operation head"}</code
        >
      </dd>
    </div>
    <div>
      <dt>Operation heads</dt>
      <dd class="store-settings__stack">
        {#each state.operationHeads as head (head)}<code>{head}</code>{/each}
      </dd>
    </div>
    <div>
      <dt>Conflicts</dt>
      <dd>{state.conflictCount ?? "Not loaded"}</dd>
    </div>
    <div>
      <dt>Epoch</dt>
      <dd><code>{settings.snapshot?.epoch ?? "Unavailable"}</code></dd>
    </div>
    <div>
      <dt>Published position</dt>
      <dd>{number(settings.snapshot?.published_seq)}</dd>
    </div>
    <div>
      <dt>Manifest revision</dt>
      <dd>{number(settings.snapshot?.manifest_revision)}</dd>
    </div>
    <div>
      <dt>Code state version</dt>
      <dd>{number(settings.snapshot?.code_state_version)}</dd>
    </div>
  </dl>
</section>

{#if capabilities}
  <section
    class="store-settings__section"
    aria-labelledby="repository-capabilities"
  >
    <header><h2 id="repository-capabilities">Capabilities</h2></header>
    <dl class="store-settings__rows">
      <div>
        <dt>Native profile</dt>
        <dd>{capabilities.native_profile}</dd>
      </div>
      <div>
        <dt>Authentication</dt>
        <dd>{capabilities.authentication.replaceAll("_", " ")}</dd>
      </div>
      <div>
        <dt>Supported APIs</dt>
        <dd>{featureList(capabilities)}</dd>
      </div>
      <div>
        <dt>Typed mutations</dt>
        <dd>{words(capabilities.typed_mutations)}</dd>
      </div>
      <div>
        <dt>Conflict actions</dt>
        <dd>{words(capabilities.conflict_resolution_actions)}</dd>
      </div>
      <div>
        <dt>Mutation request limit</dt>
        <dd>{number(capabilities.mutation_request_bytes)} bytes</dd>
      </div>
      <div>
        <dt>Conflict upload limit</dt>
        <dd>{number(capabilities.conflict_upload_bytes)} bytes</dd>
      </div>
    </dl>
  </section>
  <section class="store-settings__section" aria-labelledby="repository-limits">
    <header><h2 id="repository-limits">Read and event limits</h2></header>
    <dl class="store-settings__rows">
      <div>
        <dt>Requests / workers</dt>
        <dd>
          {number(capabilities.limits.requests)} / {number(
            capabilities.limits.workers,
          )}
        </dd>
      </div>
      <div>
        <dt>Deadline</dt>
        <dd>{number(capabilities.limits.deadline_millis)} ms</dd>
      </div>
      <div>
        <dt>Page / index commits</dt>
        <dd>
          {number(capabilities.limits.page)} / {number(
            capabilities.limits.index_commits,
          )}
        </dd>
      </div>
      <div>
        <dt>JSON / file limits</dt>
        <dd>
          {number(capabilities.limits.json_bytes)} / {number(
            capabilities.limits.file_bytes,
          )} bytes
        </dd>
      </div>
      <div>
        <dt>Event streams / replay</dt>
        <dd>
          {number(capabilities.event_limits.streams)} / {number(
            capabilities.event_limits.replay_window,
          )}
        </dd>
      </div>
      <div>
        <dt>Event page / polling</dt>
        <dd>
          {number(capabilities.event_limits.page)} / {number(
            capabilities.event_limits.poll_millis,
          )} ms
        </dd>
      </div>
    </dl>
  </section>
{/if}
