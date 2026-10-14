<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import WorkflowPageSkeleton from "./WorkflowPageSkeleton.svelte";
  import WorkflowRunDetail from "./WorkflowRunDetail.svelte";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import type { createWorkflowHistory } from "./workflow-history.svelte.js";
  let {
    history,
    owner,
    catalogLoading,
    selection,
    sourceRepository,
    repositoryName,
    writable,
    onChange,
    onClose,
  }: {
    history: ReturnType<typeof createWorkflowHistory>;
    owner?: WorkflowController;
    catalogLoading: boolean;
    selection: WorkflowSelection;
    sourceRepository?: string;
    repositoryName(id: string): string;
    writable: boolean;
    onChange(fields: WorkflowSelection): void;
    onClose(): void;
  } = $props();
</script>

{#if !owner && catalogLoading}<WorkflowPageSkeleton view="detail" />
{:else if !owner}<Button variant="ghost" onclick={onClose}
    >Back to history</Button
  ><StoreWorkEmpty
    title="Metadata home unavailable"
    description="Reconnect with access to the run's metadata home."
  />
{:else if sourceRepository && history.detail?.runs[0] && history.detail.runs[0].source.repository !== sourceRepository}<Button
    variant="ghost"
    onclick={onClose}>Back to history</Button
  ><StoreWorkEmpty
    kind="activity"
    title="Run unavailable in this repository"
    description="Return to Actions or inspect the run from global History."
  />
{:else if history.detail}<WorkflowRunDetail
    page={history.detail}
    work={owner}
    {selection}
    {repositoryName}
    {writable}
    loading={history.detailLoading}
    {onChange}
    {onClose}
    onRefresh={() => void history.inspect()}
    onMore={() => void history.inspect(true)}
  />
{:else}<WorkflowPageSkeleton view="detail" />{/if}
