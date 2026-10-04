<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import GitCommitHorizontalIcon from "@lucide/svelte/icons/git-commit-horizontal";

  let {
    pane,
    branch = "",
  }: {
    pane: "log" | "oplog";
    branch?: string | undefined;
  } = $props();

  const title = $derived(
    pane === "oplog"
      ? "No operations"
      : branch
        ? "No revisions for this bookmark"
        : "No revisions",
  );
  const description =
    "This fixture has no history to display. Select another bookmark or repository.";
</script>

<Empty.Root
  class="community-projects__empty community-projects-repository-log__empty"
>
  <Empty.Header>
    <Empty.Media variant="icon">
      <GitCommitHorizontalIcon aria-hidden="true" />
    </Empty.Media>
    <Empty.Title>{title}</Empty.Title>
    <Empty.Description>{description}</Empty.Description>
  </Empty.Header>
</Empty.Root>
