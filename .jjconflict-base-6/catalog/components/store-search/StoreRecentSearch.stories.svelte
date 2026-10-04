<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within } from "storybook/test";
  import StoreSearchCommand from "../../../web/src/lib/ui/store-search/StoreSearchCommand.svelte";
  import { NOW } from "../../fixtures/repository.js";
  const { Story } = defineMeta({
    title: "JJ Store/Search/Recent activity",
    component: StoreSearchCommand,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Context scope and bottom actions"
  args={{
    open: true,
    query: "",
    search: {
      available: true,
      query: "",
      results: [],
      status: "ready",
      error: "",
      isStale: false,
      degraded: false,
      truncated: false,
      canLoadMore: false,
    },
    now: NOW,
    rows: Array.from({ length: 6 }, (_, n) => ({
      id: String(n),
      kind: "thread" as const,
      title: `Thread ${n}`,
      description: "Recent message",
      source: "Home",
      timestamp: NOW - n * 1000,
    })),
    status: "ready",
    scopeLabel: "general",
    onScope: fn(),
    onSelectRow: fn(),
    onQueryChange: fn(),
    onSelect: fn(),
    onLoadMore: fn(),
    actions: [
      {
        id: "create-thread",
        label: "Create new thread",
        create: true,
        run: fn(),
      },
    ],
  }}
  play={async ({ canvasElement, args }) => {
    const body = within(canvasElement.ownerDocument.body);
    const dialog = body.getByRole("dialog", { name: "Search workspace" });
    const c = within(dialog);
    await expect(
      c.getByText("Recent activity", {
        selector: "[data-command-group-heading]",
      }),
    ).toBeVisible();
    await expect(c.getAllByRole("option", { name: /Thread \d/ })).toHaveLength(
      4,
    );
    await userEvent.click(c.getByRole("option", { name: /Search in general/ }));
    await expect(args.onScope).toHaveBeenCalledOnce();
    await userEvent.click(c.getByRole("option", { name: /Thread 0/ }));
    await expect(args.onSelectRow).toHaveBeenCalledWith("0");
    await userEvent.click(c.getByRole("option", { name: /Create new thread/ }));
    await expect(args.actions![0]!.run).toHaveBeenCalledOnce();
  }}
/>
