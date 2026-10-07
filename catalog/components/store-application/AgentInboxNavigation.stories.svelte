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
  name="Directory icon actions open all sessions and expose tooltips"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const refresh = await c.findByRole("button", { name: "Refresh agents" });
    const sessions = c.getByRole("button", {
      name: "View sessions",
    });
    await expect(refresh.textContent?.trim()).toBe("");
    await expect(sessions.textContent?.trim()).toBe("");
    await expect(sessions.getBoundingClientRect().left).toBeLessThan(
      refresh.getBoundingClientRect().left,
    );
    refresh.focus();
    await expect(
      await within(document.body).findByRole("tooltip"),
    ).toHaveTextContent("Refresh agents");
    await userEvent.unhover(refresh);
    sessions.focus();
    await userEvent.keyboard("{Enter}");
    await c.findByRole("button", { name: /Open session Repository helper/ });
    await expect(c.getByText("Sessions", { exact: true })).toBeVisible();
    await expect(
      c.queryByLabelText("Session commands"),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Offline chat row uses presence and an independent exact repository link"
  args={{ status: "Offline" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Open fixture conversation" }),
    );
    const chat = await c.findByRole("button", {
      name: /Open session Repository helper/,
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
      `#/repos/${"11".repeat(16)}/files?at_op=${"22".repeat(64)}&workspace=agent-${"44".repeat(16)}`,
    );
    await expect(chat.contains(link)).toBe(false);
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
