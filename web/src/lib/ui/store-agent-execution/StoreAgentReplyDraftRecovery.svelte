<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { ActivityRow } from "./presentation.js";
  let {
    manager,
    source,
    rows,
    onRestore,
  }: {
    manager: AgentExecution;
    source: string;
    rows: ActivityRow[];
    onRestore: (row: string) => void;
  } = $props();
  const session = $derived(manager.session!);
  const discussions = $derived(manager.discussions);
  const key = $derived(`${session.id}:${source}`);
  const threads = $derived(discussions.reader(session.repository));
  const options = $derived(
    rows
      .filter(
        (row) =>
          (row.kind === "user" || row.kind === "assistant") &&
          discussions.message(session.id, row.id),
      )
      .map((row, index) => ({
        value: row.id,
        label: `${index + 1}. ${row.kind === "user" ? "You" : session.title}: ${row.text.slice(0, 100)}`,
        disabled:
          !!discussions.drafts[`${session.id}:${row.id}`] ||
          discussions.awaiting(session.id, row.id),
      })),
  );
  let target = $state("");
  const blocked = $derived(
    threads.busy ||
      !!threads.pending ||
      discussions.awaiting(session.id, source) ||
      !!discussions.errors[session.id],
  );
  const selected = $derived(options.find((option) => option.value === target));
  function restore() {
    if (blocked || !selected || selected.disabled || !discussions.drafts[key])
      return;
    // Only move an editable draft. Saving remains the original message's exact
    // collaboration action; unresolved request identities are never moved.
    discussions.drafts[`${session.id}:${target}`] = discussions.drafts[key];
    discussions.drafts[key] = "";
    onRestore(target);
  }
  const id = $props.id();
</script>

<section aria-label="Retained reply draft">
  <Field.Group>
    <Field.Field>
      <Field.Content>
        <Field.Title>Retained reply draft</Field.Title>
        <Field.Description>
          This draft was attached to replayed history. Choose the original
          published message to restore it. Messages with another draft are
          unavailable until that draft is saved or cleared.
        </Field.Description>
      </Field.Content>
      <p>{discussions.drafts[key]}</p>
    </Field.Field>
    <StoreWorkField label="Reply to" {id} disabled={blocked}>
      <Select.Root type="single" bind:value={target} disabled={blocked}>
        <Select.Trigger {id} class="w-full">
          <span class="truncate"
            >{selected?.label ?? "Choose original message"}</span
          >
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            <Select.Label>Published messages</Select.Label>
            {#each options as option (option.value)}
              <Select.Item {...option}>{option.label}</Select.Item>
            {/each}
          </Select.Group>
        </Select.Content>
      </Select.Root>
    </StoreWorkField>
    <Field.Field>
      <Button
        variant="secondary"
        disabled={blocked || !selected || selected.disabled}
        onclick={restore}>Restore reply draft</Button
      >
    </Field.Field>
  </Field.Group>
</section>
