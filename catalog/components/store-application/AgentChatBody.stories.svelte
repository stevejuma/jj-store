<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreAgentChatBody from "../../../web/src/lib/ui/store-agent-chat-body/StoreAgentChatBody.svelte";
  import Fixture from "../../fixtures/ChatBodyFixture.svelte";
  import {
    reader,
    draft,
    addComment,
    inlineComments,
  } from "../../fixtures/delta-chat/acceptance-helpers.js";
  import {
    patchFilesCollapse,
    shellAndTurnNavigation,
  } from "../../fixtures/delta-chat/acceptance-plays.js";
  import { chatBodyExample } from "../../fixtures/delta-chat/example-source.js";
  import { largeCaptures } from "../../fixtures/delta-chat/large-capture-play.js";
  import { messageActions } from "../../fixtures/delta-chat/message-action-play.js";
  import { nextMessagePlay } from "../../fixtures/delta-chat/next-message-play.js";
  import {
    commentLifecycle,
    unavailableDraft,
    retainedDraft,
  } from "../../fixtures/delta-chat/comment-lifecycle-play.js";
  import {
    collapsedComments,
    deltaCommentReference,
  } from "../../fixtures/delta-chat/comment-navigation-play.js";
  import { toolSpacing } from "../../fixtures/delta-chat/spacing-play.js";
  import { readPanelResize } from "../../fixtures/delta-chat/read-panel-resize-play.js";
  import { numberedCodeGutter } from "../../fixtures/delta-chat/numbered-code-gutter-play.js";
  import { footerRuntime } from "../../fixtures/delta-chat/footer-runtime-play.js";
  import {
    groupedCommentEntries,
    commentRail,
  } from "../../fixtures/delta-chat/comment-followup-play.js";
  const { Story } = defineMeta({
    title: "JJ Store/Commentable agent chat body",
    component: StoreAgentChatBody,
    tags: ["visual-pending"],
    args: { items: [] },
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: {
        source: {
          language: "svelte",
          code: chatBodyExample,
        },
      },
    },
  });
</script>

<Story asChild name="Footer runtime controls" play={footerRuntime}>
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>

<Story asChild name="Captured line number gutter" play={numberedCodeGutter}>
  {#snippet children()}<Fixture />{/snippet}
</Story>

<Story asChild name="Resizable read sidebar" play={readPanelResize}>
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>

<Story
  asChild
  name="Grouped comments and message links"
  play={groupedCommentEntries}
>
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>
<Story asChild name="Comment navigation previews" play={commentRail}>
  {#snippet children()}<Fixture scenario="comments" />{/snippet}
</Story>
<Story asChild name="Delta comments reference" play={deltaCommentReference}>
  {#snippet children()}<Fixture scenario="comments" />{/snippet}
</Story>
<Story asChild name="Consistent tool flow spacing" play={toolSpacing}>
  {#snippet children()}<Fixture scenario="playground" />{/snippet}
</Story>
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
    await userEvent.click(
      c.getByRole("button", { name: "Show details for Read README.md" }),
    );
    const readme = await reader(canvasElement, "README.md");
    const heading = () =>
      Array.from(readme.querySelectorAll<HTMLElement>(".cm-heading")).find(
        (node) => node.textContent?.includes("Bedtime"),
      );
    await waitFor(() => expect(heading()).toHaveTextContent("Bedtime"));
    expect(getComputedStyle(heading()!).color).not.toBe(
      getComputedStyle(readme).color,
    );
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>

<Story
  asChild
  name="Typing anchors a discussion and preserves the captured text"
  play={commentLifecycle}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Keyboard selection paste and Escape retain an exact draft"
  play={retainedDraft}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Changed content retains an open draft"
  play={unavailableDraft}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
>

<Story
  asChild
  name="Tool comments survive rich raw switching and disclosure"
  play={collapsedComments}
  >{#snippet children()}<Fixture scenario="playground" />{/snippet}</Story
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
    const commentEditor = await draft(
      canvasElement,
      "Captured patch example.ts",
    );
    const nextEditor = c.getByRole("textbox", { name: "Next message" });
    expect(getComputedStyle(commentEditor).fontFamily).toBe(
      getComputedStyle(nextEditor).fontFamily,
    );
    expect(getComputedStyle(commentEditor).paddingLeft).toBe("0px");
    await addComment(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Hide body" }));
    await expect(
      c.queryByRole("region", { name: "Agent chat body" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Show body" }));
    await expect(
      await inlineComments(canvasElement).findByRole("article", {
        name: "Comment by You",
      }),
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
