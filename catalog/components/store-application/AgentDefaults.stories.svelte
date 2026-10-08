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
    await userEvent.click(c.getByRole("combobox", { name: "Model" }));
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
    const headings = c.getAllByRole("heading", { level: 2 });
    await expect(headings.map((h) => h.textContent?.trim())).toEqual([
      "Conversations",
      "Preferences",
      "Agent runtimes",
      "Agent defaults",
    ]);
    for (const h of headings) {
      const header = h.closest("header")!;
      await expect(header).not.toBeNull();
      await expect(
        parseFloat(getComputedStyle(header).paddingTop),
      ).toBeGreaterThanOrEqual(8);
      await expect(
        parseFloat(getComputedStyle(header).paddingBottom),
      ).toBeGreaterThanOrEqual(8);
      await expect(getComputedStyle(h).fontWeight).toBe("650");
    }
    const defaults = within(c.getByRole("region", { name: "Agent defaults" }));
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
