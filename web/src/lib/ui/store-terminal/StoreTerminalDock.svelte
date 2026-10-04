<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Plus from "@lucide/svelte/icons/plus";
  import X from "@lucide/svelte/icons/x";
  import Maximize from "@lucide/svelte/icons/maximize-2";
  import Minimize from "@lucide/svelte/icons/minimize-2";
  import type { TerminalController } from "../../terminal/controller.svelte.js";
  import TerminalViewport from "./TerminalViewport.svelte";
  let {
    controller,
    onHide,
    onLink,
  }: { controller: TerminalController; onHide(): void; onLink(): void } =
    $props();
  let sessions = $derived(
    controller.sessions.filter(
      (s) => s.target.repository === controller.repository,
    ),
  );
  let active = $derived(sessions.find((s) => s.id === controller.activeId));
  function tabKeys(event: KeyboardEvent, index: number) {
    if (!["ArrowLeft", "ArrowRight", "Home", "End"].includes(event.key)) return;
    event.preventDefault();
    const next =
      event.key === "Home"
        ? 0
        : event.key === "End"
          ? sessions.length - 1
          : (index + (event.key === "ArrowRight" ? 1 : -1) + sessions.length) %
            sessions.length;
    controller.select(sessions[next].id);
    (event.currentTarget as HTMLElement).parentElement?.parentElement
      ?.querySelectorAll<HTMLButtonElement>('[role="tab"]')
      [next]?.focus();
  }
</script>

<section
  id="store-terminal-dock"
  class="store-terminal-dock"
  aria-label="Repository terminal"
>
  <div class="store-terminal-toolbar">
    <div class="store-terminal-tab-actions">
      <div class="store-terminal-tabs" role="tablist" aria-label="Terminals">
        {#each sessions as session, i (session.id)}
          <div
            class="store-terminal-tab"
            class:active={session.id === active?.id}
          >
            <button
              type="button"
              role="tab"
              aria-selected={session.id === active?.id}
              tabindex={session.id === active?.id ? 0 : -1}
              aria-controls="store-terminal-panel"
              title={session.target.path}
              onclick={() => controller.select(session.id)}
              onkeydown={(event) => tabKeys(event, i)}
              >{session.title || "Shell"}
              {i + 1}{session.exited ? " · Exited" : ""}</button
            >
          </div>
        {/each}
      </div>
      {#if active}<Button
          variant="ghost"
          size="icon-sm"
          class="store-terminal-close-tab"
          aria-label={`Close terminal ${sessions.indexOf(active) + 1}`}
          disabled={active.closing}
          onclick={() => controller.close(active.id)}><X /></Button
        >{/if}
      <Button
        variant="default"
        size="icon-sm"
        class="store-terminal-new-tab"
        aria-label="New terminal"
        disabled={!controller.targetId || controller.busy}
        onclick={() => controller.create()}><Plus /></Button
      >
    </div>
    <div class="store-terminal-window-actions">
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label={controller.maximized
          ? "Restore terminal"
          : "Maximize terminal"}
        onclick={() => (controller.maximized = !controller.maximized)}
        >{#if controller.maximized}<Minimize />{:else}<Maximize />{/if}</Button
      >
      <Button
        variant="ghost"
        size="icon-sm"
        aria-label="Hide terminal panel"
        onclick={onHide}><X /></Button
      >
    </div>
  </div>
  {#if controller.targets.length > 1}
    <label class="store-terminal-folder"
      >New terminal folder
      <select bind:value={controller.targetId} aria-label="Terminal folder"
        ><option value="">Choose a linked folder…</option
        >{#each controller.targets as target}<option value={target.id}
            >{target.path}</option
          >{/each}</select
      >
    </label>
  {/if}
  <div
    id="store-terminal-panel"
    role="tabpanel"
    aria-label={active
      ? `Terminal ${sessions.indexOf(active) + 1}`
      : "Terminal setup"}
    class="store-terminal-panel"
  >
    {#if active}
      <div class="store-terminal-path" title={active.target.path}>
        {active.target.path}{#if active.exited}<span role="status"
            >Session exited</span
          >{/if}
      </div>
      <TerminalViewport session={active} {controller} />
    {:else if controller.loading}<p class="store-terminal-empty" role="status">
        Loading linked folders…
      </p>
    {:else if !controller.targets.length}<div class="store-terminal-empty">
        <p>
          Link an existing local folder to open a terminal for this repository.
        </p>
        <Button variant="outline" onclick={onLink}>Link local folder</Button
        ><Button
          variant="ghost"
          onclick={() =>
            controller.context(controller.repository, controller.profile)}
          >Refresh folders</Button
        >
      </div>
    {:else}<div class="store-terminal-empty">
        <p>
          {controller.targets.length > 1 && !controller.targetId
            ? "Choose the folder for your new terminal."
            : "Open a shell in the linked local folder."}
        </p>
        <Button
          disabled={!controller.targetId || controller.busy}
          onclick={() => controller.create()}
          >{controller.busy ? "Opening…" : "Open terminal"}</Button
        >
      </div>{/if}
  </div>
</section>
