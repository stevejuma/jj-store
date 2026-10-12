<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentFleetApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Shared agent preferences",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function settings(c: ReturnType<typeof within>) {
    await userEvent.click(
      c.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("menuitem", { name: "Settings" }),
    );
    await c.findByRole("navigation", { name: "Settings navigation" });
    await userEvent.click(
      within(
        c.getByRole("navigation", { name: "Settings navigation" }),
      ).getByRole("button", { name: "Agents" }),
    );
    await c.findByRole(
      "button",
      { name: "Save defaults" },
      { timeout: 10_000 },
    );
  }
</script>

<Story
  name="Settings and composer share the saved mention preference"
  args={{ general: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    const editor = await c.findByRole("textbox", { name: "Markdown editor" });
    await userEvent.type(editor, "@Help");
    await userEvent.click(
      await page.findByRole("switch", { name: "Automatically mention agents" }),
    );
    await waitFor(() =>
      expect(
        page.getByRole("switch", { name: "Automatically mention agents" }),
      ).toBeChecked(),
    );
    await userEvent.keyboard("{Escape}");
    await settings(c);
    const toggle = await c.findByRole(
      "switch",
      {
        name: "Automatically mention agents",
      },
      { timeout: 10_000 },
    );
    await expect(toggle).toBeChecked();
    await userEvent.click(toggle);
    await waitFor(() => expect(toggle).not.toBeChecked());
    await userEvent.click(c.getByRole("button", { name: "Back to jj-store" }));
    const retained = await c.findByRole("textbox", { name: "Markdown editor" });
    await expect(retained).toHaveTextContent("@Help");
    await userEvent.type(retained, " ");
    await userEvent.click(
      await c.findByRole("button", { name: "Mention someone" }),
    );
    await expect(
      await page.findByRole("switch", { name: "Automatically mention agents" }),
    ).not.toBeChecked();
    await userEvent.click(
      page.getByRole("button", { name: /^Automatically mention Helper$/ }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("textbox", { name: "Markdown editor" }),
      ).toHaveTextContent("Helper"),
    );
    await userEvent.keyboard("{Escape}");
    await settings(c);
    await expect(
      await c.findByRole("switch", { name: "Automatically mention agents" }),
    ).toBeChecked();
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>

<Story
  name="Settings dropdowns use shadcn keyboard selection and preserve defaults"
  args={{ directory: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await c.findByText("Helper", { exact: true }, { timeout: 10_000 });
    await settings(c);
    const thinking = await c.findByLabelText("Thinking");
    await expect(thinking).toHaveAttribute("data-ui-part", "select-trigger");
    thinking.focus();
    await userEvent.keyboard("{Enter}");
    await userEvent.click(await page.findByRole("option", { name: "High" }));
    await expect(thinking).toHaveTextContent("High");
    await expect(thinking).toHaveFocus();
    await waitFor(() =>
      expect(getComputedStyle(thinking).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(thinking);
    await userEvent.click(
      await within(
        document.body.querySelector<HTMLElement>(
          '[data-ui-part="select-content"][data-state="open"]',
        )!,
      ).findByRole("option", { name: "Harness default" }),
    );
    await expect(thinking).toHaveTextContent("Harness default");
    await waitFor(() =>
      expect(getComputedStyle(thinking).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(c.getByRole("button", { name: "Advanced" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Environment variable name" }),
      "EXAMPLE",
    );
    await userEvent.click(c.getByRole("button", { name: "Add variable" }));
    const source = c.getByRole("button", { name: "Source for EXAMPLE" });
    await userEvent.click(source);
    await expect(
      page.getByRole("option", { name: "Secret in device Keychain" }),
    ).toHaveAttribute("data-disabled");
    await userEvent.keyboard("{Home}{ArrowDown}{ArrowDown}{Enter}");
    await expect(source).toHaveTextContent("Unset inherited variable");
    await expect(c.queryByRole("textbox", { name: "EXAMPLE" })).toBeNull();
    await expect(canvasElement.querySelector("select")).toBeNull();
    await waitFor(() =>
      expect(getComputedStyle(source).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(c.getByRole("button", { name: "Save defaults" }));
    await expect(c.getByLabelText("Agent fleet requests")).toHaveTextContent(
      "[]",
    );
  }}
/>
