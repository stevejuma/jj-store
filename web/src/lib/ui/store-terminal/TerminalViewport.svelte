<script lang="ts">
  import { copyTerminalSelection } from "../../terminal/selection.js";
  import { onMount, tick } from "svelte";
  import type {
    TerminalController,
    TerminalView,
  } from "../../terminal/controller.svelte.js";
  import { TERMINAL_CELL_METRICS } from "../../terminal/renderer.js";
  import { terminalPalette } from "../../terminal/palette.js";
  import {
    encodeTerminalKey,
    encodePaste,
    accumulateScrollLines,
  } from "../../terminal/keyboard.js";
  let {
    session,
    controller,
  }: { session: TerminalView; controller: TerminalController } = $props();
  let root: HTMLDivElement;
  let canvas: HTMLCanvasElement;
  let input: HTMLTextAreaElement;
  let composition = false;
  let remainder = { remainderPx: 0 };
  let rows = $derived.by(() => {
    session.revision;
    return session.grid?.selectionRows() ?? [];
  });
  function paint() {
    if (!root || !canvas) return;
    const context = canvas.getContext("2d");
    if (!context) return;
    const rect = root.getBoundingClientRect();
    const ratio = window.devicePixelRatio || 1;
    canvas.width = Math.max(1, Math.round(rect.width * ratio));
    canvas.height = Math.max(1, Math.round(rect.height * ratio));
    context.setTransform(ratio, 0, 0, ratio, 0, 0);
    const palette = terminalPalette(root);
    context.fillStyle = palette.background;
    context.fillRect(0, 0, rect.width, rect.height);
    session.grid?.markAllDirty();
    session.grid?.paint(context, TERMINAL_CELL_METRICS, palette);
  }
  function resize() {
    paint();
    const rect = root.getBoundingClientRect();
    if (rect.width <= 0 || rect.height <= 0) return;
    controller.resize(
      session.id,
      Math.max(2, Math.min(512, Math.floor(rect.width / 8.4))),
      Math.max(1, Math.min(200, Math.floor(rect.height / 17))),
    );
  }
  $effect(() => {
    session.id;
    session.revision;
    paint();
  });
  $effect(() => {
    session.id;
    void tick().then(() => input?.focus({ preventScroll: true }));
  });
  onMount(() => {
    const copy = (event: ClipboardEvent) =>
      copyTerminalSelection(event, session.grid, root);
    document.addEventListener("copy", copy);
    let timer: ReturnType<typeof setTimeout>;
    const observer = new ResizeObserver(() => {
      paint();
      clearTimeout(timer);
      timer = setTimeout(resize, 80);
    });
    observer.observe(root);
    const theme = new MutationObserver(paint);
    theme.observe(document.documentElement, {
      attributes: true,
      attributeFilter: [
        "class",
        "style",
        "data-theme",
        "data-ui-theme",
        "data-store-theme",
      ],
    });
    return () => {
      document.removeEventListener("copy", copy);
      observer.disconnect();
      theme.disconnect();
      clearTimeout(timer);
    };
  });
  function send(data: string) {
    if (data && !session.exited) controller.input(session.id, data);
  }
  function key(event: KeyboardEvent) {
    if (event.isComposing || composition) return;
    if (event.metaKey && event.code === "KeyJ") return;
    const encoded = encodeTerminalKey(event);
    if (encoded) {
      event.preventDefault();
      event.stopPropagation();
      send(encoded);
    }
  }
</script>

<div
  class="store-terminal-viewport"
  bind:this={root}
  onwheel={(event) => {
    event.preventDefault();
    const pixels =
      event.deltaMode === 1
        ? event.deltaY * 17
        : event.deltaMode === 2
          ? event.deltaY * root.clientHeight
          : event.deltaY;
    const next = accumulateScrollLines(remainder, pixels, 17);
    remainder = next.state;
    if (next.lines) controller.scroll(session.id, next.lines);
  }}
>
  <canvas bind:this={canvas} aria-hidden="true"></canvas>
  <div
    class="store-terminal-selection"
    aria-hidden="true"
    onpointerup={() => {
      if (window.getSelection()?.isCollapsed !== false)
        input.focus({ preventScroll: true });
    }}
  >
    {#each rows as row (row.line)}<div data-terminal-row={row.line}>
        {row.text || "\u00a0"}
      </div>{/each}
  </div>
  <textarea
    bind:this={input}
    aria-label="Terminal input"
    aria-describedby="terminal-output-help"
    disabled={session.exited}
    autocomplete="off"
    autocapitalize="off"
    spellcheck={false}
    onkeydown={key}
    oncompositionstart={() => (composition = true)}
    oncompositionend={(event) => {
      composition = false;
      send(event.currentTarget.value);
      event.currentTarget.value = "";
    }}
    oninput={(event) => {
      if (composition || (event instanceof InputEvent && event.isComposing))
        return;
      send(event.currentTarget.value);
      event.currentTarget.value = "";
    }}
    onpaste={(event) => {
      event.preventDefault();
      send(
        encodePaste(
          event.clipboardData?.getData("text/plain") ?? "",
          session.paste,
        ),
      );
    }}
    onfocus={() => controller.focus(session.id, true)}
    onblur={() => {
      composition = false;
      controller.focus(session.id, false);
    }}
  ></textarea>
  <span id="terminal-output-help" class="store-terminal-sr"
    >Type commands here. Terminal output is available in the following text
    region.</span
  >
  <div
    role="textbox"
    aria-readonly="true"
    aria-multiline="true"
    aria-label="Terminal output"
    tabindex="0"
    class="store-terminal-sr"
  >
    {rows.map((row) => row.text).join("\n")}
  </div>
</div>
