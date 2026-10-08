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
  name="Joined model commands below description in a fixed width wrapping row"
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
        const title = c.getByRole("heading", { name: "Compute", level: 1 });
        await expect(
          c.getByRole("tab", { name: "Models" }).getBoundingClientRect().left -
            title.getBoundingClientRect().right,
        ).toBeGreaterThanOrEqual(20);
        await expect(
          getComputedStyle(canvasElement.querySelector(".store-settings")!)
            .paddingTop,
        ).toBe("16px");
        const run = c
          .getByRole("button", { name: "Prepare runtime" })
          .closest(".store-settings__action-row")!;
        const description = run.querySelector("p")!;
        const actions = run.querySelector(".store-settings__actions")!;
        await expect(
          actions.getBoundingClientRect().top -
            description.getBoundingClientRect().bottom,
        ).toBeGreaterThanOrEqual(12);
        const actionWidth = actions.getBoundingClientRect().width;
        await expect(actionWidth).toBeGreaterThan(
          run.getBoundingClientRect().width - 40,
        );
        const buttons = within(actions as HTMLElement).getAllByRole("button");
        await expect(getComputedStyle(actions).flexDirection).toBe("row");
        await expect(getComputedStyle(actions).flexWrap).toBe("wrap");
        for (const button of buttons)
          await expect(Math.round(button.getBoundingClientRect().width)).toBe(
            160,
          );
        if (width === 1280)
          await expect(buttons[3].getBoundingClientRect().top).toBe(
            buttons[0].getBoundingClientRect().top,
          );
        else
          await expect(buttons[3].getBoundingClientRect().top).toBeGreaterThan(
            buttons[0].getBoundingClientRect().top,
          );
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
    const memory = c.getByLabelText("Maximum memory / VRAM");
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
  name="Narrow settings retain bounded identities and stacked actions"
  args={{ paired: true }}
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    try {
      const c = within(canvasElement);
      await userEvent.click(await c.findByRole("tab", { name: "Sharing" }));
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
        await expect(getComputedStyle(row).flexDirection).toBe("column");
        const buttons = within(row as HTMLElement).getAllByRole("button");
        for (const button of buttons)
          await expect(Math.round(button.getBoundingClientRect().width)).toBe(
            Math.round(buttons[0].getBoundingClientRect().width),
          );
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
      "1px",
    );
    for (const row of canvasElement.querySelectorAll(
      ".store-settings__action-row",
    )) {
      if (row.closest("[hidden]")) continue;
      if (row.classList.contains("store-settings__action-row--joined")) {
        await expect(getComputedStyle(row).borderLeftWidth).toBe("0px");
        await expect(getComputedStyle(row).borderRightWidth).toBe("0px");
        await expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
      } else {
        await expect(
          row.parentElement!.closest(".store-settings__rows"),
        ).toBeNull();
      }
      const actions = row.querySelector(".store-settings__actions")!;
      await expect(actions).not.toBeNull();
      await expect(getComputedStyle(actions).flexDirection).toBe(
        actions.classList.contains("store-settings__actions--wrapping")
          ? "row"
          : "column",
      );
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
    await expect(c.getByText("Historical", { exact: true }).tagName).toBe(
      "STRONG",
    );
    await userEvent.click(c.getByText("Raw context payload", { exact: true }));
    await expect(
      c.getByText(/9007199254740993/, { selector: "pre" }),
    ).toBeVisible();
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
