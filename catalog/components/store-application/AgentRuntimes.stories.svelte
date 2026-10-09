<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import AgentRuntimesFixture from "../../fixtures/AgentRuntimesFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Agent runtimes",
    component: AgentRuntimesFixture,
    tags: ["visual-pending"],
  });
  async function inlineIcon(control: HTMLElement) {
    const style = getComputedStyle(control);
    // Flex items are blockified, so inline-flex can compute to flex.
    await expect(["flex", "inline-flex"]).toContain(style.display);
    await expect(style.flexDirection).toBe("row");
    await expect(style.alignItems).toBe("center");
    const icon = control.querySelector("svg")!.getBoundingClientRect();
    await expect(icon.width).toBeGreaterThanOrEqual(12);
    await expect(icon.width).toBeLessThanOrEqual(20);
    const text = [...control.childNodes].find(
      (node) => node.nodeType === Node.TEXT_NODE && node.textContent?.trim(),
    )!;
    const range = document.createRange();
    range.selectNode(text);
    const label = range.getBoundingClientRect();
    await expect(label.left).toBeGreaterThanOrEqual(icon.right);
    await expect(
      Math.abs(label.top + label.height / 2 - icon.top - icon.height / 2),
    ).toBeLessThanOrEqual(2);
  }
  async function open(canvasElement: HTMLElement, loading = false) {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(c.getByRole("tab", { name: "Runtimes" }));
    await expect(
      await c.findByRole("heading", { name: "Agent runtimes" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("region", { name: "Agent capture settings" }),
    ).toBeNull();
    if (!loading) {
      const guide = await c.findByRole("link", { name: "Update guide" });
      await inlineIcon(guide);
    }
    return c;
  }
</script>

<Story
  name="Runtime menus align at the far right after optional actions"
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    const controls = c.getAllByRole("button", { name: /runtime options$/ });
    expect(controls).toHaveLength(3);
    const edges = controls.map(
      (control) => control.getBoundingClientRect().right,
    );
    expect(Math.max(...edges) - Math.min(...edges)).toBeLessThanOrEqual(1);
    for (const control of controls) {
      const actions = control.closest<HTMLElement>(".store-runtime__actions")!;
      expect(
        Math.abs(
          actions.getBoundingClientRect().right -
            control.getBoundingClientRect().right,
        ),
      ).toBeLessThanOrEqual(1);
    }
    controls[1].focus();
    await userEvent.keyboard("{Enter}");
    const page = within(document.body);
    expect(
      await page.findByRole("menuitem", { name: "Setup guide" }),
    ).toHaveAttribute("href", "https://cursor.com/docs/cli/installation");
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(controls[1]).toHaveFocus());
    expect(c.getByLabelText("Installation command count")).toHaveTextContent(
      "0",
    );
  }}
/>

<Story
  name="Bundled JJ Agent uses app artwork without an installer"
  args={{ scenario: "ready" }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    const label = c.getByText("JJ Agent", { exact: true });
    const row = label.closest<HTMLElement>('[data-slot="field"]')!;
    await expect(
      row.querySelector('img[src="/agent-icons/jj-store.svg"]'),
    ).not.toBeNull();
    await expect(within(row).getByText("Ready", { exact: true })).toBeVisible();
    await expect(
      within(row).queryByRole("button", { name: /Install|Update/ }),
    ).toBeNull();
  }}
/>

<Story
  name="Install is explicit and retains settings navigation"
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    await expect(await c.findByText("Adapter needed")).toBeVisible();
    await expect(
      c.getByLabelText("Installation command count"),
    ).toHaveTextContent("0");
    c.getByRole("button", { name: "Install Codex runtime" }).focus();
    await userEvent.keyboard("{Enter}");
    const page = within(document.body);
    await expect(
      await page.findByRole("dialog", { name: "Install Codex runtime?" }),
    ).toBeVisible();
    await expect(page.getByText(/Download Codex ACP 1.13.1/)).toBeVisible();
    const install = page.getByRole("button", {
      name: "Install runtime",
    });
    await inlineIcon(install);
    await userEvent.click(page.getByRole("button", { name: "Cancel" }));
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await expect(
      c.getByLabelText("Installation command count"),
    ).toHaveTextContent("0");
    await userEvent.click(
      c.getByRole("button", { name: "Install Codex runtime" }),
    );
    await userEvent.click(
      await page.findByRole("button", { name: "Install runtime" }),
    );
    await expect(await c.findByText(/Managed by jj-store/)).toBeVisible();
    await expect(
      c.getByLabelText("Installation command actions"),
    ).toHaveTextContent("codex:install");
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Runtimes" }));
    await expect(
      c.getByLabelText("Installation command count"),
    ).toHaveTextContent("1");
  }}
/>
<Story
  name="Update uses the fixed runtime and readable versions"
  args={{ scenario: "update" }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    await expect(await c.findByText("Update available")).toBeVisible();
    await expect(c.getByText(/ACP 1.4.0/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Update Codex runtime" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("button", {
        name: "Update runtime",
      }),
    );
    await expect(await c.findByText(/Managed by jj-store/)).toBeVisible();
    await expect(
      c.getByLabelText("Installation command actions"),
    ).toHaveTextContent("codex:update");
  }}
/>
<Story
  name="Ready tooling and retained rows after failed refresh"
  args={{ scenario: "ready" }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    await expect(await c.findByText(/Managed by jj-store/)).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Install Codex runtime" }),
    ).toBeNull();
    await expect(c.getAllByText("Ready")[0]).toHaveAttribute(
      "data-chip-tone",
      "success",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Fail runtime checks" }),
    );
    await waitFor(() => expect(c.getAllByText("Unknown")).toHaveLength(3));
    await expect(c.getByText("Codex")).toBeVisible();
    await expect(
      c.getByLabelText("Installation command count"),
    ).toHaveTextContent("0");
    await userEvent.click(
      c.getByRole("button", { name: "Recover runtime checks" }),
    );
    await waitFor(() => expect(c.getAllByText("Ready")).toHaveLength(3));
  }}
/>
<Story
  name="Unknown installation queries its original receipt across tab changes"
  args={{ lost: true }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Install Codex runtime" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("button", {
        name: "Install runtime",
      }),
    );
    await expect(
      await c.findByRole("button", { name: "Install Codex runtime" }),
    ).toBeDisabled();
    await waitFor(() =>
      expect(document.body).not.toHaveStyle({ pointerEvents: "none" }),
    );
    await userEvent.click(c.getByRole("tab", { name: "Capture" }));
    await userEvent.click(c.getByRole("tab", { name: "Runtimes" }));
    await userEvent.click(
      c.getByRole("button", { name: "Complete original installation" }),
    );
    await expect(await c.findByText(/Managed by jj-store/)).toBeVisible();
    await expect(
      c.getByLabelText("Installation command count"),
    ).toHaveTextContent("1");
  }}
/>
<Story
  name="Missing prerequisites links to device setup"
  args={{ prerequisites: true }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement);
    await expect(
      await c.findByRole("button", { name: "Install Codex runtime" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("link", { name: "Install Node.js" }),
    ).toHaveAttribute("href", "https://nodejs.org/en/download");
    await expect(c.getByRole("link", { name: "Update guide" })).toHaveAttribute(
      "href",
      "https://cursor.com/docs/cli/installation",
    );
  }}
/>
<Story
  name="Loading uses settings rows"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const c = await open(canvasElement, true);
    await expect(c.getByLabelText("Loading agent runtimes")).toHaveAttribute(
      "aria-busy",
      "true",
    );
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    await expect(await c.findByText("Adapter needed")).toBeVisible();
  }}
/>
