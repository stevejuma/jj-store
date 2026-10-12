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
    selectedServer = "local",
    remoteCount = 2,
    busy = false,
    connectFails = false,
    requireSignIn = false,
    blockSwitching = false,
  }: {
    compact?: boolean;
    dialogOpen?: boolean;
    selectedServer?: string;
    remoteCount?: number;
    busy?: boolean;
    connectFails?: boolean;
    requireSignIn?: boolean;
    blockSwitching?: boolean;
  } = $props();
  let selected = $state(untrack(() => selectedServer)),
    dialog = $state(untrack(() => dialogOpen)),
    editing = $state<ServerConnection | null>(null),
    message = $state(""),
    error = $state("");
  let records = $state<ServerConnection[]>(
    untrack(() =>
      [
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
        {
          id: "e".repeat(32),
          name: "Archive",
          origin: "https://archive.example",
          installation: "f".repeat(32),
        },
        {
          id: "1".repeat(32),
          name: "Shared compute server with a very long display name",
          origin: "https://compute.example",
          installation: "2".repeat(32),
        },
      ].slice(0, remoteCount),
    ),
  );
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
    get busy() {
      return busy;
    },
    get error() {
      return error;
    },
    select(id) {
      if (blockSwitching) return;
      selected = id;
      message = `Selected ${id === "local" ? "Local" : records.find((c) => c.id === id)?.name}`;
      if (requireSignIn && id !== "local") {
        editing = records.find((c) => c.id === id) ?? null;
        dialog = true;
      }
    },
    add() {
      error = "";
      editing = null;
      dialog = true;
    },
    manage(id) {
      error = "";
      editing = records.find((c) => c.id === id) ?? null;
      dialog = true;
    },
    async connect(input) {
      if (connectFails) {
        error = "authentication_failed";
        return;
      }
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
  style={`width:${compact ? "4rem" : "min(18rem, 100%)"};background:var(--sidebar);padding:0.5rem;`}
>
  <StoreServerSwitcher {compact} {owner} />
  <p>Help</p>
</div>
<output aria-live="polite">{message}</output>
<StoreConnectionDialog {owner} />
