<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import CaptureSettingsFixture from "../../fixtures/CaptureSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Multi harness capture",
    component: CaptureSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Independent Cursor and OpenCode settings save and reload"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Configuration" }));
    for (const name of ["Codex", "Cursor", "OpenCode"]) {
      const group = await c.findByRole("button", {
        name: `${name} configuration`,
      });
      await expect(group).toHaveAttribute("aria-expanded", "false");
      await expect(group.querySelector("img")).not.toBeNull();
      await userEvent.click(group);
      await expect(group).toHaveAttribute("aria-expanded", "true");
    }
    await userEvent.click(
      c.getByRole("switch", { name: "Cursor Desktop capture" }),
    );
    await userEvent.click(
      c.getByRole("switch", { name: "Cursor CLI capture" }),
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Cursor Desktop folders" }),
      "/tmp/cursor-desktop/projects",
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Cursor CLI folders" }),
      "/tmp/cursor-cli/projects",
    );
    await userEvent.click(c.getByRole("switch", { name: "OpenCode capture" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "OpenCode API endpoint" }),
      "http://127.0.0.1:4096/",
    );
    await userEvent.click(
      c.getByRole("button", { name: "Cursor configuration" }),
    );
    await waitFor(() =>
      expect(
        c.queryByRole("textbox", { name: "Cursor CLI folders" }),
      ).toBeNull(),
    );
    await expect(
      c.getByRole("spinbutton", { name: "Concurrent readers" }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Save configuration" }),
    );
    await expect(
      await within(canvasElement.ownerDocument.body).findByText(
        /Configuration saved. Use Reload saved settings/,
      ),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Refresh capture" }));
    await userEvent.click(
      c.getByRole("button", { name: "Cursor configuration" }),
    );
    await expect(
      c.getByRole("textbox", { name: "Cursor CLI folders" }),
    ).toHaveValue("/tmp/cursor-cli/projects");
    await expect(
      c.getByRole("textbox", { name: "OpenCode API endpoint" }),
    ).toHaveValue("http://127.0.0.1:4096/");
    await expect(
      c.getByRole("switch", { name: "Codex CLI capture" }),
    ).toBeChecked();
  }}
/>
