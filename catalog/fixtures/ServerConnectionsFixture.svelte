<script lang="ts">
  import { untrack } from "svelte";
  import StoreServerSwitcher from "../../web/src/lib/ui/store-connections/StoreServerSwitcher.svelte";
  import StoreConnectionDialog from "../../web/src/lib/ui/store-connections/StoreConnectionDialog.svelte";
  import type {
    ServerSwitcherOwner,
    ConnectionDialogOwner,
  } from "../../web/src/lib/ui/store-connections/presentation.js";
  import type { ServerConnection } from "../../web/src/lib/connections/owner.svelte.js";
  let {
    compact = false,
    dialogOpen = false,
  }: { compact?: boolean; dialogOpen?: boolean } = $props();
  let selected = $state("local"),
    dialog = $state(untrack(() => dialogOpen)),
    editing = $state<ServerConnection | null>(null),
    message = $state("");
  let records = $state<ServerConnection[]>([
    {
      id: "a".repeat(32),
      name: "Build host",
      origin: "https://build.example",
      installation: "b".repeat(32),
    },
    {
      id: "c".repeat(32),
      name: "Research",
      origin: "https://research.example",
      installation: "d".repeat(32),
    },
  ]);
  const owner: ServerSwitcherOwner & ConnectionDialogOwner = {
    supported: true,
    get selected() {
      return selected;
    },
    get records() {
      return records;
    },
    get dialog() {
      return dialog;
    },
    set dialog(value) {
      dialog = value;
    },
    get editing() {
      return editing;
    },
    busy: false,
    error: "",
    select(id) {
      selected = id;
      message = `Selected ${id === "local" ? "Local" : records.find((c) => c.id === id)?.name}`;
    },
    add() {
      editing = null;
      dialog = true;
    },
    manage(id) {
      editing = records.find((c) => c.id === id) ?? null;
      dialog = true;
    },
    async connect(input) {
      message = `Connected ${input.name}`;
      dialog = false;
    },
    async rename(name) {
      if (editing) editing.name = name;
      dialog = false;
    },
    async signOut() {
      message = "Signed out";
    },
    async remove() {
      records = records.filter((c) => c !== editing);
      dialog = false;
      selected = "local";
    },
  };
</script>

<div
  style={`width:${compact ? "4rem" : "18rem"};background:var(--sidebar);padding:0.5rem;`}
>
  <StoreServerSwitcher {compact} {owner} />
  <p>Help</p>
</div>
<output aria-live="polite">{message}</output>
<StoreConnectionDialog {owner} />
