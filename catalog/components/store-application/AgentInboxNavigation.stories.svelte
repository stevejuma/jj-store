<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import SkeletonFixture from "../../fixtures/AgentsSkeletonFixture.svelte";
  import Fixture from "../../fixtures/AgentExecutionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent inbox navigation",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Navigation reuses settings feedback and transcript without another stream"
  args={{ observeRuntime: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const counts = () =>
      JSON.parse(c.getByTestId("inbox-read-counts").textContent ?? "{}");
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByRole("heading", {
      name: "Fixture conversation",
    });
    await waitFor(() =>
      expect(
        c.queryByRole("button", {
          name: "Loading previous messages",
        }),
      ).not.toBeInTheDocument(),
    );
    const initial = counts();
    await expect(initial["/v1/local/agents/settings"]).toBe(1);
    await expect(initial["/v1/local/agents/targets"]).toBe(1);
    await expect(initial["/v1/local/agents"]).toBe(1);
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    await c.findByRole("button", { name: "View sessions" });
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    await c.findByRole("heading", {
      name: "Fixture conversation",
    });
    await expect(counts()).toEqual(initial);
    await expect(
      Object.keys(counts()).some((path) => path.includes("/stream?")),
    ).toBe(false);
  }}
/>

<Story
  name="Directory session tile precedes Add agent and Refresh exposes its tooltip"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const refresh = await c.findByRole("button", { name: "Refresh agents" });
    await c.findByRole("button", { name: "View Repository helper" });
    const sessions = c.getByRole("button", {
      name: "View sessions",
    });
    await expect(refresh.textContent?.trim()).toBe("");
    await expect(sessions.textContent?.trim()).toBe("View sessions");
    await expect(sessions.querySelector("svg")).not.toBeNull();
    const add = c.getByRole("button", { name: "Add agent" });
    await expect(sessions.getBoundingClientRect().left).toBeLessThan(
      add.getBoundingClientRect().left,
    );
    refresh.focus();
    await expect(
      await within(document.body).findByRole("tooltip"),
    ).toHaveTextContent("Refresh agents");
    await userEvent.unhover(refresh);
    sessions.focus();
    await userEvent.keyboard("{Enter}");
    await c.findByRole("button", { name: /Open session Fixture conversation/ });
    await expect(c.getByText("Sessions", { exact: true })).toBeVisible();
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Offline chat row uses presence and a separate repository header action"
  args={{ status: "Offline" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const chat = await c.findByRole("button", {
      name: /Open session Fixture conversation/,
    });
    await expect(chat).not.toHaveTextContent("Offline");
    await expect(
      chat.querySelector('[data-ui-part="presence"]'),
    ).toHaveAttribute("data-presence", "offline");
    const link = c.getByRole("link", {
      name: "Open Fixture repository",
    });
    await expect(link).toHaveAttribute(
      "href",
      `#/repos/${"11".repeat(16)}/files`,
    );
    await expect(chat.contains(link)).toBe(false);
    await expect(
      chat.closest(".inbox-session-tree__children")?.querySelector("a"),
    ).toBeNull();
    await expect(link.closest(".inbox-session-tree__header")).not.toBeNull();
    await expect(link.textContent?.trim()).toBe("");
    link.focus();
    await expect(
      await within(document.body).findByRole("tooltip"),
    ).toHaveTextContent("Open Fixture repository");
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Inbox chat list resizes with keyboard and retains width while hidden"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const list = await c.findByRole("complementary", { name: "Session chats" });
    const handle = c.getByRole("button", { name: "Resize session chats" });
    const initial = list.getBoundingClientRect().width;
    handle.focus();
    await userEvent.keyboard("{Home}");
    await waitFor(() => expect(list.getBoundingClientRect().width).toBe(224));
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(() => expect(list.getBoundingClientRect().width).toBe(240));
    await userEvent.click(
      c.getByRole("button", { name: "Toggle session chats" }),
    );
    await expect(
      c.queryByRole("button", { name: "Resize session chats" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      c.getByRole("button", { name: "Toggle session chats" }),
    );
    await waitFor(() => expect(list.getBoundingClientRect().width).toBe(240));
    await userEvent.click(
      c.getByRole("button", { name: "Manage fixture agents" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const restoredList = await c.findByRole("complementary", {
      name: "Session chats",
    });
    await waitFor(() =>
      expect(restoredList.getBoundingClientRect().width).toBe(240),
    );
    await userEvent.dblClick(
      c.getByRole("button", { name: "Resize session chats" }),
    );
    await waitFor(() =>
      expect(restoredList.getBoundingClientRect().width).toBe(initial),
    );
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>
