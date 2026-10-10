<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects agents",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Landing agents and native session navigation"
  args={{ agents: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: /Open Codex ended/ });
    await expect(c.getByRole("tab", { name: "Agents" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      c.getByRole("region", { name: "Agents in UI repository" }),
    ).toBeVisible();
    const row = canvasElement.querySelector<HTMLElement>(
      '.store-collaboration__row[data-kind="agent"]',
    )!;
    const heading = row
      .closest("section")!
      .querySelector<HTMLElement>(".community-projects__list-group-heading")!;
    await expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
    await expect(
      Math.abs(
        row.getBoundingClientRect().left - heading.getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    await expect(
      Math.abs(
        row.getBoundingClientRect().right -
          heading.getBoundingClientRect().right,
      ),
    ).toBeLessThan(1);
    await userEvent.click(c.getByRole("button", { name: /Open Codex ended/ }));
    await expect(
      c.getByRole("region", { name: "Projects and tasks" }),
    ).toBeVisible();
    await expect(row.isConnected).toBe(true);
    await expect(
      c.getByRole("complementary", { name: "Agent response" }),
    ).toBeVisible();
    await c.findByRole("log", { name: "Agent conversation activity" });
    const response = c.getByRole("complementary", { name: "Agent response" });
    await expect(response).toHaveAttribute("data-ui-part", "sidebar");
    await expect(
      response.querySelector(".store-collaboration__detail-header"),
    ).toBeNull();
    const avatar = response.querySelector(
      '[data-ui-part="sidebar-header"] [data-ui-component="community-member-avatar"]',
    );
    await expect(avatar).toBeVisible();
    await expect(avatar?.querySelector("img")).toHaveAttribute(
      "src",
      "/agent-icons/codex.svg",
    );
    const transcriptRow =
      response.querySelector<HTMLElement>("[data-chat-item]")!;
    await expect(transcriptRow).not.toBeNull();
    const viewport = response.querySelector(
      '[data-ui-part="scroll-area-viewport"]',
    )!;
    await waitFor(() =>
      expect(
        transcriptRow.getBoundingClientRect().top -
          viewport.getBoundingClientRect().top,
      ).toBeLessThan(48),
    );
    const body = canvasElement.querySelector('[data-ui-part="main"]')!;
    await waitFor(() => {
      expect(
        Math.abs(
          response.getBoundingClientRect().top -
            body.getBoundingClientRect().top,
        ),
      ).toBeLessThan(1);
      expect(
        Math.abs(
          response.getBoundingClientRect().bottom -
            body.getBoundingClientRect().bottom,
        ),
      ).toBeLessThan(1);
    });
    await userEvent.click(
      c.getByRole("button", { name: "Close agent response" }),
    );
    await expect(
      c.queryByRole("complementary", { name: "Agent response" }),
    ).toBeNull();
    await expect(row.isConnected).toBe(true);
    await expect(
      within(row).getByRole("button", { name: /Open Codex ended/ }),
    ).toHaveFocus();
  }}
/>
<Story
  name="Selected project agents"
  args={{ projectDetail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Edit project" });
    await userEvent.click(c.getByRole("tab", { name: "Agents" }));
    await c.findByRole("button", { name: /Open Codex ended/ });
    await expect(
      c.getByRole("button", { name: /Open OpenCode active/ }),
    ).toBeVisible();
  }}
/>
<Story
  name="Agent loading uses a skeleton"
  args={{ agents: true, loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const status = await c.findByLabelText("Loading agents");
    await expect(status).toBeVisible();
    const row = status.querySelector<HTMLElement>(
      ".store-collaboration-skeleton__row",
    )!;
    await expect(row).not.toBeNull();
    await expect(
      status.getBoundingClientRect().left -
        status.parentElement!.getBoundingClientRect().left,
    ).toBeCloseTo(16, 0);
    await expect(
      row
        .querySelector("[data-ui-component='skeleton']")!
        .getBoundingClientRect().left - status.getBoundingClientRect().left,
    ).toBeCloseTo(10.4, 0);
    await expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
    await expect(c.queryByText("No agent sessions")).toBeNull();
  }}
/>

<Story
  name="Mobile agent response uses the standard right lane"
  args={{ agents: true, mobile: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const row = await c.findByRole("button", { name: /Open Codex ended/ });
    await userEvent.click(row);
    const response = await c.findByRole("complementary", {
      name: "Agent response",
    });
    await expect(response).toHaveAttribute("data-ui-part", "sidebar");
    await expect(response).toBeVisible();
    await userEvent.click(
      await within(response).findByRole("button", {
        name: "Close agent response",
      }),
    );
    await waitFor(() => expect(row).toBeVisible());
    await expect(row.isConnected).toBe(true);
    await expect(row).toHaveFocus();
  }}
/>
