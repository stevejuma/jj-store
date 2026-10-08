<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreAgentChatBody from "../../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import Fixture from "../../fixtures/ChatBodyFixture.svelte";
  import {
    reader,
    draft,
    addComment,
  } from "../../fixtures/delta-chat/acceptance-helpers.js";
  import {
    patchFilesCollapse,
    shellAndTurnNavigation,
  } from "../../fixtures/delta-chat/acceptance-plays.js";
  import { chatBodyExample } from "../../fixtures/delta-chat/example-source.js";
  import { largeCaptures } from "../../fixtures/delta-chat/large-capture-play.js";
  import { messageActions } from "../../fixtures/delta-chat/message-action-play.js";
  import { nextMessagePlay } from "../../fixtures/delta-chat/next-message-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Commentable agent chat body",
    component: StoreAgentChatBody,
    tags: ["visual-pending"],
    args: { items: [], onAddComment: () => {}, onReply: () => {} },
    parameters: {
      layout: "fullscreen",
      docs: {
        source: {
          language: "svelte",
          code: chatBodyExample,
        },
      },
    },
  });
</script>

<Story asChild name="Interaction playground">
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>
<Story
  asChild
  name="Inline next message and fixed send actions"
  play={nextMessagePlay}
>
  {#snippet children()}<Fixture scenario="playground" showComposer />{/snippet}
</Story>
<Story asChild name="Message reactions and XML context" play={messageActions}>
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>
<Story asChild name="Large code and diff middle folding" play={largeCaptures}>
  {#snippet children()}<Fixture scenario="large" />{/snippet}
</Story>

<Story
  asChild
  name="Delta main thread reference"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await reader(canvasElement, "User message")).toHaveTextContent(
      "Would like to extend the functionality",
    );
    await expect(
      c.getByRole("button", {
        name: "Expand Listed a directory, ran a command, read 7 files, ran a search, thought",
      }),
    ).toBeVisible();
    await expect(
      c.getByRole("textbox", {
        name: "Captured patch · Sources/BedtimeCore/Schedule.swift",
      }),
    ).toHaveTextContent("public enum BedtimeAction");
    await userEvent.click(
      c.getByRole("button", {
        name: "Expand Listed a directory, ran a command, read 7 files, ran a search, thought",
      }),
    );
    await userEvent.click(
      c.getByRole("button", {
        name: "Show details for Read Sources/BedtimeCore/Schedule.swift",
      }),
    );
    await expect(
      await reader(canvasElement, "Sources/BedtimeCore/Schedule.swift"),
    ).toHaveTextContent("import Foundation");
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>

<Story
  asChild
  name="Typing anchors a discussion and preserves the captured text"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      source = await reader(canvasElement);
    const original = source.textContent;
    await userEvent.click(source);
    await userEvent.keyboard("Please clarify");
    const editor = await draft(canvasElement);
    await waitFor(() => expect(editor).toHaveTextContent("Please clarify"));
    expect(source.textContent).toContain("stable identities");
    await expect(
      canvasElement.querySelector(".chat-comment-highlight"),
    ).not.toBeNull();
    await userEvent.keyboard("{Enter}More detail");
    await expect(editor).toHaveTextContent("More detail");
    await addComment(canvasElement);
    await expect(
      await c.findByRole("article", { name: "Comment by You" }),
    ).toHaveTextContent("Please clarify");
    await expect(source).toHaveTextContent("stable identities");
    await userEvent.click(c.getByRole("button", { name: "Reply" }));
    await userEvent.type(await draft(canvasElement), "Here is a reply");
    await userEvent.click(c.getByRole("button", { name: "Add reply" }));
    expect(c.getAllByRole("article", { name: "Comment by You" })).toHaveLength(
      2,
    );
    await userEvent.click(c.getByRole("button", { name: "Append response" }));
    await expect(
      c.getByRole("article", { name: "Appended response" }),
    ).toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Replace content" }));
    await expect(
      c.getByRole("complementary", { name: "Comments on unavailable content" }),
    ).toHaveTextContent("Please clarify");
    expect(original).toContain("stable identities");
  }}>{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Keyboard selection paste and Escape retain an exact draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      source = await reader(canvasElement);
    source.focus();
    // Native pointer/keyboard selection is covered by browser.acceptance.mjs.
    const text = source.querySelector(".chat-strong")!.firstChild!;
    const range = document.createRange();
    range.selectNodeContents(text);
    window.getSelection()!.removeAllRanges();
    window.getSelection()!.addRange(range);
    source.dispatchEvent(new Event("selectionchange", { bubbles: true }));
    await new Promise((resolve) => requestAnimationFrame(resolve));
    await userEvent.keyboard("{Control>}{Alt>}m{/Alt}{/Control}");
    const editor = await draft(canvasElement);
    await userEvent.type(editor, "Retained draft");
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(
        c.queryByRole("form", { name: "Inline comment composer" }),
      ).toBeNull(),
    );
    await userEvent.click(
      c.getByRole("button", {
        name: "Comment on Assistant explanation",
      }),
    );
    await expect(await draft(canvasElement)).toHaveTextContent(
      "Retained draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Cancel" }));
    source.focus();
    await userEvent.paste("Pasted comment");
    await expect(await draft(canvasElement)).toHaveTextContent(
      "Pasted comment",
    );
    await userEvent.keyboard("{Control>}{Enter}{/Control}");
    await expect(
      await c.findByRole("article", { name: "Comment by You" }),
    ).toHaveTextContent("Pasted comment");
  }}>{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Changed content retains an open draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await reader(canvasElement));
    await userEvent.keyboard("Keep this draft");
    await expect(await draft(canvasElement)).toHaveTextContent(
      "Keep this draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Replace content" }));
    await expect(
      c.getByRole("complementary", { name: "Comments on unavailable content" }),
    ).toHaveTextContent("has changed");
    await expect(await draft(canvasElement)).toHaveTextContent(
      "Keep this draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Cancel" }));
    await expect(
      c.queryByRole("form", { name: "Inline comment composer" }),
    ).toBeNull();
  }}>{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Tool comments survive rich raw switching and disclosure"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Expand Read a file, ran a command" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read example.ts" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Comment on Captured example.ts" }),
    );
    await userEvent.type(
      await draft(canvasElement, "Captured example.ts"),
      "Check this constant",
    );
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      c.getByRole("button", { name: "Close read tool details" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read example.ts" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Comment on Captured example.ts" }),
    );
    await expect(
      await draft(canvasElement, "Captured example.ts"),
    ).toHaveTextContent("Check this constant");
    await addComment(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Show raw output for Read example.ts" }),
    );
    await expect(
      await reader(canvasElement, "Raw captured data for Read example.ts"),
    ).toHaveTextContent("export const answer");
    await userEvent.click(
      c.getByRole("button", {
        name: "Comment on Raw captured data for Read example.ts",
      }),
    );
    await userEvent.type(
      await draft(canvasElement, "Raw captured data for Read example.ts"),
      "Captured raw context",
    );
    await addComment(canvasElement);
    await expect(
      c.queryByRole("complementary", {
        name: "Comments on unavailable content",
      }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Show rich output for Read example.ts" }),
    );
    await expect(
      await c.findByRole("article", { name: "Comment by You" }),
    ).toHaveTextContent("Check this constant");
    await userEvent.click(
      c.getByRole("button", { name: "Collapse Read a file, ran a command" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Expand Read a file, ran a command" }),
    );
    await expect(
      await c.findByRole("article", { name: "Comment by You" }),
    ).toHaveTextContent("Check this constant");
    await userEvent.click(
      c.getByRole("button", { name: "Comment on Decision table" }),
    );
    await userEvent.type(
      await draft(canvasElement, "Decision table"),
      "Whole section comment",
    );
    await addComment(canvasElement);
    expect(c.getAllByRole("article", { name: "Comment by You" })).toHaveLength(
      2,
    );
  }}>{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Patch selection composition and teardown preserve content"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      source = await reader(canvasElement, "Captured patch example.ts");
    source.focus();
    source.dispatchEvent(
      new CompositionEvent("compositionstart", { bubbles: true }),
    );
    source.dispatchEvent(
      new CompositionEvent("compositionend", { bubbles: true, data: "日本語" }),
    );
    await expect(
      await draft(canvasElement, "Captured patch example.ts"),
    ).toHaveTextContent("日本語");
    await expect(source).toHaveTextContent("-export const answer = 41;");
    await addComment(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Hide body" }));
    await expect(
      c.queryByRole("region", { name: "Agent chat body" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Show body" }));
    await expect(
      await c.findByRole("article", { name: "Comment by You" }),
    ).toHaveTextContent("日本語");
  }}>{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Each captured patch file collapses independently"
  play={patchFilesCollapse}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Narrow dark body with honest unavailable tool output"
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await reader(canvasElement, "Unavailable output"),
    ).toHaveTextContent("Permission denied");
    const body = c.getByRole("region", { name: "Agent chat body" });
    expect(body.scrollWidth).toBeLessThanOrEqual(body.clientWidth + 1);
  }}>{#snippet children()}<Fixture scenario="failure" narrow />{/snippet}</Story
>

<Story
  asChild
  name="App body shell read sidebar and turn navigation"
  play={shellAndTurnNavigation}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Empty captured history"
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByText("No captured messages."),
    ).toBeVisible();
  }}>{#snippet children()}<Fixture scenario="empty" />{/snippet}</Story
>
