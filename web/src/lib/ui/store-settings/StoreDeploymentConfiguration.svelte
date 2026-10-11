<script lang="ts">
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import ServerIcon from "@lucide/svelte/icons/server";
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import SettingsIcon from "@lucide/svelte/icons/settings-2";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    section,
  }: {
    management?: ServerSettingsManagement;
    section: "service" | "storage";
  } = $props();
  const paths = new Set([
    "state_dir",
    "operator_policy",
    "web_root",
    "access.catalog",
    "access.installation",
    "access.rpc_socket",
    "tls.certificate",
    "tls.private_key",
    "storage.volume",
    "storage.runtime_executable",
    "storage.installation",
    "storage.credentials",
    "storage.ownership",
  ]);
  const labels: Record<string, string> = {
    bind: "Listen address",
    public_url: "Public URL",
    state_dir: "State directory",
    operator_policy: "Operator policy",
    web_root: "Web assets",
    format: "Configuration format",
    "access.catalog": "Repository catalog",
    "access.installation": "Installation identity",
    "access.rpc_socket": "Native RPC socket",
    "storage.runtime_executable": "Runtime executable",
  };
  function groupFor(name: string) {
    if (section === "service") {
      if (name.startsWith("access.") || name === "operator_policy")
        return "access";
      if (name === "state_dir" || name === "web_root" || name === "format")
        return "installation";
      return "launch";
    }
    if (name.startsWith("backup")) return "backup";
    if (
      name.startsWith("storage.credentials") ||
      name === "storage.ownership" ||
      name === "storage.installation"
    )
      return "custody";
    if (paths.has(name)) return "paths";
    return "storage";
  }
  const groups = $derived(
    (section === "service"
      ? [
          { id: "launch", title: "Server and network", Icon: NetworkIcon },
          { id: "installation", title: "Installation files", Icon: FolderIcon },
          { id: "access", title: "Identity and access", Icon: ShieldIcon },
        ]
      : [
          { id: "storage", title: "Storage connection", Icon: DatabaseIcon },
          { id: "paths", title: "Runtime and data", Icon: FolderIcon },
          {
            id: "custody",
            title: "Storage identity and credentials",
            Icon: ShieldIcon,
          },
          { id: "backup", title: "Backup inventory", Icon: DatabaseIcon },
        ]
    )
      .map((group) => {
        const items =
          management?.state.value?.inventory.filter(
            (item) =>
              item.section === section && groupFor(item.name) === group.id,
          ) ?? [];
        return {
          ...group,
          items,
          description: [...new Set(items.map((item) => item.reason))].join(" "),
        };
      })
      .filter((group) => group.items.length),
  );
  function label(name: string) {
    const text = name
      .replace(/^(access|storage)\./, "")
      .replaceAll("_", " ")
      .replaceAll(".", " / ");
    return labels[name] ?? text.charAt(0).toUpperCase() + text.slice(1);
  }
</script>

<StoreSettingsCard
  id={`deployment-${section}`}
  title={section === "service"
    ? "Launch and installation configuration"
    : "Authority storage configuration"}
  layout="page"
  Icon={ServerIcon}
  description="These values are managed by setup and launch. Operational limits and Protection are under Local service; Agents are global. Search and Sync have editors in repository settings."
>
  {#if management?.available() && management.state.value}
    {#each groups as group (group.id)}
      <StoreSettingsCard
        id={`deployment-${section}-${group.id}`}
        title={group.title}
        Icon={group.Icon}
        description={group.description}
      >
        <dl class="store-settings__rows">
          {#each group.items as item (item.name)}
            {@const path = paths.has(item.name)}
            <StoreReadOnlyRow
              label={label(item.name)}
              copyLabel={label(item.name)}
              value={item.saved}
              copyable={(path || /(?:endpoint|public_url)$/.test(item.name)) &&
                !["Not configured", "None", ""].includes(item.saved)}
              Icon={path
                ? FolderIcon
                : /(?:address|bind|url|endpoint|socket)/.test(item.name)
                  ? NetworkIcon
                  : /(?:mode|permission|credential|tls|insecure)/.test(
                        item.name,
                      )
                    ? ShieldIcon
                    : SettingsIcon}
              description={item.saved !== item.effective
                ? `Running: ${item.effective}`
                : undefined}
            />
          {/each}
        </dl>
      </StoreSettingsCard>
    {/each}
  {:else if management?.available() && !management.state.error}<StoreSettingsSkeleton
      heading={false}
      label="Loading configuration inventory"
      cards={globalSettingsSkeletonCards(
        "service",
        section === "service" ? "launch" : "storage",
      )}
    />
  {:else}<p>
      Configuration inventory requires a local service with administrator
      access.
    </p>{/if}
</StoreSettingsCard>
