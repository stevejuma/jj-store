<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import ServerSettingsFixture from "../../fixtures/ServerSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Server configuration",
    component: ServerSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Service limits"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("spinbutton", {
      name: "Concurrent requests",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "3");
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a server restart."),
    ).toBeVisible();
    const field = input.closest('[data-slot="field"]')!;
    await expect(field.getBoundingClientRect().left).toBeLessThan(
      input.getBoundingClientRect().left,
    );
    await expect(input.closest(".store-settings__rows")).not.toBeNull();
  }}
/>
<Story
  name="Installation protection"
  args={{ section: "protection" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("textbox", {
      name: "Protected bookmarks",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "main\nrelease");
    await expect(input).toHaveValue("main\nrelease");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a server restart."),
    ).toBeVisible();
  }}
/>
<Story
  name="Upstream configuration"
  args={{ section: "sync" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Add upstream" }),
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Upstream origin" }),
      "https://upstream.example/",
    );
    await expect(
      c.getByRole("spinbutton", { name: "Origins per batch" }),
    ).toHaveValue(128);
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(
      c.getByText("Saved changes need a server restart."),
    ).toBeVisible();
  }}
/>
<Story
  name="Unknown response retains draft"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = await c.findByRole("spinbutton", {
      name: "Request timeout (seconds)",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "18");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(input).toHaveValue(18);
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Refresh settings" }));
    await userEvent.click(
      c.getByRole("button", { name: "Use latest saved settings" }),
    );
    await expect(input).toHaveValue(30);
  }}
/>
<Story
  name="Unavailable"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByText(
        "Configuration editing requires a local service with administrator access.",
      ),
    ).toBeVisible();
    await expect(c.queryByRole("button", { name: "Save settings" })).toBeNull();
  }}
/>
