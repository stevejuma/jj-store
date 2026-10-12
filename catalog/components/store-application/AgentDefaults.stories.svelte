<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Agent defaults",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function defaultsSessionModels({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await userEvent.click(
      c.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Settings" }),
    );
    await c.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Agents" }),
    );
    await userEvent.click(await c.findByRole("combobox", { name: "Model" }));
    await userEvent.click(
      await page.findByRole("option", { name: /Codex Fast/ }),
    );
    await expect(c.getByRole("combobox", { name: "Model" })).toHaveTextContent(
      "Codex Fast",
    );
    await c.findByText(/Models could not be loaded/);
    const retry = c.getByRole("button", { name: "Retry models" });
    const row = retry.closest<HTMLElement>(".store-settings__action-row")!;
    await expect(row.parentElement).toHaveClass("store-settings__rows");
    await expect(getComputedStyle(row).borderTopWidth).toBe("1px");
    await expect(getComputedStyle(row).borderRadius).toBe("0px");
    await expect(
      row.querySelector("p")!.getBoundingClientRect().left,
    ).toBeLessThan(retry.getBoundingClientRect().left);
    const count = Number(
      c.getByLabelText("Model catalog requests").textContent,
    );
    await userEvent.click(c.getByRole("button", { name: "Retry models" }));
    await waitFor(() =>
      expect(
        Number(c.getByLabelText("Model catalog requests").textContent),
      ).toBeGreaterThan(count),
    );
    await expect(c.getByRole("combobox", { name: "Model" })).toHaveTextContent(
      "Codex Fast",
    );
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }
</script>

<Story
  name="Defaults use live session models when discovery fails"
  args={{ directory: true, catalogSession: "live", modelFailure: true }}
  play={defaultsSessionModels}
/>

<Story
  name="Defaults use remembered session models and retain selection on retry"
  args={{ directory: true, catalogSession: "remembered", modelFailure: true }}
  play={defaultsSessionModels}
/>

<Story
  name="Defaults last with shared harness model picker and joined save"
  args={{ directory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await userEvent.click(
      c.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Settings" }),
    );
    await c.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Agents" }),
    );
    await c.findByRole("button", { name: "Save defaults" });
    const toggle = c.getByRole("switch", {
      name: "Automatically mention agents",
    });
    const toggleRow = toggle.closest(".store-settings__option-row")!;
    await expect(
      toggleRow.getBoundingClientRect().right -
        toggle.getBoundingClientRect().right,
    ).toBeLessThanOrEqual(17);
    await expect(
      toggleRow
        .querySelector('[data-slot="field-content"]')!
        .getBoundingClientRect().width,
    ).toBeGreaterThan(toggleRow.getBoundingClientRect().width * 0.75);
    const metadata = within(
      c.getByRole("region", { name: "Preferences" }),
    ).getByRole("button", { name: "Conversation metadata" });
    await expect(metadata).toHaveAttribute("aria-expanded", "false");
    metadata.focus();
    await userEvent.keyboard("{Enter}");
    await expect(metadata).toHaveAttribute("aria-expanded", "true");
    for (const name of ["Metadata model", "Model"]) {
      const picker = c.getByRole("combobox", { name });
      await expect(picker.getBoundingClientRect().width).toBeLessThanOrEqual(
        320,
      );
      await expect(picker.getBoundingClientRect().right).toBe(
        picker.parentElement!.getBoundingClientRect().right,
      );
    }
    const metadataRegion = within(
      c.getByRole("region", { name: "Conversation metadata settings" }),
    );
    await userEvent.click(
      metadataRegion.getByRole("button", {
        name: "Metadata provider environment",
      }),
    );
    const metadataDraft = metadataRegion.getByRole("textbox", {
      name: "Environment variable name",
    });
    await userEvent.type(metadataDraft, "RETAINED_METADATA_DRAFT");
    metadata.focus();
    await userEvent.keyboard(" ");
    await expect(metadataDraft).not.toBeVisible();
    await userEvent.keyboard("{Enter}");
    await expect(metadataDraft).toHaveValue("RETAINED_METADATA_DRAFT");
    await userEvent.click(metadata);
    const headings = c.getAllByRole("heading", { level: 2 });
    await expect(headings.map((h) => h.textContent?.trim())).toEqual([
      "Preferences",
      "Agent runtimes",
      "ChatGPT accounts",
      "Agent defaults",
    ]);
    for (const h of headings) {
      const header = h.closest(".store-settings-card__header")!;
      await expect(header).not.toBeNull();
      await expect(
        parseFloat(getComputedStyle(header).paddingTop),
      ).toBeGreaterThanOrEqual(8);
      await expect(
        parseFloat(getComputedStyle(header).paddingBottom),
      ).toBeGreaterThanOrEqual(8);
      await expect(getComputedStyle(h).fontWeight).toBe("600");
    }
    const defaults = within(c.getByRole("region", { name: "Agent defaults" }));
    const table = c
      .getByRole("region", { name: "Agent defaults" })
      .querySelector<HTMLElement>(".store-settings__rows")!;
    for (const label of ["Harness", "Model", "Execution mode"]) {
      const field = defaults
        .getByLabelText(label, { exact: true })
        .closest<HTMLElement>(".community-settings__option-row")!;
      await expect(field.getBoundingClientRect().width).toBeGreaterThan(
        table.getBoundingClientRect().width - 4,
      );
      const control = field.querySelector<HTMLElement>(".store-work__control")!;
      await expect(control.getBoundingClientRect().left).toBeGreaterThan(
        field.querySelector("label")!.getBoundingClientRect().right,
      );
    }
    const discovery = defaults
      .getByRole("button", { name: "Retry models" })
      .closest<HTMLElement>(".store-settings__action-row")!;
    const lastField = defaults
      .getByLabelText("Execution mode", { exact: true })
      .closest<HTMLElement>(".community-settings__option-row")!;
    await expect(discovery.getBoundingClientRect().top).toBeCloseTo(
      lastField.getBoundingClientRect().bottom,
      0,
    );
    await expect(
      defaults.queryByText("Harness and model", { exact: true }),
    ).toBeNull();
    const model = defaults.getByRole("combobox", { name: "Model" });
    await expect(
      model.closest(".community-settings__option-row"),
    ).not.toBeNull();
    await userEvent.click(model);
    await userEvent.type(
      page.getByRole("combobox", { name: "Search models" }),
      "fixture",
    );
    await userEvent.click(
      await page.findByRole("option", { name: /Fixture model/ }),
    );
    await expect(model).toHaveTextContent("Fixture model");
    await expect(model).toHaveFocus();
    await expect(await defaults.findByLabelText("Thinking")).toBeVisible();
    const save = defaults.getByRole("button", { name: "Save defaults" });
    const action = save.closest<HTMLElement>(".store-settings__action-row")!;
    await expect(action.parentElement).toHaveClass("store-settings__rows");
    const style = getComputedStyle(action);
    await expect(style.borderTopWidth).toBe("1px");
    await expect(style.borderRightWidth).toBe("0px");
    await expect(style.borderBottomWidth).toBe("0px");
    await expect(style.borderLeftWidth).toBe("0px");
    await expect(style.borderRadius).toBe("0px");
    await userEvent.click(save);
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Local service" }),
    );
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Agents" }),
    );
    await waitFor(() =>
      expect(c.getByRole("combobox", { name: "Model" })).toHaveTextContent(
        "Fixture model",
      ),
    );
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>

<Story
  name="Global defaults Advanced retains environment draft without saving"
  args={{ directory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true });
    await userEvent.click(
      c.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await page.findByRole("menuitem", { name: "Settings" }),
    );
    await c.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Agents" }),
    );
    const advanced = await c.findByRole("button", {
      name: "Advanced",
    });
    await expect(
      advanced.querySelector("svg.lucide-chevron-right"),
    ).not.toBeNull();
    await expect(advanced).toHaveAttribute("aria-expanded", "false");
    const group = advanced.closest<HTMLElement>('[data-slot="field-group"]')!;
    const table = advanced.closest<HTMLElement>(".store-settings__rows")!;
    await expect(getComputedStyle(group).borderTopWidth).toBe("1px");
    await expect(getComputedStyle(group).borderRightWidth).toBe("0px");
    await expect(getComputedStyle(group).borderBottomWidth).toBe("0px");
    await expect(getComputedStyle(group).borderLeftWidth).toBe("0px");
    await expect(getComputedStyle(group).borderRadius).toBe("0px");
    await expect(advanced.getBoundingClientRect().width).toBeGreaterThan(
      table.getBoundingClientRect().width - 4,
    );
    const discovery = c
      .getByRole("button", { name: "Retry models" })
      .closest<HTMLElement>(".store-settings__action-row")!;
    await expect(group.getBoundingClientRect().top).toBeCloseTo(
      discovery.getBoundingClientRect().bottom,
      0,
    );
    advanced.focus();
    await userEvent.keyboard("{Enter}");
    const name = c.getByRole("textbox", { name: "Environment variable name" });
    await userEvent.type(name, "GLOBAL_DRAFT");
    advanced.focus();
    await userEvent.keyboard(" ");
    await expect(name).not.toBeVisible();
    await userEvent.keyboard("{Enter}");
    await expect(name).toHaveValue("GLOBAL_DRAFT");
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>
