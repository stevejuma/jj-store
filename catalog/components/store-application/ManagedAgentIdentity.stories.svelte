<script module lang="ts">
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ManagedAgentRepositoryFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Managed agent identity",
    component: Fixture,
    beforeEach: async () => {
      await Promise.all([views.agentsRecorded(), views.agentSidebar()]);
    },
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="Repository agent actions live in the Workspace title row with tooltips"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    const header = canvasElement.querySelector<HTMLElement>(
      'header[aria-label="Workspace section header"]',
    )!;
    await expect(header.querySelector("svg.lucide-folder-root")).not.toBeNull();
    for (const name of ["New agent session", "Manage agents"]) {
      const action = c.getByRole("button", { name });
      await expect(header).toContainElement(action);
      await expect(action.textContent?.trim()).toBe("");
      await userEvent.hover(action);
      await expect(await portal.findByRole("tooltip")).toHaveTextContent(name);
      await userEvent.unhover(action);
      await userEvent.keyboard("{Escape}");
    }
    await expect(c.getByLabelText("Execution commands")).toHaveTextContent("0");
  }}
/>

<Story
  name="Recorded identity, last activity and selected response survive deleted setup"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const row = await c.findByRole("button", {
      name: "Open Nyra ended agent session",
    });
    const item = row.closest(".store-agent-session-item")!;
    await expect(
      item.querySelector('[data-ui-part="emoji"]'),
    ).toHaveTextContent("🐝");
    const last = within(item as HTMLElement).getByLabelText(/^Last activity:/);
    await expect(last).toBeVisible();
    const state = within(item as HTMLElement).getByText("ended", {
      exact: true,
    });
    await expect(
      last.compareDocumentPosition(state) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).toBeTruthy();
    await userEvent.click(row);
    const response = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await waitFor(() =>
      expect(
        response.querySelector('[data-ui-part="emoji"]'),
      ).toHaveTextContent("🐝"),
    );
    await expect(c.getByLabelText("Execution commands")).toHaveTextContent("0");
  }}
/>

<Story
  name="Workspace choices and selected trigger use the captured avatar"
  args={{ page: "files" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    const trigger = c.getByRole("combobox", { name: "Repository workspace" });
    await userEvent.click(trigger);
    const item = await portal.findByRole("option", {
      name: /exact-agent-workspace/,
    });
    await expect(
      item.querySelector('[data-ui-part="emoji"]'),
    ).toHaveTextContent("🐝");
    await userEvent.click(item);
    await expect(trigger).toHaveTextContent("exact-agent-workspace");
    await expect(
      trigger.querySelector('[data-ui-part="emoji"]'),
    ).toHaveTextContent("🐝");
    await userEvent.click(trigger);
    const selected = await portal.findByRole("option", {
      name: /exact-agent-workspace/,
    });
    await expect(
      selected.querySelector('[aria-label="Selected workspace"] svg'),
    ).not.toBeNull();
    await userEvent.keyboard("{Escape}");
    await expect(c.getByLabelText("Execution commands")).toHaveTextContent("0");
  }}
/>

<Story
  name="Unmatched Controller sessions retain their integration identity"
  args={{ unmatched: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", {
      name: "Open Controller ended agent session",
    });
    await expect(
      c.queryByRole("button", { name: "Open Nyra ended agent session" }),
    ).toBeNull();
    await expect(c.getByLabelText("Execution commands")).toHaveTextContent("0");
  }}
/>
