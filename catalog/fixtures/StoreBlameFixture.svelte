<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FilePanel from "../../web/src/lib/ui/community-projects/CommunityProjectsRepositoryFilePanel.svelte";
  import { createStoreRouteHistory } from "../../web/src/lib/controller/store-route-history.svelte.js";
  import { storeRouteToHash } from "../../web/src/lib/controller/store-route-codec.js";
  import { createStoreQueryClient } from "../../web/src/lib/controller/store-query-client.js";
  import { createBlameQuery } from "../../web/src/lib/controller/blame-query.js";
  import type { StoreController } from "../../web/src/lib/controller/contracts.js";
  import type { BlameProgress } from "../../web/src/lib/controller/blame-types.js";
  import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
  import "../../web/src/lib/ui/community-projects/repository-files.css";
  let {
    enabled = true,
    restored = false,
    empty = false,
    contentKind = "text",
    sample,
  }: {
    enabled?: boolean;
    restored?: boolean;
    empty?: boolean;
    contentKind?: "text" | "binary" | "large";
    sample?: { path: string; contents: string };
  } = $props();
  const origin = "56".repeat(20);
  const path = untrack(() => sample?.path ?? "src/sample.ts");
  const initial = `#/repos/${"ab".repeat(16)}/files?at_op=${"12".repeat(64)}&commit=${"34".repeat(20)}&file=${encodeURIComponent(path)}`;
  const browserHashes = [
    untrack(() => initial + (restored ? "&file_view=blame" : "")),
  ];
  let index = 0;
  let listener: (() => void) | undefined;
  const history = createStoreRouteHistory({
    hash: () => browserHashes[index]!,
    push(hash) {
      browserHashes.splice(++index);
      browserHashes.push(hash);
    },
    replace(hash) {
      browserHashes[index] = hash;
    },
    go(delta) {
      index = Math.max(0, Math.min(browserHashes.length - 1, index + delta));
      listener?.();
    },
    listen(next) {
      listener = next;
      return () => {
        listener = undefined;
      };
    },
  });
  const queries = createStoreQueryClient();
  let calls = $state(0),
    cancellations = $state(0),
    pending = $state(false);
  let finish: (() => void) | undefined, fail: (() => void) | undefined;
  const contents = untrack(() =>
    empty
      ? ""
      : (sample?.contents ??
        Array.from(
          { length: 600 },
          (_, i) =>
            `const value${i + 1} = ${i + 1};${i === 0 ? " // " + "a long source line ".repeat(100) : ""}`,
        ).join("\n")),
  );
  const blob = $derived({
    path,
    contents,
    binary: contentKind === "binary",
    too_large: contentKind === "large",
  });
  const provider = createBlameQuery({
    client: queries,
    scope: "fixture-blame-v1",
    active: () => true,
    read(selection, continuation, signal) {
      calls++;
      pending = true;
      return new Promise<BlameProgress>((resolve, reject) => {
        signal.addEventListener(
          "abort",
          () => {
            cancellations++;
            pending = false;
            reject(new DOMException("Aborted", "AbortError"));
          },
          { once: true },
        );
        finish = () => {
          pending = false;
          resolve({
            lineCount: selection.lineCount,
            complete: Boolean(continuation) || empty,
            continuation: continuation || empty ? null : "second",
            ranges: empty
              ? []
              : [{ start: 1, end: continuation ? 590 : 101, commitId: origin }],
            commits: {
              [origin]: {
                eventId: origin,
                commitId: origin,
                changeId: "78".repeat(16),
                author: "Example Author",
                message:
                  "Correct line attribution with a very long description that must never push source or time out of place\nBody text must stay hidden",
                createdAt: 1_700_000_000,
              },
            },
          });
        };
        fail = () => {
          pending = false;
          reject(new Error("Fixture annotation failed"));
        };
      });
    },
  });
  const base = createInMemoryStoreController("files");
  const controller: StoreController = {
    get blame() {
      return enabled ? provider : undefined;
    },
    get state() {
      return {
        ...base.state,
        route: history.route,
        blob,
        fileRevision: undefined,
        targets: ["34".repeat(20)],
      };
    },
    actions: {
      ...base.actions,
      setFileDisplay(fileDisplay) {
        history.push({ ...history.route, fileDisplay });
      },
      async openRoute(route) {
        history.push(route);
      },
    },
  };
  onDestroy(() => {
    history.destroy();
    queries.clear();
  });
</script>

<div class="fixture-controls">
  <FixtureControls floating={false}
    ><Button onclick={() => finish?.()} disabled={!pending}
      >Resolve window</Button
    ></FixtureControls
  >
  <Button onclick={() => fail?.()} disabled={!pending}>Fail window</Button>
  <Button onclick={() => history.navigate(-1)} disabled={!history.canBack}
    >Back</Button
  >
  <Button onclick={() => history.navigate(1)} disabled={!history.canForward}
    >Forward</Button
  >
  <output aria-label="Blame requests">{calls}</output>
  <output aria-label="Canceled requests">{cancellations}</output>
</div>
<output aria-label="Current route">{storeRouteToHash(history.route)}</output>
<div class="fixture-panel">
  {#if history.route.page === "files"}
    <FilePanel
      {controller}
      {blob}
      selectedPath={blob.path}
      now={1_700_086_400}
      isMarkdown={false}
      fileTreeOpen={false}
      onToggleFileTree={() => {}}
    />
  {:else}<p>Commits: {history.route.commitId}</p>{/if}
</div>

<style>
  .fixture-controls {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  output {
    overflow-wrap: anywhere;
    font-size: 0.75rem;
  }
  .fixture-panel {
    height: 700px;
    width: min(100%, 950px);
    min-width: 0;
    margin-top: 0.5rem;
  }
</style>
