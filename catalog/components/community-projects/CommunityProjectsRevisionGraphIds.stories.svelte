<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import CommunityProjectsRevisionGraphIds from "../../../web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import { IDS } from "../../fixtures/repository.js";
  const demoCommitId = `565656b3e4a1${"a".repeat(26)}c9`;
  const demoChangeId = `ututut${"v".repeat(24)}ut`;
  const { Story } = defineMeta({
    title: "JJ Store/Components/Native IDs",
    component: CommunityProjectsRevisionGraphIds,
    tags: ["visual-pending", "component"],
    args: {
      commitId: demoCommitId,
      changeId: demoChangeId,
      placement: "inline",
    },
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte",
      },
      docs: {
        source: {
          code: '<script>\n  import CommunityProjectsRevisionGraphIds from "./web/src/lib/ui/community-projects/CommunityProjectsRevisionGraphIds.svelte";\n<\/script>\n<CommunityProjectsRevisionGraphIds {commitId} {changeId} compactLeadingChars={4} expandedLeadingChars={5} placement="inline" />',
        },
      },
    },
  });

  async function withClipboard(
    run: (write: ReturnType<typeof fn>) => Promise<void>,
  ) {
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await run(write);
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }
</script>

<Story name="Default" />
<Story
  name="Copy complete identity"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const body = within(canvasElement.ownerDocument.body);
    await withClipboard(async (write) => {
      const commit = canvas.getByRole("button", {
        name: `Copy Git commit SHA ${demoCommitId}`,
      });
      const change = canvas.getByRole("button", {
        name: `Copy JJ Change ID ${demoChangeId}`,
      });
      const split = canvas.getByRole("group", {
        name: "Native revision identifiers",
      });
      await expect(within(commit).getByText("GIT")).toBeVisible();
      await expect(within(change).getByText("JJ")).toBeVisible();
      await expect(within(commit).getByText("5656")).toBeVisible();
      await expect(within(change).getByText("utut")).toBeVisible();
      await expect(split.querySelectorAll("svg")).toHaveLength(0);
      const width = split.getBoundingClientRect().width;
      const before = commit.getBoundingClientRect().width;
      await userEvent.hover(commit);
      await expect(await body.findByText(demoCommitId)).toBeVisible();
      await userEvent.tab();
      await expect(within(commit).getByText("56565..c9")).toBeVisible();
      await expect(within(change).getByText("JJ")).toBeVisible();
      await expect(
        within(change).getByText("utut", { exact: true }),
      ).not.toBeVisible();
      await waitFor(() =>
        expect(commit.getBoundingClientRect().width).toBeGreaterThan(
          before + 10,
        ),
      );
      await expect(split.getBoundingClientRect().width).toBe(width);
      await userEvent.click(commit);
      await expect(write).toHaveBeenLastCalledWith(demoCommitId);
      await expect(
        within(commit).getByText("Copied", { exact: true }),
      ).toBeVisible();
      await userEvent.tab();
      await expect(within(change).getByText("ututu..ut")).toBeVisible();
      await expect(split.getBoundingClientRect().width).toBe(width);
      await userEvent.click(change);
      await expect(write).toHaveBeenLastCalledWith(demoChangeId);
      await expect(
        within(change).getByText("Copied", { exact: true }),
      ).toBeVisible();
      await expect(commit).toHaveAttribute("data-copied", "true");
      await waitFor(() => expect(commit).not.toHaveAttribute("data-copied"), {
        timeout: 2000,
      });
      await waitFor(() => expect(change).not.toHaveAttribute("data-copied"), {
        timeout: 2000,
      });
      await expect(split.getBoundingClientRect().width).toBe(width);
      await userEvent.unhover(change);
    });
  }}
/>
<Story
  name="Keyboard copy"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await withClipboard(async (write) => {
      const commit = canvas.getByRole("button", {
        name: `Copy Git commit SHA ${demoCommitId}`,
      });
      const change = canvas.getByRole("button", {
        name: `Copy JJ Change ID ${demoChangeId}`,
      });
      await userEvent.tab();
      await expect(commit).toHaveFocus();
      await expect(within(commit).getByText("56565..c9")).toBeVisible();
      await userEvent.keyboard("{Enter}");
      await expect(write).toHaveBeenLastCalledWith(demoCommitId);
      await userEvent.tab();
      await expect(change).toHaveFocus();
      await expect(within(change).getByText("ututu..ut")).toBeVisible();
      await userEvent.keyboard(" ");
      await expect(write).toHaveBeenLastCalledWith(demoChangeId);
      await expect(
        within(change).getByText("Copied", { exact: true }),
      ).toBeVisible();
    });
  }}
/>
<Story
  name="Git only"
  args={{ changeId: "" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await withClipboard(async (write) => {
      await expect(canvas.getAllByRole("button")).toHaveLength(1);
      const button = canvas.getByRole("button", {
        name: `Copy Git commit SHA ${demoCommitId}`,
      });
      await expect(
        within(button).getByText("56565..c9", { exact: true }),
      ).toBeVisible();
      await userEvent.click(button);
      await expect(write).toHaveBeenLastCalledWith(demoCommitId);
      await expect(
        within(button).getByText("Copied", { exact: true }),
      ).toBeVisible();
    });
  }}
/>
<Story
  name="JJ only"
  args={{ commitId: "" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await withClipboard(async (write) => {
      await expect(canvas.getAllByRole("button")).toHaveLength(1);
      const button = canvas.getByRole("button", {
        name: `Copy JJ Change ID ${demoChangeId}`,
      });
      await expect(
        within(button).getByText("ututu..ut", { exact: true }),
      ).toBeVisible();
      await userEvent.click(button);
      await expect(write).toHaveBeenLastCalledWith(demoChangeId);
    });
  }}
/>
<Story
  name="No identifiers"
  args={{ commitId: "", changeId: "" }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).queryByRole("button"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Custom abbreviation"
  args={{ compactLeadingChars: 5, expandedLeadingChars: 12 }}
  play={async ({ canvasElement }) => {
    const commit = within(canvasElement).getByRole("button", {
      name: `Copy Git commit SHA ${demoCommitId}`,
    });
    await expect(
      within(commit).getByText("56565", { exact: true }),
    ).toBeVisible();
    await userEvent.tab();
    await expect(within(commit).getByText("565656b3e4a1..c9")).toBeVisible();
    await userEvent.unhover(commit);
  }}
/>
<Story
  name="Clipboard failure"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await withClipboard(async (write) => {
      write.mockRejectedValueOnce(new Error("Clipboard denied"));
      const commit = canvas.getByRole("button", {
        name: `Copy Git commit SHA ${demoCommitId}`,
      });
      await userEvent.click(commit);
      await expect(
        within(commit).getByText("Retry", { exact: true }),
      ).toBeVisible();
      await expect(canvas.getByRole("status")).toHaveTextContent(
        "Could not copy Git commit SHA",
      );
      await userEvent.click(commit);
      await expect(
        within(commit).getByText("Copied", { exact: true }),
      ).toBeVisible();
    });
  }}
/>
<Story
  name="Operation identity"
  args={{ changeId: "", commitId: IDS.opLeft, commitLabel: "operation ID" }}
/>
