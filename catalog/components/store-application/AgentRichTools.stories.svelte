<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentRichToolsFixture.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-agent-activity/CommunityAgentTranscriptTools.svelte",
      },
    },
    title: "JJ Store/Rich agent tools",
    component: Fixture,
    tags: ["visual-pending", "component"],
  });
  async function open(canvasElement: HTMLElement, name: string) {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: `Show details for ${name}` }),
    );
    return within(c.getByRole("region", { name }));
  }
</script>

<Story
  name="Captured read content highlights and toggles the complete raw snapshot"
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement, "Read example.ts");
    await expect(c.getByRole("group", { name: "typescript" })).toBeVisible();
    await expect(c.getByText(/export const answer = 42/)).toBeVisible();
    const highlighted = c.getByRole("group", {
      name: "typescript",
    });
    await waitFor(() => {
      const registry = (
        CSS as unknown as { highlights: Map<string, Set<Range>> }
      ).highlights;
      expect(
        [...registry.values()].some((h) =>
          [...h].some((r) => highlighted.contains(r.startContainer)),
        ),
      ).toBe(true);
    });
    await expect(canvasElement.querySelector("script")).toBeNull();
    const toggle = c.getByRole("button", { name: "Show raw output" });
    toggle.focus();
    await userEvent.keyboard("{Enter}");
    await expect(c.getByText(/formatted_output/)).toBeVisible();
    await expect(c.getByRole("group", { name: "json" })).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Show rich output" }));
    await expect(c.getByRole("group", { name: "typescript" })).toBeVisible();
  }}
/>
<Story
  name="Edit diff and JSON view remain independent across collapse"
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement, "Edit example.ts");
    await expect(
      c.getByRole("region", { name: "Diff for src/example.ts" }),
    ).toBeVisible();
    await expect(c.getByLabelText("1 additions and 1 deletions")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Show raw output" }));
    await expect(c.getByText(/oldText/)).toBeVisible();
    const page = within(canvasElement);
    await userEvent.click(
      page.getByRole("button", {
        name: "Hide details for Edit example.ts",
      }),
    );
    await userEvent.click(
      page.getByRole("button", {
        name: "Show details for Edit example.ts",
      }),
    );
    await expect(
      c.getByRole("button", { name: "Show rich output" }),
    ).toBeVisible();
    const read = await open(canvasElement, "Read example.ts");
    await expect(
      read.getByRole("button", { name: "Show raw output" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Commands searches failures and unsupported tools retain their evidence"
  play={async ({ canvasElement }) => {
    const shell = await open(canvasElement, "Command");
    await expect(shell.getByText(/7 tests passed/)).toBeVisible();
    const search = await open(canvasElement, "Search");
    await expect(
      search.getByRole("region", { name: "Search results" }),
    ).toBeVisible();
    await expect(
      search.getByText("src/example.ts:1", { exact: true }),
    ).toBeVisible();
    const failed = await open(canvasElement, "Failed read");
    await expect(
      failed.getByText("Permission denied", { exact: true }),
    ).toBeVisible();
    const unknown = await open(canvasElement, "Unsupported");
    await userEvent.click(
      unknown.getByRole("button", { name: "Show raw output" }),
    );
    await expect(unknown.getByText(/untrusted.png/)).toBeVisible();
    await expect(unknown.queryByRole("img")).toBeNull();
  }}
/>
<Story
  name="Partial edit discloses missing preimages and truncation"
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement, "Partial edit");
    await expect(
      c.getByText(
        "Only one side was captured; a complete diff is unavailable.",
      ),
    ).toBeVisible();
    await expect(c.getByText("Output was truncated.")).toBeVisible();
    await expect(c.getByRole("group", { name: "rust" })).toBeVisible();
    await expect(c.queryByLabelText(/additions and/)).toBeNull();
  }}
/>
<Story
  name="Recorded agent tools use the same read and edit presentation"
  args={{ recorded: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      read = within(c.getByRole("region", { name: "Read example.ts" })),
      edit = within(c.getByRole("region", { name: "Edit example.ts" }));
    await expect(read.getByRole("group", { name: "typescript" })).toBeVisible();
    await expect(
      edit.getByRole("region", { name: "Diff for src/example.ts" }),
    ).toBeVisible();
    await userEvent.click(
      read.getByRole("button", { name: "Show raw output" }),
    );
    await expect(read.getByText(/native_name/)).toBeVisible();
    await expect(
      edit.getByRole("button", { name: "Show raw output" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Tool rows use the full available container width"
  play={async ({ canvasElement }) => {
    const roots = Array.from(
      canvasElement.querySelectorAll(
        '[data-ui-component="ai-chat-tool-calls"]',
      ),
    );
    expect(roots).toHaveLength(7);
    for (const root of roots) {
      expect(root.parentElement!.getBoundingClientRect().width).toBeGreaterThan(
        672,
      );
      expect(root.getBoundingClientRect().width).toBeCloseTo(
        root.parentElement!.getBoundingClientRect().width,
        0,
      );
    }
  }}
/>
