<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreWorkspacePicker from "../store-application/StoreWorkspacePicker.svelte";
  import StoreSettingsNotice from "../store-settings/StoreSettingsNotice.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const query = $derived(controller.historyQuery);
  const operations = $derived(controller.state.route.page === "oplog");
  const id = $props.id();
</script>

{#if query}
  <section
    class="store-history-query__context"
    aria-label="Query context and syntax"
  >
    {#if !operations}
      <Field.Group class="store-work__fields community-settings__option-group">
        <StoreWorkField
          label="Workspace for @"
          description="Resolve @ in the selected workspace at the displayed operation."
        >
          <StoreWorkspacePicker
            {controller}
            label="Workspace for @"
            emptyLabel="Choose workspace"
            fullWidth
          />
        </StoreWorkField>
        <StoreWorkField
          label="Author email for mine()"
          id={`${id}-email`}
          description="Used for this session only."
        >
          <Input
            id={`${id}-email`}
            type="email"
            value={query.email}
            oninput={(event) => query.setEmail(event.currentTarget.value)}
          />
        </StoreWorkField>
      </Field.Group>
    {/if}
    <StoreSettingsNotice>
      {#if operations}
        Combine fields with spaces or AND. Text matches descriptions. Use
        description, user, host, workspace, id, snapshot, after, before, or
        from. Dates use YYYY-MM-DD in UTC; after includes the day and before
        excludes it. For example: <code
          >from:@-- description:"rebase branch" snapshot:false</code
        >.
      {:else}
        JJ {query.info?.jj_version ?? "0.45.1"} revsets and standard aliases. Try
        <code>mutable()</code>, <code>::@</code> or
        <code>description("fix")</code>. Custom aliases are unavailable. Email
        is kept for this session.
      {/if}
    </StoreSettingsNotice>
    {#if !operations && query.info?.unsupported.length}<StoreSettingsNotice
        >Unavailable for this repository: {query.info.unsupported.join(
          ", ",
        )}.</StoreSettingsNotice
      >{/if}
    <StoreSettingsNotice
      >Queries use the displayed operation snapshot. Enter adds a line;
      Ctrl/⌘+Enter or Search runs the query. Enter accepts an active completion.
      Escape closes completions. Invalid drafts keep the previous results.</StoreSettingsNotice
    >
  </section>
{/if}
