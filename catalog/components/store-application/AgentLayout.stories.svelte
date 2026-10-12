<script module lang="ts">
  import { selectSettingsSubtab } from "./settings-shell-play.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentLayoutFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agent layout",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function agentRows(canvasElement: HTMLElement, project = false) {
    const c = within(canvasElement);
    if (project) await c.findByRole("button", { name: "Edit project" });
    const agents = await c.findByRole("tab", { name: "Agents" });
    if (agents.getAttribute("aria-selected") !== "true")
      await userEvent.click(agents);
    await c.findByRole("button", { name: /Open Codex ended/ });
    return c;
  }
  async function spacing({
    canvasElement,
    args,
  }: {
    canvasElement: HTMLElement;
    args: { project?: boolean; mobile?: boolean };
  }) {
    const c = await agentRows(canvasElement, args.project);
    const row = c
      .getByRole("button", { name: /Open Codex ended/ })
      .closest<HTMLElement>(".store-collaboration__row")!;
    const avatar = row.querySelector<HTMLElement>(
      '[data-ui-component="community-member-avatar"]',
    )!;
    const menu = within(row).getByRole("button", {
      name: "Agent session actions",
    });
    const duration = row.querySelector<HTMLElement>(
      ".store-collaboration__agent-duration",
    )!;
    const heading = row
      .closest("section")!
      .querySelector<HTMLElement>(".community-projects__list-group-heading")!;
    await waitFor(() => {
      const r = row.getBoundingClientRect(),
        a = avatar.getBoundingClientRect();
      expect(
        Math.abs(a.top + a.height / 2 - r.top - r.height / 2),
      ).toBeLessThan(1);
      expect(
        duration.getBoundingClientRect().top -
          menu.getBoundingClientRect().bottom,
      ).toBeGreaterThan(4);
      expect(
        Math.abs(r.left - heading.getBoundingClientRect().left),
      ).toBeLessThan(1);
      expect(
        Math.abs(r.right - heading.getBoundingClientRect().right),
      ).toBeLessThan(1);
      expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
    });
    await userEvent.click(menu);
    await within(document.body).findByRole("menuitem", {
      name: /^Copy$/,
    });
    await userEvent.keyboard("{Escape}");
    await userEvent.click(c.getByRole("button", { name: /Open Codex ended/ }));
    await c.findByRole("complementary", { name: "Agent response" });
    await userEvent.hover(
      c.getByRole("button", { name: /Open OpenCode active/ }),
    );
    await new Promise<void>((resolve) =>
      requestAnimationFrame(() =>
        requestAnimationFrame(() => requestAnimationFrame(() => resolve())),
      ),
    );
    const rows = canvasElement.querySelectorAll<HTMLElement>(
      '.store-collaboration__row[data-kind="agent"]',
    );
    await waitFor(() =>
      expect(
        rows[1]!.getBoundingClientRect().top -
          rows[0]!.getBoundingClientRect().bottom,
      ).toBeGreaterThanOrEqual(4),
    );
  }
  async function focused({
    canvasElement,
    args,
  }: {
    canvasElement: HTMLElement;
    args: { project?: boolean; mobile?: boolean };
  }) {
    const c = await agentRows(canvasElement, args.project);
    const opener = c.getByRole("button", { name: /Open Codex ended/ });
    const row = opener.closest(".store-collaboration__row")!;
    await userEvent.click(opener);
    const response = await c.findByRole("region", { name: "Agent response" });
    await expect(response).toHaveAttribute(
      "data-ui-part",
      "surface-layer-surface",
    );
    await expect(
      c.queryByRole("complementary", { name: "Agent response" }),
    ).toBeNull();
    await within(response).findByRole("log", {
      name: "Agent conversation activity",
    });
    await expect(
      response.querySelector('[data-ui-component="community-member-avatar"]'),
    ).toBeVisible();
    const main = canvasElement.querySelector('[data-ui-part="main"]')!;
    const layer = response.closest('[data-ui-part="surface-layer"]')!;
    await waitFor(() => {
      if (args.mobile) {
        expect(layer).toHaveAttribute("data-mobile-placement", "root");
        const root = layer.parentElement!.getBoundingClientRect();
        expect(response.getBoundingClientRect().left - root.left).toBeCloseTo(
          8,
          0,
        );
        expect(root.right - response.getBoundingClientRect().right).toBeCloseTo(
          8,
          0,
        );
      } else {
        expect(
          response.getBoundingClientRect().left -
            main.getBoundingClientRect().left,
        ).toBeCloseTo(112, 0);
        expect(response.getBoundingClientRect().right).toBeGreaterThanOrEqual(
          main.getBoundingClientRect().right,
        );
      }
    });
    await expect(row.isConnected).toBe(true);
    await expect(row).toHaveAttribute("data-selected", "true");
    await userEvent.click(
      within(response).getByRole("button", { name: /^Close agent response$/i }),
    );
    await waitFor(() =>
      expect(c.queryByRole("region", { name: "Agent response" })).toBeNull(),
    );
    await expect(opener.isConnected).toBe(true);
    await expect(opener).toHaveFocus();
    await expect(opener).toHaveAttribute("aria-pressed", "false");
    await userEvent.click(opener);
    await c.findByRole("region", { name: "Agent response" });
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(opener).toHaveFocus());
    await expect(
      c.queryByRole("region", { name: "Agent response" }),
    ).toBeNull();
    await userEvent.click(opener);
    await c.findByRole("region", { name: "Agent response" });
    await userEvent.click(
      canvasElement.querySelector<HTMLElement>(
        '[data-ui-part="surface-layer-scrim"]',
      )!,
    );
    await waitFor(() => expect(opener).toHaveFocus());
    await expect(
      c.queryByRole("region", { name: "Agent response" }),
    ).toBeNull();
  }
  async function layouts({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = await agentRows(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("button", { name: /Open Codex ended/ }));
    await c.findByRole("complementary", { name: "Agent response" });
    for (const layout of ["Focus", "Split"] as const) {
      await userEvent.click(
        c.getByRole("button", { name: /Open profile menu for/ }),
      );
      await userEvent.click(body.getByRole("menuitem", { name: "Settings" }));
      await userEvent.click(
        await c.findByRole("button", { name: "Appearance" }),
      );
      await selectSettingsSubtab(canvasElement, "Preferences");
      await userEvent.click(await c.findByRole("radio", { name: layout }));
      await userEvent.click(
        c.getByRole("button", { name: "Back to jj-store" }),
      );
      const response = await c.findByRole(
        layout === "Focus" ? "region" : "complementary",
        { name: "Agent response" },
      );
      const selected = canvasElement.querySelector(
        '.store-collaboration__row-select[aria-pressed="true"]',
      );
      await expect(selected).toHaveAttribute(
        "aria-label",
        "Open Codex ended agent session",
      );
      await within(response).findByRole("log", {
        name: "Agent conversation activity",
      });
    }
    const opener = c.getByRole("button", { name: /Open Codex ended/ });
    await userEvent.click(
      c.getByRole("button", { name: "Close agent response" }),
    );
    await waitFor(() => expect(opener).toHaveFocus());
  }
</script>

<Story
  name="Projects agent row spacing"
  args={{ focus: false }}
  play={spacing}
/>
<Story
  name="Project agent row spacing"
  args={{ focus: false, project: true }}
  play={spacing}
/>
<Story
  name="Repository agent row spacing"
  args={{ focus: false, repository: true }}
  play={spacing}
/>
<Story name="Projects agents follow Focus layout" play={focused} />
<Story
  name="Project agents follow Focus layout"
  args={{ project: true }}
  play={focused}
/>
<Story
  name="Repository agents follow Focus layout"
  args={{ repository: true }}
  play={focused}
/>
<Story
  name="Mobile Projects agents follow Focus layout"
  args={{ mobile: true }}
  play={focused}
/>
<Story
  name="Mobile repository agents follow Focus layout"
  args={{ repository: true, mobile: true }}
  play={focused}
/>

<Story
  name="Projects retain agent selection across layouts"
  args={{ focus: false }}
  play={layouts}
/>
<Story
  name="Repository retains agent selection across layouts"
  args={{ focus: false, repository: true }}
  play={layouts}
/>
