<script lang="ts">
  import StoreLocalFolders from "./StoreLocalFolders.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreWorkspaceSettings from "./StoreWorkspaceSettings.svelte";
  import StoreGraphIndexSettings from "./StoreGraphIndexSettings.svelte";
  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const settings = $derived(state.settings);
  const capabilities = $derived(settings.capabilities);
  const native = $derived(
    settings.snapshot && "epoch" in settings.snapshot
      ? settings.snapshot
      : null,
  );
  const local = $derived(
    settings.snapshot && "profile" in settings.snapshot
      ? settings.snapshot
      : null,
  );
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
  const words = (values: readonly string[]) =>
    values.length
      ? values.map((value) => value.replaceAll("_", " ")).join(", ")
      : "None";
  const featureList = (
    value: import("../../controller/repository-profile.js").RepositoryCapabilities,
  ) =>
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

<StoreSettingsPage
  section="repository"
  value="general"
  tabs={[
    { value: "general", label: "General" },
    { value: "folders", label: "Local folders" },
    { value: "advanced", label: "Advanced" },
  ]}
>
  <Tabs.Content value="general">
    <section
      class="store-settings__section"
      aria-labelledby="repository-identity"
    >
      <header><h2 id="repository-identity">Identity and snapshot</h2></header>
      <dl class="store-settings__rows">
        <div>
          <dt>Name</dt>
          <dd>{state.repository.name}</dd>
        </div>
        <div>
          <dt>Repository ID</dt>
          <dd>
            <CopyableValue label="Repository ID" value={state.repository.id} />
          </dd>
        </div>
        <div>
          <dt>Permission</dt>
          <dd>{state.repository.permission ?? "Unavailable"}</dd>
        </div>
        <div>
          <dt>Selected operation</dt>
          <dd>
            <CopyableValue
              label="Selected operation"
              value={state.route.snapshotOperationId}
              emptyLabel="Choose an operation head"
            />
          </dd>
        </div>
        <div>
          <dt>Operation heads</dt>
          <dd class="store-settings__stack">
            {#each state.operationHeads as head, index (head)}<CopyableValue
                label={`Operation head ${index + 1}`}
                value={head}
              />{/each}
          </dd>
        </div>
        <div>
          <dt>Conflicts</dt>
          <dd>{state.conflictCount ?? "Not loaded"}</dd>
        </div>
        {#if local}<div>
            <dt>Profile</dt>
            <dd>Git-backed JJ (SHA-1)</dd>
          </div>
          <div>
            <dt>Folder generation</dt>
            <dd>{number(local.generation)}</dd>
          </div>
          <div>
            <dt>Companion metadata</dt>
            <dd>
              <CopyableValue
                label="Companion metadata"
                value={local.metadata_commit}
              />
            </dd>
          </div>{/if}
        {#if native}<div>
            <dt>Epoch</dt>
            <dd>
              <CopyableValue
                label="Epoch"
                value={native?.epoch}
                emptyLabel="Unavailable"
              />
            </dd>
          </div>
          <div>
            <dt>Published position</dt>
            <dd>{number(native?.published_seq)}</dd>
          </div>
          <div>
            <dt>Manifest revision</dt>
            <dd>{number(native?.manifest_revision)}</dd>
          </div>
          <div>
            <dt>Code state version</dt>
            <dd>{number(native?.code_state_version)}</dd>
          </div>{/if}
      </dl>
    </section>
  </Tabs.Content>
  <Tabs.Content value="folders">
    {#if state.repository.profile === "git_sha1_jj045"}<StoreLocalFolders
        {controller}
        repository={state.repository.id}
      />{:else}<StoreWorkspaceSettings
        {settings}
        supported={Boolean(settings.capabilities?.local_workspace_links)}
        onRefresh={controller.actions.refreshWorkspaceLinks}
        onPath={controller.actions.setWorkspacePath}
        onLink={controller.actions.linkWorkspace}
        pickerSupported={Boolean(settings.capabilities?.local_folder_picker)}
        onBrowse={controller.actions.pickWorkspaceFolder}
      />{/if}
  </Tabs.Content>
  <Tabs.Content value="advanced">
    <StoreGraphIndexSettings {controller} />

    {#if capabilities}
      <section
        class="store-settings__section"
        aria-labelledby="repository-capabilities"
      >
        <header><h2 id="repository-capabilities">Capabilities</h2></header>
        <dl class="store-settings__rows">
          <div>
            <dt>Native profile</dt>
            <dd>
              {"native_profile" in capabilities
                ? capabilities.native_profile
                : capabilities.profile}
            </dd>
          </div>
          <div>
            <dt>Authentication</dt>
            <dd>
              {"authentication" in capabilities
                ? capabilities.authentication.replaceAll("_", " ")
                : "Local trusted"}
            </dd>
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
          {#if "mutation_request_bytes" in capabilities}<div>
              <dt>Mutation request limit</dt>
              <dd>{number(capabilities.mutation_request_bytes)} bytes</dd>
            </div>{/if}
          <div>
            <dt>Conflict upload limit</dt>
            <dd>{number(capabilities.conflict_upload_bytes)} bytes</dd>
          </div>
        </dl>
      </section>
      {#if "limits" in capabilities}<section
          class="store-settings__section"
          aria-labelledby="repository-limits"
        >
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
        </section>{/if}
    {/if}
  </Tabs.Content>
</StoreSettingsPage>
