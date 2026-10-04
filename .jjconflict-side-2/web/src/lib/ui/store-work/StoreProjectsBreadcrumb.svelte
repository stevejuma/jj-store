<script lang="ts">
  // CommunityApplication projectBreadcrumb, with native routes.
  import * as Breadcrumb from "@lapismd/design-core/shadcn/breadcrumb";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import type { StoreController } from "../../controller/contracts.js";
  import { projectsRouteToHash } from "../../controller/projects-route-codec.js";
  import "./community-work-breadcrumb.css";
  let { controller }: { controller: StoreController } = $props();
  const hub = $derived(controller.projects!),
    route = $derived(hub.route),
    project = $derived(hub.detail?.state.project),
    task = $derived(hub.detail?.state.task),
    tab = $derived(route.workTab ?? (route.projectId ? "tasks" : "projects"));
  const projectHref = $derived(
    projectsRouteToHash({ ...route, taskId: null, workTab: "tasks" }) ??
      "#/projects",
  );
</script>

<div class="community-application__project-breadcrumb">
  <Breadcrumb.Root>
    <Breadcrumb.List>
      <Breadcrumb.Item>
        {#if !route.projectId && tab === "projects"}<Breadcrumb.Page
            ><FolderKanbanIcon aria-hidden="true" /><span>Projects</span
            ></Breadcrumb.Page
          >
        {:else}<Breadcrumb.Link
            href="#/projects"
            onclick={(event) => {
              event.preventDefault();
              void hub.openHub();
            }}
            ><FolderKanbanIcon aria-hidden="true" /><span>Projects</span
            ></Breadcrumb.Link
          >{/if}
      </Breadcrumb.Item>
      {#if route.projectId}<Breadcrumb.Separator /><Breadcrumb.Item>
          <Breadcrumb.Link
            href={projectHref}
            onclick={(event) => {
              event.preventDefault();
              void hub.openProject(route.repository, route.projectId!);
            }}
            ><span title={project?.title}>{project?.title ?? "Project"}</span
            ></Breadcrumb.Link
          >
        </Breadcrumb.Item>{/if}
      {#if route.projectId || tab !== "projects"}<Breadcrumb.Separator
        /><Breadcrumb.Item>
          {#if task}<Breadcrumb.Link
              href={projectHref}
              onclick={(event) => {
                event.preventDefault();
                void hub.openProject(route.repository, route.projectId!);
              }}><span>Tasks</span></Breadcrumb.Link
            >
          {:else}<Breadcrumb.Page
              ><span>{tab[0]!.toUpperCase() + tab.slice(1)}</span
              ></Breadcrumb.Page
            >{/if}
        </Breadcrumb.Item>{/if}
      {#if task}<Breadcrumb.Separator /><Breadcrumb.Item
          ><Breadcrumb.Page
            ><span title={task.title}>{task.title}</span></Breadcrumb.Page
          ></Breadcrumb.Item
        >{/if}
    </Breadcrumb.List>
  </Breadcrumb.Root>
</div>
