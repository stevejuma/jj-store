<script lang="ts">
  import { onDestroy } from "svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryConversionDialog from "./StoreRepositoryConversionDialog.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { ApiError, pretty, type Model } from "../../api/client.js";
  import {
    RepositoryConversionManager,
    type ConversionIntent,
  } from "../../controller/repository-conversion-manager.js";
  import { emptyRoute } from "../../controller/global-search-presentation.js";
  import { storeErrorMessage } from "../../controller/store-error.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    onClose,
  }: { controller: StoreController; onClose?: () => void } = $props();
  const id = $props.id();
  const view = $derived(controller.state);
  const local = $derived(view.repository.profile === "git_sha1_jj045");
  const enabled = $derived(
    view.accessMode === "local_trusted" &&
      view.repositoryConversion === true &&
      view.repository.permission === "admin" &&
      Boolean(view.repository.id),
  );
  let manager = $state<RepositoryConversionManager>();
  let open = $state(false),
    busy = $state(false),
    pending = $state(false),
    hasReport = $state(false),
    name = $state(""),
    path = $state(""),
    operation = $state(""),
    error = $state(""),
    validation = $state(""),
    progress = $state("");
  let checkedIntent = $state<ConversionIntent | null>(null);
  let preview = $state<Model<"ConversionPreview"> | null>(null),
    report = $state<Model<"ConversionReport"> | null>(null);
  $effect(() => {
    if (enabled && !manager) {
      try {
        manager = new RepositoryConversionManager(undefined, localStorage);
        pending = Boolean(manager.pending);
        hasReport = manager.hasReport;
      } catch (cause) {
        error = storeErrorMessage(cause);
      }
    }
  });
  onDestroy(() => manager?.close());
  function edit() {
    preview = null;
    checkedIntent = null;
    validation = "";
    progress = "";
  }
  function show() {
    if (!manager) return;
    try {
      manager.refresh();
      pending = Boolean(manager.pending);
      const saved = manager.pending?.intent;
      name =
        saved?.name ??
        `${view.repository.name} (${local ? "server" : "local"})`;
      path = saved?.path ?? "";
      operation =
        saved?.operation ??
        (view.operationHeads.includes(view.route.snapshotOperationId ?? "")
          ? view.route.snapshotOperationId!
          : view.operationHeads.length === 1
            ? view.operationHeads[0]
            : "");
      preview = null;
      report = null;
      validation = "";
      error = "";
      progress = pending
        ? "Conversion is awaiting confirmation. Check status or retry the same request."
        : "";
      open = true;
    } catch (cause) {
      error = storeErrorMessage(cause);
    }
  }
  function intent(): ConversionIntent {
    return {
      version: 1,
      source: view.repository.id,
      source_profile: local ? "local_jj_git" : "native",
      operation,
      destination_profile: local ? "native" : "local_jj_git",
      name: name.trim(),
      path: local ? null : path.trim(),
    };
  }
  async function check() {
    if (busy || !manager) return;
    busy = true;
    error = "";
    validation = "";
    try {
      checkedIntent = intent();
      preview = await manager.preview(checkedIntent);
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        ["invalid_local_folder", "local_registration_conflict"].includes(
          cause.code,
        )
      )
        validation =
          "Choose a valid name and an absolute path to a new folder. The destination must be unused.";
      else error = storeErrorMessage(cause);
    } finally {
      busy = false;
    }
  }
  async function run(status = false, showReport = false) {
    if (busy || !manager) return;
    busy = true;
    error = "";
    validation = "";
    try {
      const result = status
        ? await manager.status()
        : await manager.start(
            checkedIntent ?? manager.pending?.intent ?? intent(),
            preview?.prepared_source ?? manager.pending?.prepared_source ?? "",
          );
      if (result?.status === "committed") {
        report = result.report;
        pending = false;
        progress = "Conversion completed and verified.";
        if (showReport) {
          open = true;
        } else {
          open = false;
          await controller.projects?.refresh();
          await controller.actions.openRoute?.({
            ...emptyRoute(result.report.destination),
            page: "files",
          });
          onClose?.();
        }
      } else
        progress =
          "Conversion is awaiting confirmation. Check status or retry the same request.";
    } catch (cause) {
      if (cause instanceof ApiError && cause.code === "stale_local_revision") {
        preview = null;
        validation =
          "The source changed after preflight. Check it again before converting.";
      } else if (
        cause instanceof ApiError &&
        cause.code === "local_registration_conflict"
      )
        validation =
          "The destination conflicts with another repository or folder. Your exact request is retained for status and retry.";
      else error = storeErrorMessage(cause);
    } finally {
      pending = Boolean(manager.pending);
      hasReport = manager.hasReport;
      busy = false;
    }
  }
  function download() {
    if (!report) return;
    const url = URL.createObjectURL(
      new Blob([pretty(report)], { type: "application/json" }),
    );
    const link = document.createElement("a");
    link.href = url;
    link.download = `repository-conversion-${report.request_id}.json`;
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 0);
  }
</script>

{#if enabled}
  <section class="store-settings__section" aria-labelledby={`${id}-heading`}>
    <header><h2 id={`${id}-heading`}>Repository storage</h2></header>
    <p class="store-settings__row-description">
      {local
        ? "Use this installation's server storage for code and collaboration."
        : "Create a local Git-backed JJ folder with code and collaboration."}
    </p>
    <div class="store-settings__actions">
      <Button variant="outline" disabled={busy || !manager} onclick={show}
        >{pending
          ? "Continue conversion"
          : local
            ? "Use server storage"
            : "Use local Git-backed JJ"}</Button
      >
      {#if hasReport}<Button
          variant="ghost"
          disabled={busy || !manager}
          onclick={() => run(true, true)}>Last conversion report</Button
        >{/if}
    </div>
  </section>
{/if}
<StoreNotice
  noticeKey={`repository-conversion:${manager?.pending?.request_id ?? id}`}
  title="Repository conversion failed"
  message={error}
  lifetime="dismiss"
  pending={pending || (busy && !report)}
/>
<StoreRepositoryConversionDialog
  bind:open
  bind:name
  bind:path
  bind:operation
  {local}
  {busy}
  {pending}
  pendingIntent={manager?.pending?.intent ?? null}
  operationHeads={view.operationHeads}
  {validation}
  {progress}
  {preview}
  {report}
  onEdit={edit}
  onCheck={check}
  onStart={() => run()}
  onStatus={() => run(true)}
  onDownload={download}
/>
