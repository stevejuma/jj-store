<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import History from "@lucide/svelte/icons/history";
  import FolderKanban from "@lucide/svelte/icons/folder-kanban";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import ListTodo from "@lucide/svelte/icons/list-todo";
  import MessageSquare from "@lucide/svelte/icons/message-square";
  let {
    kind = "projects",
    title,
    description,
  }: {
    kind?:
      | "projects"
      | "repositories"
      | "tasks"
      | "reviews"
      | "comments"
      | "activity";
    title: string;
    description: string;
  } = $props();
  const Icon = $derived(
    kind === "activity"
      ? History
      : kind === "repositories"
        ? FolderGit
        : kind === "tasks"
          ? ListTodo
          : kind === "reviews" || kind === "comments"
            ? MessageSquare
            : FolderKanban,
  );
</script>

<Empty.Root
  ><Empty.Header
    ><Empty.Media variant="icon"><Icon aria-hidden="true" /></Empty.Media
    ><Empty.Title>{title}</Empty.Title><Empty.Description
      >{description}</Empty.Description
    ></Empty.Header
  ></Empty.Root
>
