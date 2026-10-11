<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/LocalComputeFixture.svelte";
  import { page } from "vitest/browser";
  const { Story } = defineMeta({
    title: "JJ Store/Local compute",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Browse GGUF selects a file before explicit import"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: "Browse…" }));
    await expect(c.getByLabelText("Import GGUF")).toHaveValue(
      "/models/My model.gguf",
    );
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
    await userEvent.click(c.getByRole("button", { name: "Import model" }));
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]")[0]
          .action,
      ).toEqual({ action: "import", path: "/models/My model.gguf" }),
    );
  }}
/>
<Story
  name="Cancelled GGUF browse preserves manual path"
  args={{ picker: "cancelled" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.type(
      await c.findByLabelText("Import GGUF"),
      "/models/keep.gguf",
    );
    await userEvent.click(c.getByRole("button", { name: "Browse…" }));
    await expect(c.getByLabelText("Import GGUF")).toHaveValue(
      "/models/keep.gguf",
    );
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
  }}
/>

<Story
  name="Model commands use right-aligned wrapping section footers"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByLabelText("Model");
    for (const [field, label] of [
      ["Model", "Run a local model"],
      ["Import GGUF", "Import model weights"],
    ]) {
      const rows = c
        .getByLabelText(field)
        .closest<HTMLElement>(".store-settings__rows")!;
      const action = c
        .getByText(label, { selector: "strong" })
        .closest<HTMLElement>(".store-settings__action-row")!;
      await expect(rows).toContainElement(action);
      await expect(getComputedStyle(action).borderLeftWidth).toBe("0px");
      await expect(getComputedStyle(action).borderRightWidth).toBe("0px");
      await expect(getComputedStyle(action).borderRadius).toBe("0px");
      await expect(getComputedStyle(action).borderTopWidth).toBe("1px");
    }
    for (const width of [1280, 390]) {
      await page.viewport(width, 844);
      try {
        const run = c
          .getByRole("button", { name: "Prepare runtime" })
          .closest(".store-settings-footer")!;
        const actions = run.querySelector(".store-settings__actions")!;
        await expect(getComputedStyle(actions).flexDirection).toBe("row");
        await expect(getComputedStyle(actions).flexWrap).toBe("wrap");
        await expect(actions.lastElementChild).toBe(
          c.getByRole("button", { name: "Start model" }),
        );
        for (const button of within(actions as HTMLElement).getAllByRole(
          "button",
        )) {
          await expect(
            button.getBoundingClientRect().right,
          ).toBeLessThanOrEqual(run.getBoundingClientRect().right);
          await expect(
            button.getBoundingClientRect().left,
          ).toBeGreaterThanOrEqual(run.getBoundingClientRect().left);
        }
        await expect(
          canvasElement.ownerDocument.documentElement.scrollWidth,
        ).toBeLessThanOrEqual(width);
      } finally {
        await page.viewport(1280, 800);
      }
    }
  }}
/>

<Story
  name="Logical tabs retain all drafts and save resource settings"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const models = c.getByRole("tab", { name: "Models" });
    await expect(models).toHaveAttribute("aria-selected", "true");
    await userEvent.type(
      await c.findByLabelText("Import GGUF"),
      "/models/draft.gguf",
    );
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    await expect(c.getByRole("tabpanel", { name: "Resources" })).toBeVisible();
    await expect(c.queryByRole("heading", { name: "Local models" })).toBeNull();
    const memory = c.getByRole("spinbutton", { name: "Maximum memory / VRAM" });
    await userEvent.clear(memory);
    await userEvent.type(memory, "6");
    await userEvent.click(c.getByRole("tab", { name: "Sharing" }));
    await userEvent.type(c.getByLabelText("Machine name"), "Pairing draft");
    await userEvent.click(models);
    await expect(c.getByLabelText("Import GGUF")).toHaveValue(
      "/models/draft.gguf",
    );
    await userEvent.click(c.getByRole("tab", { name: "Sharing" }));
    await expect(c.getByLabelText("Machine name")).toHaveValue("Pairing draft");
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    await expect(memory).toHaveValue(6);
    await expect(
      JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
    ).toEqual([]);
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
      ).toMatchObject([
        { action: { action: "configure", settings: { memory_gb: 6 } } },
      ]),
    );
    models.focus();
    await userEvent.keyboard("{ArrowRight}{Enter}");
    await expect(c.getByRole("tab", { name: "Resources" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
  }}
/>

<Story
  name="Narrow settings retain bounded identities and wrapping footers"
  args={{ paired: true }}
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    try {
      const c = within(canvasElement);
      const sharing = c.queryByRole("tab", { name: "Sharing" });
      if (sharing) await userEvent.click(sharing);
      else {
        await userEvent.click(
          await c.findByRole("button", {
            name: "More compute settings sections",
          }),
        );
        await userEvent.click(
          await within(document.body).findByRole("menuitem", {
            name: "Sharing",
          }),
        );
      }
      await c.findByRole("button", { name: "Unpair" });
      await expect(
        c.getByLabelText("Machine invitation", {
          selector: "pre",
        }),
      ).toBeVisible();
      await expect(
        c.getByRole("button", { name: /^Copy full Machine identity/ }),
      ).toBeVisible();
      await expect(
        canvasElement.ownerDocument.documentElement.scrollWidth,
      ).toBeLessThanOrEqual(390);
      for (const row of canvasElement.querySelectorAll(
        ".store-settings__action-row",
      )) {
        if (row.closest("[hidden]")) continue;
        const actions = row.querySelector(".store-settings__actions")!;
        await expect(getComputedStyle(actions).flexWrap).toBe("wrap");
        for (const button of within(row as HTMLElement).getAllByRole("button"))
          await expect(
            button.getBoundingClientRect().right,
          ).toBeLessThanOrEqual(390);
      }
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>

<Story
  name="Compact settings sections and one border per group"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const title = c.getByRole("heading", { name: "Compute", level: 1 });
    for (const [tab, name] of [
      ["Resources", "This machine"],
      ["Models", "Local models"],
      ["Resources", "Resource limits"],
      ["Sharing", "Shared compute"],
    ]) {
      await userEvent.click(c.getByRole("tab", { name: tab }));
      const heading = await c.findByRole("heading", { name, level: 2 });
      await expect(
        parseFloat(getComputedStyle(heading).fontSize),
      ).toBeLessThanOrEqual(parseFloat(getComputedStyle(title).fontSize));
      await expect(c.getByRole("region", { name })).toBeVisible();
    }
    await userEvent.click(c.getByRole("tab", { name: "Models" }));
    const group = c
      .getByLabelText("Model")
      .closest('[data-slot="field-group"]')!;
    await expect(getComputedStyle(group).borderLeftWidth).toBe("0px");
    await expect(getComputedStyle(group).borderRightWidth).toBe("0px");
    await expect(getComputedStyle(group).borderBottomWidth).toBe("0px");
    await expect(getComputedStyle(group.parentElement!).borderLeftWidth).toBe(
      "0px",
    );
    for (const row of canvasElement.querySelectorAll(
      ".store-settings__action-row",
    )) {
      if (row.closest("[hidden]")) continue;
      await expect(row).toHaveClass("store-settings-footer");
      await expect(getComputedStyle(row).borderLeftWidth).toBe("0px");
      await expect(getComputedStyle(row).borderRightWidth).toBe("0px");
      const actions = row.querySelector(".store-settings__actions")!;
      await expect(getComputedStyle(actions).flexDirection).toBe("row");
    }
  }}
/>

<Story
  name="Lucide Advanced disclosure preserves relay draft with keyboard"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("tab", { name: "Sharing" }));
    const label = c
      .getByRole("button", { name: "Advanced" })
      .querySelector("span")!;
    await expect(
      label.parentElement!.querySelector("svg.lucide-chevron-right"),
    ).not.toBeNull();
    const trigger = c.getByRole("button", { name: "Advanced" });
    await expect(trigger).toHaveAttribute("aria-expanded", "false");
    trigger.focus();
    await userEvent.keyboard("{Enter}");
    await expect(trigger).toHaveAttribute("aria-expanded", "true");
    const relay = c.getByLabelText("Encrypted relay URLs");
    await userEvent.type(relay, "https://relay.example.test/");
    trigger.focus();
    await userEvent.keyboard(" ");
    await expect(relay).not.toBeVisible();
    await userEvent.keyboard("{Enter}");
    await expect(relay).toHaveValue("https://relay.example.test/");
    await expect(trigger).toHaveFocus();
    await expect(
      JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
    ).toEqual([]);
  }}
/>

<Story
  name="Explicit preparation download start and stop"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("heading", { name: "Compute", level: 1 }),
    ).toBeVisible();
    const start = await c.findByRole("button", { name: "Start model" });
    await expect(start).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Prepare runtime" }));
    await expect(start).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Download model" }));
    await waitFor(() => expect(start).toBeEnabled());
    await userEvent.click(start);
    await waitFor(() => expect(c.getByLabelText("Model")).toBeDisabled());
    await userEvent.click(c.getByRole("button", { name: "Stop compute" }));
    await waitFor(() => expect(c.getByLabelText("Model")).toBeEnabled());
    await expect(
      JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]").map(
        (row: { action: { action: string } }) => row.action.action,
      ),
    ).toEqual([
      "prepare",
      "configure",
      "download",
      "configure",
      "start",
      "stop",
    ]);
  }}
/>

<Story
  name="Rich historical context and exact raw version"
  args={{ context: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const summary = c.getByText("Sent context", { exact: true });
    await expect(
      c.getByText("fn captured_file() {}", { exact: true }),
    ).not.toBeVisible();
    await userEvent.click(summary);
    await expect(
      c.getByText("fn captured_file() {}", { exact: true }),
    ).toBeVisible();
    await expect(
      c.getByText("captured-revision", { exact: true }),
    ).toBeVisible();
    await expect(
      c.getByText("Captured task · version 9007199254740993"),
    ).toBeVisible();
    const panel = c.getByRole("region", {
      name: "Application context for this message",
    });
    const paragraph = c.getByText("Historical", { exact: true }).closest("p")!;
    expect(
      paragraph.getBoundingClientRect().left -
        panel.getBoundingClientRect().left,
    ).toBeLessThanOrEqual(16);
    await expect(c.getByText("Historical", { exact: true }).tagName).toBe(
      "STRONG",
    );
    await userEvent.click(c.getByRole("button", { name: "Show raw context" }));
    await expect(
      c.getByRole("region", { name: "Raw context" }),
    ).toHaveTextContent("9007199254740993");
  }}
/>

<Story
  name="Inference host dropdown retains peer and local default selections"
  args={{ paired: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      menu = within(document.body);
    await userEvent.click(await c.findByRole("tab", { name: "Sharing" }));
    const host = c.getByLabelText("Inference host");
    await expect(host).toHaveAttribute("data-ui-part", "select-trigger");
    host.focus();
    await userEvent.keyboard("{Enter}");
    await userEvent.click(
      await menu.findByRole("option", { name: "Trusted inference machine" }),
    );
    await waitFor(() =>
      expect(getComputedStyle(host).pointerEvents).not.toBe("none"),
    );
    await expect(host).toHaveTextContent("Trusted inference machine");
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]")[0]
          .action.settings.target_peer,
      ).toBe("ab".repeat(32)),
    );
    await userEvent.click(host);
    await userEvent.click(
      await menu.findByRole("option", { name: "This machine" }),
    );
    await waitFor(() =>
      expect(getComputedStyle(host).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]")[1]
          .action.settings.target_peer,
      ).toBeNull(),
    );
  }}
/>

<Story
  name="Automatic memory remains optional with a joined fixed unit"
  args={{ memoryLimit: null }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    const input = await c.findByRole("spinbutton", {
      name: "Maximum memory / VRAM",
    });
    await expect(input).toHaveValue(null);
    await expect(input).not.toBeRequired();
    await expect(input).not.toHaveAttribute("aria-invalid", "true");
    await userEvent.type(input, "6");
    await userEvent.clear(input);
    await userEvent.click(c.getByRole("tab", { name: "Models" }));
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
      ).toMatchObject([
        { action: { action: "configure", settings: { memory_gb: null } } },
      ]),
    );
  }}
/>
<Story name="Dark compute settings" globals={{ colorMode: "dark" }} />

<Story
  name="Shared save footer stays separated on every Compute tab"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Save compute settings" });
    for (const name of ["Models", "Resources", "Sharing"]) {
      await userEvent.click(c.getByRole("tab", { name }));
      const panel = c.getByRole("tabpanel", { name });
      const cards = panel.querySelectorAll(".store-settings__section");
      const previous = cards[cards.length - 1];
      const footer = c
        .getByRole("button", { name: "Save compute settings" })
        .closest(".store-settings-save-card")!;
      await expect(
        footer.getBoundingClientRect().top -
          previous.getBoundingClientRect().bottom,
      ).toBe(16);
    }
  }}
/>
