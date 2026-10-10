<script lang="ts">
  import * as Breadcrumb from "@lapismd/design-core/shadcn/breadcrumb";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import type { StoreController } from "../../controller/contracts.js";
  import { threadsRoute } from "../../controller/threads-route-codec.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import "../store-work/community-work-breadcrumb.css";
  let { controller }: { controller: StoreController } = $props();
  const repository = $derived(controller.state.repository);
  const detail = $derived(controller.threads?.detail);
  const repositoryHref = $derived(
    storeRouteToHash({
      ...controller.state.route,
      page: "files",
      threadId: null,
      replyId: null,
    }) ?? "#",
  );
  const threadsHref = $derived(storeRouteToHash(threadsRoute()));
</script>

<div class="community-application__project-breadcrumb">
  <Breadcrumb.Root>
    <Breadcrumb.List>
      {#if !controller.state.route.threadsHub}<Breadcrumb.Item
          ><Breadcrumb.Link
            href={repositoryHref}
            onclick={(event) => {
              event.preventDefault();
              controller.actions.selectPage("files");
            }}
          >
            <FolderGitIcon aria-hidden="true" /><span title={repository.name}
              >{repository.name}</span
            >
          </Breadcrumb.Link></Breadcrumb.Item
        >
        <Breadcrumb.Separator />{/if}
      <Breadcrumb.Item>
        {#if detail}<Breadcrumb.Link
            href={threadsHref}
            onclick={(event) => {
              event.preventDefault();
              void controller.actions.openThreads?.();
            }}><span>Threads</span></Breadcrumb.Link
          >
        {:else}<Breadcrumb.Page><span>Threads</span></Breadcrumb.Page>{/if}
      </Breadcrumb.Item>
      {#if detail}<Breadcrumb.Separator /><Breadcrumb.Item
          ><Breadcrumb.Page
            ><span title={detail.thread.value.title}
              >{detail.thread.value.title}</span
            ></Breadcrumb.Page
          ></Breadcrumb.Item
        >{/if}
    </Breadcrumb.List>
  </Breadcrumb.Root>
</div>
