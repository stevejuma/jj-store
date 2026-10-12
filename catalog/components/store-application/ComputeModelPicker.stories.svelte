<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page } from "vitest/browser";
  import Fixture from "../../fixtures/LocalComputeFixture.svelte";
  const gemma = "unsloth/gemma-4-E4B-it-GGUF:Q4_K_M";
  const { Story } = defineMeta({
    title: "JJ Store/Compute model picker",
    component: Fixture,
    tags: ["visual-pending"],
    args: { memoryLimit: null, selectedModel: gemma, installedModel: true },
  });
</script>

<Story
  name="Searchable choices show fit, recommendation and installed metadata without dispatch"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    const trigger = await c.findByRole("combobox", {
      name: "Model",
    });
    await expect(trigger).toHaveTextContent(
      "4.6 GB Fits well Recommended Installed",
    );
    await userEvent.click(trigger);
    const search = await portal.findByRole("combobox", {
      name: "Search local models",
    });
    await expect(search).toHaveFocus();
    const options = portal.getAllByRole("option");
    await expect(options.length).toBeGreaterThan(9);
    const large = portal.getByRole("option", { name: /Qwen3.8-27B/ });
    await expect(large).toHaveTextContent("17.1 GB Too large");
    await expect(portal.getByText(/70% of host RAM/)).toHaveTextContent(
      "GPU capacity is not measured",
    );
    await userEvent.type(search, "Qwen2.5-Coder");
    await waitFor(() => expect(portal.getAllByRole("option")).toHaveLength(1));
    await expect(portal.getByRole("option")).toHaveTextContent(
      "4.4 GB Fits well Advanced",
    );
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(trigger).toHaveFocus();
    await expect(trigger).toHaveTextContent(
      "Qwen2.5-Coder-7B-Instruct-GGUF:Q4_K_M",
    );
    await expect(c.getByLabelText("Custom model")).toHaveValue(
      "Qwen/Qwen2.5-Coder-7B-Instruct-GGUF:Q4_K_M",
    );
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
    await userEvent.click(trigger);
    await userEvent.keyboard("{Escape}");
    await expect(trigger).toHaveFocus();
  }}
/>

<Story
  name="Draft memory limits recalculate fit and unknown imported sizes remain honest"
  args={{ memoryLimit: 4, selectedModel: "fixture-gguf" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    const trigger = await c.findByRole("combobox", {
      name: "Model",
    });
    await expect(trigger).toHaveTextContent(
      "Fixture GGUF Size unknown Fit unknown",
    );
    await userEvent.click(trigger);
    await expect(
      portal.getByRole("option", { name: /gemma-4-E4B/ }),
    ).toHaveTextContent("Too large");
    await userEvent.keyboard("{Escape}");
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    await userEvent.clear(
      c.getByRole("spinbutton", { name: "Maximum memory / VRAM" }),
    );
    await userEvent.click(c.getByRole("tab", { name: "Models" }));
    await userEvent.click(trigger);
    await expect(
      portal.getByRole("option", { name: /gemma-4-E4B/ }),
    ).toHaveTextContent("Fits well Recommended Installed");
    await userEvent.keyboard("{Escape}");
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
  }}
/>

<Story
  name="Unknown hardware has no recommendation and custom selection survives"
  args={{ memoryBytes: null, selectedModel: "custom/model:Q8" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    const trigger = await c.findByRole("combobox", {
      name: "Model",
    });
    await expect(trigger).toHaveTextContent(
      "custom/model:Q8 Size unknown Fit unknown",
    );
    await userEvent.click(trigger);
    for (const option of portal.getAllByRole("option")) {
      await expect(option).toHaveTextContent("Fit unknown");
      await expect(option).not.toHaveTextContent("Recommended");
    }
    await userEvent.type(
      portal.getByRole("combobox", { name: "Search local models" }),
      "nonexistent",
    );
    await expect(portal.getByText("No matching models.")).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await expect(trigger).toHaveTextContent("custom/model:Q8");
  }}
/>

<Story
  name="Phone command menu stays bounded and serving locks model selection"
  play={async ({ canvasElement }) => {
    await page.viewport(390, 844);
    try {
      const c = within(canvasElement),
        portal = within(document.body);
      await userEvent.click(await c.findByRole("combobox", { name: "Model" }));
      await portal.findByRole("combobox", { name: "Search local models" });
      const menu = document.querySelector<HTMLElement>(
        ".compute-model-picker__menu",
      )!;
      await expect(menu.getBoundingClientRect().left).toBeGreaterThanOrEqual(0);
      await expect(menu.getBoundingClientRect().right).toBeLessThanOrEqual(390);
      await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
        390,
      );
      await userEvent.keyboard("{Escape}");
    } finally {
      await page.viewport(1280, 800);
    }
  }}
/>
<Story
  name="Running model picker is disabled"
  args={{ serving: true }}
  play={async ({ canvasElement }) => {
    await expect(
      await within(canvasElement).findByRole("combobox", {
        name: "Model",
      }),
    ).toBeDisabled();
  }}
/>
