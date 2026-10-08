<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentHeaderActions from "../store-agent-execution/StoreAgentHeaderActions.svelte";
  import StoreWorkLayoutControls from "./StoreWorkLayoutControls.svelte";
  import StoreProjectsAddMenu from "./StoreProjectsAddMenu.svelte";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import LinkIcon from "@lucide/svelte/icons/link";
  import type { Model } from "../../api/client.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type { WorkController } from "../../controller/work-types.js";
  import type { ProjectsTab } from "../../controller/projects-hub-types.js";
  let {
    controller,
    project,
    work,
    writable,
    canCreate,
    busy,
    tab,
    layout = $bindable("list"),
    linkOpen = $bindable(false),
    onOpen,
  }: {
    controller: StoreController;
    project?: Model<"ProjectV1"> | null;
    work?: WorkController | null;
    writable: boolean;
    canCreate: boolean;
    busy: boolean;
    tab: ProjectsTab;
    layout?: "grid" | "list";
    linkOpen?: boolean;
    onOpen: (mode: "createTask" | "createProject" | "editProject") => void;
  } = $props();
</script>

<div class="community-projects__surface-actions">
  <StoreAgentHeaderActions
    manager={controller.execution}
    showManage={tab === "agents"}
    context={{
      text: project
        ? `Project: ${project.title}\nProject ID: ${project.id}\n${project.body}`
        : undefined,
    }}
  />
  {#if tab !== "activity" && tab !== "agents"}<StoreWorkLayoutControls
      bind:value={layout}
    />{/if}
  <Button
    variant="ghost"
    size="icon-sm"
    aria-label="Refresh projects"
    title="Refresh projects"
    disabled={busy}
    onclick={() => controller.actions.refresh()}
    ><RefreshIcon aria-hidden="true" /></Button
  >
  {#if canCreate && !project && (tab === "projects" || tab === "tasks")}<Button
      variant="ghost"
      size="icon-sm"
      aria-label={tab === "tasks" ? "New task" : "New project"}
      title={tab === "tasks" ? "New task" : "New project"}
      disabled={busy}
      onclick={() => onOpen(tab === "tasks" ? "createTask" : "createProject")}
      ><PlusIcon aria-hidden="true" /></Button
    >{/if}
  {#if project && writable}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Edit project"
      title="Edit project"
      disabled={busy ||
        work?.state.loading ||
        work?.state.writing ||
        !!work?.state.pending}
      onclick={() => onOpen("editProject")}
      ><PencilIcon aria-hidden="true" /></Button
    >{/if}
  {#if project && work && writable && tab === "repositories"}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Link repository"
      title="Link repository"
      disabled={busy ||
        work?.state.loading ||
        work?.state.writing ||
        !!work?.state.pending}
      onclick={() => (linkOpen = true)}><LinkIcon aria-hidden="true" /></Button
    >{/if}
  {#if (project && writable) || (tab === "repositories" && controller.state.accessMode === "local_trusted")}
    {#key `${project?.id ?? "hub"}:${tab}`}<StoreProjectsAddMenu
        {controller}
        includeRepository={tab === "repositories"}
        disabled={busy ||
          work?.state.loading ||
          work?.state.writing ||
          !!work?.state.pending}
        onTask={project && writable ? () => onOpen("createTask") : undefined}
      />{/key}
  {/if}
</div>
