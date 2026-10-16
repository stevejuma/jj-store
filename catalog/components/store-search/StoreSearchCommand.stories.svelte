<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within, waitFor } from "storybook/test";
  import StoreSearchCommand from "../../../web/src/lib/ui/store-search/StoreSearchCommand.svelte";
  import StoreSearchCommandFixture from "../../fixtures/StoreSearchCommandFixture.svelte";
  import { searchResults } from "../../fixtures/search.js";
  import { NOW } from "../../fixtures/repository.js";

  const ready = {
    available: true,
    query: "native",
    results: searchResults,
    status: "ready" as const,
    error: "",
    canLoadMore: true,
    isStale: true,
    degraded: false,
    truncated: false,
  };
  const { Story } = defineMeta({
    title: "JJ Store/Search/Repository search command",
    component: StoreSearchCommand,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Results"
  args={{
    open: true,
    query: "native",
    search: ready,
    now: NOW,
    onQueryChange: fn(),
    onSelect: fn(),
    onLoadMore: fn(),
  }}
  play={async ({ canvasElement, args }) => {
    const page = within(canvasElement.ownerDocument.body);
    const dialog = page.getByRole("dialog", { name: "Search workspace" });
    await expect(
      dialog.querySelector('[data-ui-component="command-view"]'),
    ).not.toBeNull();
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Operations", {
          selector: "[data-command-group-heading]",
        }),
      ).toBeVisible();
    });
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Commits", {
          selector: "[data-command-group-heading]",
        }),
      ).toBeVisible();
    });
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Files", {
          selector: "[data-command-group-heading]",
        }),
      ).toBeVisible();
    });
    await waitFor(async () => {
      await expect(
        within(dialog).getByText("Indexing newer history"),
      ).toBeVisible();
    });
    const commit = within(dialog).getByRole("option", {
      name: /Stream immutable native objects/u,
    });
    await userEvent.click(commit);
    await expect(args.onSelect).toHaveBeenCalledWith(
      expect.objectContaining({ kind: "commit" }),
    );
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Load more results" }),
    );
    await expect(args.onLoadMore).toHaveBeenCalledOnce();
  }}
>
  {#snippet template(args)}
    <StoreSearchCommandFixture {args} />
  {/snippet}
</Story>

<Story
  name="Loading"
  args={{
    open: true,
    query: "layout",
    search: {
      ...ready,
      query: "layout",
      results: [],
      status: "loading",
      canLoadMore: false,
      isStale: false,
    },
    now: NOW,
    onQueryChange: fn(),
    onSelect: fn(),
    onLoadMore: fn(),
  }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await expect(page.getByRole("status")).toHaveTextContent("Searching…");
  }}
>
  {#snippet template(args)}
    <StoreSearchCommandFixture {args} />
  {/snippet}
</Story>

<Story
  name="Unavailable"
  args={{
    open: true,
    query: "",
    search: {
      ...ready,
      available: false,
      query: "",
      results: [],
      status: "unavailable",
      error: "Search is not configured for this repository.",
      canLoadMore: false,
      isStale: false,
    },
    now: NOW,
    onQueryChange: fn(),
    onSelect: fn(),
    onLoadMore: fn(),
  }}
  play={async ({ canvasElement }) => {
    const page = within(canvasElement.ownerDocument.body);
    await expect(await page.findByRole("alert")).toHaveTextContent(
      "Search is not configured",
    );
    await userEvent.click(page.getByRole("button", { name: "Close search" }));
    await expect(
      canvasElement.querySelector("[data-search-open]"),
    ).toHaveAttribute("data-search-open", "false");
  }}
>
  {#snippet template(args)}
    <StoreSearchCommandFixture {args} />
  {/snippet}
</Story>
