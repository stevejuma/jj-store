<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import StoreSearchCommand from "../../../web/src/lib/ui/store-search/StoreSearchCommand.svelte";
  import { NOW } from "../../fixtures/repository.js";
  const typedSections = [
    { id: "scope:threads", label: "Search threads", kind: "thread" as const },
    {
      id: "scope:projects",
      label: "Search projects",
      kind: "project" as const,
    },
    { id: "scope:tasks", label: "Search tasks", kind: "task" as const },
    { id: "scope:code", label: "Search code", kind: "file" as const },
  ].map((section) => ({ ...section, run: fn() }));
  const typedActions = [
    { id: "browse-threads", label: "Browse threads", kind: "thread" as const },
    {
      id: "browse-projects",
      label: "Browse projects",
      kind: "project" as const,
    },
    { id: "browse-tasks", label: "Browse tasks", kind: "task" as const },
    {
      id: "browse-repositories",
      label: "Browse repositories",
      kind: "repository" as const,
    },
    {
      id: "create-thread",
      label: "Create new thread",
      kind: "thread" as const,
      create: true,
    },
    {
      id: "create-project",
      label: "Create new project",
      kind: "project" as const,
      create: true,
    },
    {
      id: "create-task",
      label: "Create new task",
      kind: "task" as const,
      create: true,
    },
  ].map((action) => ({ ...action, run: fn() }));
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

<Story
  name="Typed search and action icons"
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
    scopeLabel: "general",
    scopeKind: "thread",
    onScope: fn(),
    onQueryChange: fn(),
    onSelect: fn(),
    onLoadMore: fn(),
    sections: typedSections,
    actions: typedActions,
  }}
  play={async ({ canvasElement, args }) => {
    const page = within(canvasElement.ownerDocument.body);
    const dialog = page.getByRole("dialog", { name: "Search workspace" });
    const c = within(dialog);
    const glyphs = {
      thread: "hash",
      project: "folder-kanban",
      task: "circle-dot",
      repository: "folder-git-2",
      file: "file-code-2",
    };
    const scope = c.getByRole("option", { name: /Search in general/ });
    await expect(scope.querySelector("svg.lucide-hash")).not.toBeNull();
    const input = c.getByRole("combobox", { name: "Search workspace" });
    input.focus();
    await userEvent.keyboard("{End}{Home}");
    await waitFor(() => expect(scope).toHaveAttribute("aria-selected", "true"));
    await userEvent.keyboard("{Enter}");
    await expect(args.onScope).toHaveBeenCalledOnce();
    await waitFor(() => expect(input).toHaveFocus());
    for (const row of [...typedSections, ...typedActions]) {
      const option = c.getByRole("option", { name: row.label });
      const icon = option.querySelector(
        ".community-search-command__browse-icon",
      )!;
      await expect(icon).not.toBeNull();
      await expect(icon).toHaveAttribute("aria-hidden", "true");
      await expect(
        icon.querySelector(`svg.lucide-${glyphs[row.kind]}`),
      ).not.toBeNull();
      const badge = icon.querySelector(
        `svg.lucide-${"create" in row && row.create ? "plus" : "search"}`,
      )!;
      await expect(badge).not.toBeNull();
      const mainRect = icon.firstElementChild!.getBoundingClientRect();
      const badgeRect = badge.getBoundingClientRect();
      await expect(badgeRect.width).toBeLessThan(mainRect.width);
      await expect(badgeRect.right).toBeGreaterThan(mainRect.right);
      await expect(badgeRect.bottom).toBeGreaterThan(mainRect.bottom);
      await userEvent.click(option);
      await expect(row.run).toHaveBeenCalledOnce();
    }
  }}
/>
