<script lang="ts">
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { Snippet } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityNavigation from "../community-navigation/CommunityNavigation.svelte";
  import type { CommunityNavigationItemModel } from "../community-navigation/model.js";
  import { projectsSidebarModel } from "../store-work/projects-sidebar-model.js";
  import { withThreadNavigation } from "../store-threads/thread-navigation.js";
  import StoreLocalFolders from "../store-settings/StoreLocalFolders.svelte";
  import StoreNativeRepositoryCreate from "../store-settings/StoreNativeRepositoryCreate.svelte";
  import StoreWorkDialog from "../store-work/StoreWorkDialog.svelte";
  import { projectsBusy } from "../store-work/projects-initial-loading.js";
  let {
    controller,
    stackWorkspace,
    activeItemId,
    compact = false,
    sectionOpen = $bindable<Record<string, boolean>>({}),
    itemOpen = $bindable<Record<string, boolean>>({}),
    onSelect,
    onSearch,
    footer,
  }: {
    controller: StoreController;
    stackWorkspace?: StackWorkspaceController;
    activeItemId?: string;
    compact?: boolean;
    sectionOpen?: Record<string, boolean>;
    itemOpen?: Record<string, boolean>;
    onSelect: (item: CommunityNavigationItemModel) => void;
    onSearch?: (query: string) => void;
    footer?: Snippet;
  } = $props();
  let createProject = $state(false);
  const canCreateProject = $derived(
    !!controller.projects &&
      (controller.state.accessMode === "local_trusted" ||
        controller.state.repositories.some(
          (repo) => repo.permission === "write" || repo.permission === "admin",
        )),
  );
  const model = $derived(
    withThreadNavigation(
      controller,
      projectsSidebarModel(
        controller.projects?.navigation.homes ?? [],
        controller.state.repositories,
        !!controller.projects,
      ),
    ),
  );
</script>

{#snippet navigation(
  add?: () => void,
  disabled = false,
  create?: () => void,
  creationDisabled = false,
)}
  <CommunityNavigation
    model={{
      ...model,
      primaryItems: [
        ...model.primaryItems,
        ...(stackWorkspace?.snapshot?.folder &&
        stackWorkspace.snapshot.repository === controller.state.route.repository
          ? [{ id: "stacks", kind: "stacks" as const, label: "Stacks" }]
          : []),
      ],
      sections: model.sections.map((section) => ({
        ...section,
        actions: [
          ...(section.action ? [section.action] : []),
          ...(section.id === "projects" && canCreateProject
            ? [
                {
                  id: "add-project",
                  kind: "add-project" as const,
                  label: "Add project",
                  disabled: projectsBusy(controller),
                },
              ]
            : []),
          ...(section.id === "repositories" && add
            ? [
                {
                  id: `add-repository:${section.id}`,
                  kind: "add-repository" as const,
                  label: "Add repository",
                  disabled:
                    disabled &&
                    (!create ||
                      !controller.state.nativeRepositoryCreation ||
                      creationDisabled),
                  children: [
                    ...(create && controller.state.nativeRepositoryCreation
                      ? [
                          {
                            id: "create-native-repository",
                            kind: "create-server-repository" as const,
                            label: "Add Server Repository",
                            disabled: creationDisabled,
                          },
                        ]
                      : []),
                    {
                      id: "add-local-repository",
                      kind: "add-local-repository" as const,
                      label: "Add Local Repository",
                      disabled,
                    },
                  ],
                },
              ]
            : []),
        ],
      })),
    }}
    {activeItemId}
    {compact}
    bind:sectionOpen
    bind:itemOpen
    {onSearch}
    {footer}
    onSelect={(item) => {
      if (item.disabled) return;
      if (item.id === "create-native-repository") create?.();
      else if (item.kind === "add-local-repository") add?.();
      else if (item.kind === "add-project") createProject = true;
      else onSelect(item);
    }}
  />
{/snippet}

{#if controller.state.accessMode === "local_trusted"}
  <StoreNativeRepositoryCreate {controller}>
    {#snippet trigger(create, creationDisabled)}
      <StoreLocalFolders {controller} mode="add">
        {#snippet trigger(add, unavailable)}{@render navigation(
            add,
            unavailable,
            create,
            creationDisabled,
          )}{/snippet}
      </StoreLocalFolders>
    {/snippet}
  </StoreNativeRepositoryCreate>
{:else}{@render navigation()}{/if}

{#if createProject}<StoreWorkDialog
    {controller}
    mode="createProject"
    onClose={() => (createProject = false)}
  />{/if}
