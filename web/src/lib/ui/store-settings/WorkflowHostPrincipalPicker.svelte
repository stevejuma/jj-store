<script lang="ts">
  import { useProfileDirectory } from "../../profiles/context.svelte.js";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import User from "@lucide/svelte/icons/user-round";
  let {
    id,
    label,
    values,
    authors = [],
    multiple = true,
    disabled = false,
    onChange,
  }: {
    id: string;
    label: string;
    values: string[];
    authors?: readonly { id: string; name: string }[];
    multiple?: boolean;
    disabled?: boolean;
    onChange(values: string[]): void;
  } = $props();
  const profiles = useProfileDirectory();
  let query = $state(""),
    error = $state("");
  const choices = $derived([
    ...new Map(
      [
        ...authors.filter((a) => /^[0-9a-f]{32}$/.test(a.id)),
        ...values.map((id) => ({ id, name: id })),
        ...(/^[0-9a-f]{32}$/.test(query.trim())
          ? [{ id: query.trim(), name: query.trim() }]
          : []),
      ].map((a) => [
        a.id,
        {
          ...a,
          name:
            profiles?.directory?.state.profiles[a.id]?.display_name ||
            authors.find((p) => p.id === a.id)?.name ||
            a.name,
          description: a.id,
        },
      ]),
    ).values(),
  ]);
  async function hydrate() {
    const directory = profiles?.directory;
    if (!directory?.available()) return;
    try {
      await directory.loadProfiles(
        choices.map((a) => a.id),
        true,
      );
      error = "";
    } catch {
      error =
        "Principal profiles could not be loaded. Your selection is retained.";
    }
  }
</script>

<StoreNotice
  noticeKey={`workflow-host-profiles-${id}`}
  title="Profiles unavailable"
  message={error}
  action={{ id: "retry", label: "Retry", run: hydrate }}
/>
<StoreRepositoryPicker
  {id}
  {label}
  repositories={choices}
  {multiple}
  {disabled}
  limit={64}
  allowEmpty
  value={values[0] ?? ""}
  onChange={(v) => onChange(v ? [v] : [])}
  bind:values={() => values, onChange}
  selectionLabel="principals"
  emptyLabel={multiple ? "Choose principals" : "Choose principal"}
  searchLabel="Find an author or enter a full principal ID"
  groupLabel="Principals"
  emptyResult="No known principals. Enter a full 32-character principal ID."
  keepItemIcon
  onSearch={(v) => (query = v)}
  onOpen={() => void hydrate()}
>
  {#snippet triggerIcon()}<User aria-hidden="true" />{/snippet}
  {#snippet itemIcon(id)}{@const name =
      choices.find((a) => a.id === id)?.name || id}
    <CommunityMemberAvatar
      member={{
        id,
        name,
        handle: id,
        initials: name.slice(0, 2).toUpperCase(),
      }}
      size="inline"
      decorative
    />
  {/snippet}
</StoreRepositoryPicker>
