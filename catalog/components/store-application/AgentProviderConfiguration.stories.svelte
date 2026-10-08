<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentProviderConfigurationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agents/Provider configuration",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function select(label: string, option: string) {
    const p = within(document.body);
    const trigger = p.getByLabelText(label, { exact: true });
    await userEvent.click(trigger);
    await userEvent.click(await p.findByRole("option", { name: option }));
    await waitFor(() =>
      expect(getComputedStyle(trigger).pointerEvents).not.toBe("none"),
    );
  }
</script>

<Story
  name="Add agent discovers unsaved providers and uses Settings rows"
  play={async ({ canvasElement }) => {
    const p = within(document.body),
      c = within(canvasElement);
    const dialog = await p.findByRole("dialog", { name: "Add agent" });
    const advanced = p.getByRole("button", { name: /^Advanced$/ });
    const boundary = advanced.closest(
      '[data-ui-part="agent-advanced-fields"]',
    )!;
    await expect(advanced).toHaveAttribute("aria-expanded", "false");
    await expect(getComputedStyle(boundary).borderTopWidth).toBe("1px");
    await expect(
      parseFloat(getComputedStyle(boundary).borderRadius),
    ).toBeGreaterThan(0);
    await userEvent.click(advanced);
    await userEvent.keyboard("{Enter}");
    await expect(advanced).toHaveAttribute("aria-expanded", "false");
    await expect(advanced).toHaveFocus();
    await expect(
      parseFloat(getComputedStyle(advanced).borderTopLeftRadius),
    ).toBeGreaterThan(0);
    await expect(
      parseFloat(getComputedStyle(advanced).borderBottomLeftRadius),
    ).toBeGreaterThan(0);
    await expect(getComputedStyle(advanced).outlineStyle).toBe("solid");
    await waitFor(() =>
      expect(parseFloat(getComputedStyle(advanced).outlineOffset)).toBeLessThan(
        0,
      ),
    );
    await userEvent.keyboard("{Enter}");
    await waitFor(() =>
      expect(getComputedStyle(advanced).borderBottomLeftRadius).toBe("0px"),
    );
    await userEvent.type(
      p.getByRole("textbox", { name: "Environment variable name" }),
      "RETAIN_DRAFT",
    );
    await userEvent.click(advanced);
    await expect(getComputedStyle(boundary).borderTopWidth).toBe("1px");
    await userEvent.click(advanced);
    await expect(
      p.getByRole("textbox", { name: "Environment variable name" }),
    ).toHaveValue("RETAIN_DRAFT");
    await userEvent.click(advanced);
    const harness = p.getByLabelText("Harness", { exact: true });
    await expect(harness).toHaveTextContent("JJ Agent");
    await expect(
      harness.querySelector('img[src="/agent-icons/jj-store.svg"]'),
    ).not.toBeNull();
    await userEvent.click(harness);
    for (const label of ["JJ Agent", "Codex"]) {
      const option = await p.findByRole("option", { name: label });
      await expect(option.querySelector("svg, img")).not.toBeNull();
    }
    await userEvent.keyboard("{Escape}");

    await userEvent.type(
      p.getByRole("textbox", { name: "Agent name" }),
      "Provider helper",
    );
    await select("LLM provider", "Anthropic");
    await userEvent.click(p.getByLabelText("LLM provider", { exact: true }));
    await p.findByRole("option", {
      name: "Application default (This machine)",
    });
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(c.getByLabelText("Catalog preview requests")).toHaveTextContent(
        '"provider":"anthropic"',
      ),
    );
    await select("Model", "anthropic available model");
    await expect(p.getByRole("combobox", { name: "Model" })).toHaveTextContent(
      "anthropic available model",
    );
    for (const label of [
      "Harness",
      "LLM provider",
      "Model",
      "Execution mode",
    ]) {
      const field = p
        .getByLabelText(label, { exact: true })
        .closest(".community-settings__option-row");
      await expect(field).not.toBeNull();
    }
    const rows = dialog.querySelector(
      '[data-ui-part="agent-configuration-fields"]',
    )!;
    await expect(getComputedStyle(rows).borderTopWidth).toBe("1px");
    await select("LLM provider", "OpenRouter");
    await expect(
      p.getByRole("combobox", { name: "Model" }),
    ).not.toHaveTextContent("anthropic");
    await select("LLM provider", "JJ Store shared compute");
    await p.findByText("Trusted host · trusted");
    await userEvent.click(p.getByRole("button", { name: "Cancel" }));
    await expect(
      c.getByLabelText("Saved agent configuration"),
    ).toHaveTextContent("");
  }}
/>
<Story
  name="Defaults share provider discovery and retain custom models"
  args={{ defaults: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      p = within(document.body);
    await select("LLM provider", "OpenAI-compatible");
    await userEvent.type(
      c.getByRole("textbox", { name: "Provider endpoint" }),
      "https://models.example/v1",
    );
    await select("Model", "Custom model ID…");
    await userEvent.type(
      c.getByRole("textbox", { name: "Custom model ID" }),
      "private/exact-model",
    );
    await userEvent.click(c.getByRole("button", { name: "Done" }));
    await userEvent.click(c.getByRole("button", { name: "Save defaults" }));
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Save defaults" })).toBeEnabled(),
    );
    await expect(c.getByRole("combobox", { name: "Model" })).toHaveTextContent(
      "private/exact-model",
    );
    await expect(p.queryByRole("dialog")).toBeNull();
  }}
/>
<Story
  name="Missing authentication retains drafts and supports keyboard retry"
  args={{ failure: true }}
  play={async ({ canvasElement }) => {
    const p = within(document.body),
      c = within(canvasElement);
    await select("LLM provider", "Anthropic");
    await p.findByText(/Authentication is required/);
    const name = p.getByRole("textbox", { name: "Agent name" });
    await userEvent.type(name, "Keep this draft");
    const retry = p.getByRole("button", { name: "Retry models" });
    await waitFor(() => expect(retry).toBeEnabled());
    retry.focus();
    await userEvent.keyboard("{Enter}");
    await p.findByText(/Authentication is required/);
    await expect(name).toHaveValue("Keep this draft");
    await expect(
      c.getByLabelText("Saved agent configuration"),
    ).toHaveTextContent("");
  }}
/>
