<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/AgentInboxTreeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent inbox repository tree",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Repository groups retain exact children and independent links"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const nav = c.getByRole("navigation", {
      name: "Session chats by repository",
    });
    const groups = within(nav).getAllByRole("group", {
      name: "Sample repository sessions",
    });
    expect(groups).toHaveLength(2);
    const first = groups.find(
      (group) => group.getAttribute("data-repository-id") === "repo-a",
    )!;
    const other = groups.find(
      (group) => group.getAttribute("data-repository-id") === "repo-b",
    )!;
    const chats = within(first).getAllByRole("button", {
      name: /Open session/,
    });
    expect(chats).toHaveLength(2);
    expect(
      within(other).getAllByRole("button", { name: /Open session/ }),
    ).toHaveLength(1);
    expect(chats[0]).toHaveAttribute("aria-pressed", "true");
    const toggle = within(first).getByRole("button", {
      name: "Collapse Sample repository sessions",
    });
    expect(toggle).toHaveAttribute("aria-expanded", "true");
    expect(chats[0].getBoundingClientRect().left).toBeGreaterThan(
      toggle.getBoundingClientRect().left,
    );
    expect(chats[0].getBoundingClientRect().height).toBeLessThanOrEqual(64);
    const links = within(first).getAllByRole("link", {
      name: "Open Sample repository",
    });
    expect(links[1]).toHaveAttribute(
      "href",
      "#/repos/repo-a/files?at_op=other-operation&workspace=retained-workspace",
    );
    expect(chats[1].contains(links[1])).toBe(false);
    await userEvent.click(chats[1]);
    expect(c.getByText("Selected session: second")).toBeVisible();
    expect(chats[1]).toHaveAttribute("aria-pressed", "true");
    expect(c.getByRole("textbox", { name: "Conversation draft" })).toHaveValue(
      "Retained conversation draft",
    );
  }}
/>

<Story
  name="Collapse survives refresh and exact selection reveals the group"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const groups = c.getAllByRole("group", {
      name: "Sample repository sessions",
    });
    const first = groups.find(
      (group) => group.getAttribute("data-repository-id") === "repo-a",
    )!;
    const toggle = within(first).getByRole("button", {
      name: "Collapse Sample repository sessions",
    });
    toggle.focus();
    await userEvent.keyboard("{Enter}");
    expect(toggle).toHaveAttribute("aria-expanded", "false");
    await waitFor(() =>
      expect(
        c.queryByRole("button", { name: /Open session First chat/ }),
      ).toBeNull(),
    );
    expect(c.getByText("Selected session: first")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Toggle session chats" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle session chats" }),
    );
    expect(
      c.getByRole("button", { name: "Expand Sample repository sessions" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      c.getByRole("button", { name: "Hide loaded sessions" }),
    );
    expect(c.getByText("No sessions yet", { exact: true })).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Restore loaded sessions" }),
    );
    expect(
      c.getByRole("button", { name: "Expand Sample repository sessions" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      c.getByRole("button", { name: "Refresh and rename repository" }),
    );
    const renamed = c.getByRole("button", {
      name: "Expand Renamed repository sessions",
    });
    expect(renamed).toHaveAttribute("aria-expanded", "false");
    await waitFor(() =>
      expect(
        c.queryByRole("button", { name: /Open session First chat/ }),
      ).toBeNull(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Select other repository externally" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Select first chat externally" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Collapse Renamed repository sessions" }),
      ).toHaveAttribute("aria-expanded", "true"),
    );
    expect(
      c.getByRole("button", { name: /Open session First chat/ }),
    ).toHaveAttribute("aria-pressed", "true");
    expect(c.getByRole("textbox", { name: "Conversation draft" })).toHaveValue(
      "Retained conversation draft",
    );
  }}
/>

<Story
  name="Repository names truncate and expand with the resized sidebar"
  args={{
    repositoryName:
      "Sample repository with a long name that must fit within the session sidebar",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const list = c.getByRole("complementary", { name: "Session chats" });
    const handle = c.getByRole("button", { name: "Resize session chats" });
    handle.focus();
    await userEvent.keyboard("{Home}");
    await waitFor(() => expect(list.getBoundingClientRect().width).toBe(224));
    const group = list.querySelector('[data-repository-id="repo-a"]')!;
    const row = within(group as HTMLElement).getByRole("button", {
      name: /Collapse Sample repository/,
    });
    const label = row.querySelector("span")!;
    expect(label.scrollWidth).toBeGreaterThan(label.clientWidth);
    expect(getComputedStyle(label).textOverflow).toBe("ellipsis");
    expect(getComputedStyle(label).overflowX).toBe("hidden");
    expect(row).toHaveAttribute(
      "title",
      "Sample repository with a long name that must fit within the session sidebar",
    );
    expect(row.getBoundingClientRect().right).toBeLessThanOrEqual(
      list.getBoundingClientRect().right,
    );
    const narrowWidth = label.clientWidth;
    handle.focus();
    await userEvent.keyboard("{End}");
    await waitFor(() => expect(list.getBoundingClientRect().width).toBe(480));
    expect(label.clientWidth).toBeGreaterThan(narrowWidth);
    expect(row.getBoundingClientRect().right).toBeLessThanOrEqual(
      list.getBoundingClientRect().right,
    );
    expect(c.getByRole("textbox", { name: "Conversation draft" })).toHaveValue(
      "Retained conversation draft",
    );
  }}
/>

<Story
  name="Unbound drafts stay visible without an invented repository"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const group = c.getByRole("group", { name: "No repository sessions" });
    expect(within(group).queryByRole("link")).toBeNull();
    const draft = within(group).getByRole("button", {
      name: /Open session New chat/,
    });
    draft.focus();
    await userEvent.keyboard(" ");
    expect(c.getByText("Selected session: draft")).toBeVisible();
    expect(draft).toHaveAttribute("aria-pressed", "true");
  }}
/>
