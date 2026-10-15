import { describe, expect, it } from "vitest";
import {
  appearanceStorageKey,
  defaultAppearance,
  parseAppearance,
  resolveTheme,
  switchMode,
} from "../../src/lib/appearance/preferences.js";
import { createAppearanceStore } from "../../src/lib/appearance/store.svelte.js";
import { createDrafts } from "../../src/lib/appearance/drafts.svelte.js";
import {
  themeNames,
  buzzThemeNames,
  lightThemes,
  pairedThemes,
  themePairs,
} from "../../src/lib/appearance/theme-catalog.js";
import { localLink } from "../../src/lib/appearance/local-link.js";
import {
  applyCommunityMarkdownSyntax,
  type CommunityMarkdownHastNode,
} from "../../src/lib/ui/community-markdown-content/syntax-hast.js";
import { createMemoryStoryStorage } from "../../../catalog/fixtures/memory-storage.js";
import { createThemeVars } from "../../src/lib/appearance/adaptive-theme.js";

describe("device appearance", () => {
  it("defaults new link preferences to inline without resetting saved card styles", () => {
    expect(parseAppearance(null).links).toBe("inline");
    expect(parseAppearance('{"version":1,"links":"invalid"}').links).toBe(
      "inline",
    );
    for (const links of ["inline", "compact", "rich"] as const) {
      const storage = createMemoryStoryStorage();
      const store = createAppearanceStore(storage);
      store.update({ links });
      expect(createAppearanceStore(storage).saved.links).toBe(links);
      const saved = storage.getItem(appearanceStorageKey);
      store.preview({ links: "rich" });
      expect(storage.getItem(appearanceStorageKey)).toBe(saved);
      store.resetPreview();
      expect(store.live.links).toBe(links);
      store.restore('{"version":1,"links":"compact"}');
      expect(store.live.links).toBe("compact");
    }
  });
  it.each([
    ["#ffffff", "#24292e", "#6a737d"],
    ["#0d1117", "#c9d1d9", "#8b949e"],
    ["#282a36", "#f8f8f2", "#6272a4"],
  ])("uses the app surface for popovers derived from %s", (bg, fg, comment) => {
    const { vars } = createThemeVars(bg, fg, comment);
    expect(vars["--popover"]).toBe(vars["--background"]);
    expect(vars["--popover-foreground"]).toBe(vars["--foreground"]);
  });
  it("restores the original theme and follows its light and dark pair", () => {
    const prefs = parseAppearance(
      '{"version":1,"mode":"system","theme":"original"}',
    );
    expect(prefs.theme).toBe("original");
    expect(resolveTheme(prefs, false)).toBe("original");
    expect(resolveTheme(prefs, true)).toBe("original-dark");
    const storage = createMemoryStoryStorage();
    const store = createAppearanceStore(storage);
    store.update(prefs);
    store.mode("dark");
    expect(store.theme).toBe("original-dark");
    store.mode("light");
    expect(store.theme).toBe("original");
    expect(createAppearanceStore(storage).saved.theme).toBe("original");
  });
  it("contains the complete pinned theme catalog and reciprocal pairs", () => {
    expect(buzzThemeNames).toHaveLength(62);
    expect(themeNames).toHaveLength(64);
    expect(new Set(themeNames).size).toBe(64);
    expect(lightThemes.size).toBe(20);
    expect(pairedThemes).toHaveLength(19);
    for (const [name, pair] of themePairs)
      expect(themePairs.get(pair)).toBe(name);
  });
  it("validates corrupt, future and out-of-range storage without adopting unknown fields", () => {
    for (const raw of [null, "bad json", '{"version":2}', "null"])
      expect(parseAppearance(raw)).toEqual(defaultAppearance);
    const saved = parseAppearance(
      JSON.stringify({
        version: 1,
        theme: "missing",
        accent: "url(evil)",
        opacity: 1000,
        fontSize: "huge",
        glass: true,
      }),
    );
    expect(saved).toEqual({ ...defaultAppearance, opacity: 90, glass: true });
    expect(parseAppearance('{"version":1,"opacity":-8}').opacity).toBe(30);
  });
  it("follows system pairs and switches unpaired modes to a working fallback", () => {
    const prefs = { ...defaultAppearance, theme: "rose-pine-dawn" as const };
    expect(resolveTheme(prefs, false)).toBe("rose-pine-dawn");
    expect(resolveTheme(prefs, true)).toBe("rose-pine");
    expect(
      parseAppearance('{"version":1,"mode":"system","theme":"andromeeda"}')
        .theme,
    ).toBe("buzz");
    expect(switchMode({ ...prefs, theme: "andromeeda" }, "system")).toEqual({
      mode: "system",
      theme: "buzz",
    });
    expect(switchMode({ ...prefs, theme: "andromeeda" }, "light")).toEqual({
      mode: "light",
      theme: "buzz",
    });
  });
  it("previews without persisting, restores device state and tolerates unavailable storage", () => {
    const storage = createMemoryStoryStorage();
    const store = createAppearanceStore(storage);
    store.update({
      fontSize: "larger",
      density: "spacious",
      links: "rich",
      threads: "focus",
    });
    const saved = storage.getItem(appearanceStorageKey);
    store.preview({ fontSize: "smaller" });
    expect(store.live.fontSize).toBe("smaller");
    expect(storage.getItem(appearanceStorageKey)).toBe(saved);
    store.resetPreview();
    expect(store.live.fontSize).toBe("larger");
    expect(createAppearanceStore(storage).saved).toEqual(store.saved);
    const broken = {
      getItem() {
        throw Error("disabled");
      },
      setItem() {
        throw Error("quota");
      },
    } as unknown as Storage;
    const memory = createAppearanceStore(broken);
    memory.update({ glass: true });
    expect(memory.live.glass).toBe(true);
    memory.restore(null);
    expect(memory.saved).toEqual(defaultAppearance);
  });
  it("keeps drafts by exact repository and conversation identities", () => {
    let repo = "a";
    const drafts = createDrafts(() => repo);
    const first = drafts.get("root");
    first.value = "unsent";
    first.editing = "message-a";
    expect(drafts.get("root")).toBe(first);
    repo = "b";
    expect(drafts.get("root").value).toBe("");
    repo = "a";
    expect(drafts.get("root").editing).toBe("message-a");
  });
});

describe("local link cards", () => {
  it("accepts only HTTP(S) with existing labels and no credentials", () => {
    expect(localLink("https://example.com/path", "Existing label")).toEqual({
      href: "https://example.com/path",
      host: "example.com",
      label: "Existing label",
    });
    for (const href of [
      "javascript:alert(1)",
      "data:text/html,hi",
      "file:///etc/passwd",
      "https://user:password@example.com",
      "/relative",
    ])
      expect(localLink(href, "label")).toBeUndefined();
  });
  it("keeps native entity links and code while converting ordinary website anchors", () => {
    const link = (href: string): CommunityMarkdownHastNode => ({
      type: "element",
      tagName: "a",
      properties: { href },
      children: [{ type: "text", value: "title" }],
    });
    const tree: CommunityMarkdownHastNode = {
      type: "root",
      children: [
        link("https://example.com"),
        link("jj://repo/test"),
        {
          type: "element",
          tagName: "code",
          children: [link("https://example.com/code")],
        },
      ],
    };
    applyCommunityMarkdownSyntax(
      tree,
      {
        members: () => [],
        rooms: () => [],
        emojis: () => [],
        mentionPubkeys: () => [],
      },
      (v) => [{ kind: "text", value: v }],
    );
    expect(tree.children![0]!.tagName).toBe("communityexternallink");
    expect(tree.children![1]!.tagName).not.toBe("communitylinkcard");
    expect(tree.children![2]!.children![0]!.tagName).toBe("a");
  });
});

it("defaults missing conversation view to Document and keeps placement independent", () => {
  expect(parseAppearance(null).conversationView).toBe("document");
  expect(parseAppearance('{"version":1,"threads":"focus"}')).toMatchObject({
    conversationView: "document",
    threads: "focus",
  });
  expect(
    parseAppearance('{"version":1,"conversationView":"unknown"}')
      .conversationView,
  ).toBe("document");
  const storage = createMemoryStoryStorage(),
    store = createAppearanceStore(storage);
  store.update({ conversationView: "classic", threads: "focus" });
  expect(createAppearanceStore(storage).saved).toMatchObject({
    conversationView: "classic",
    threads: "focus",
  });
});
