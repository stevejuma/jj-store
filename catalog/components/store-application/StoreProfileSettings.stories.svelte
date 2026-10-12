<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProfileSettingsFixture from "../../fixtures/ProfileSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Profile and emoji",
    component: ProfileSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Edit profile with keyboard"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: /^Edit$/ }));
    const name = c.getByRole("textbox", { name: "Display name" });
    await userEvent.clear(name);
    await userEvent.type(name, "Updated local admin");
    await userEvent.click(c.getByRole("button", { name: "Save profile" }));
    await waitFor(() =>
      expect(c.getByText("Updated local admin")).toBeVisible(),
    );
    await expect(c.queryByRole("button", { name: "Save profile" })).toBeNull();
  }}
/>
<Story
  name="Unknown profile save retains draft"
  args={{ unknown: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: /^Edit$/ }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Display name" }),
      " draft",
    );
    await userEvent.click(c.getByRole("button", { name: "Save profile" }));
    await expect(
      await c.findByRole("button", { name: "Check save status" }),
    ).toBeVisible();
    await expect(c.getByRole("textbox", { name: "Display name" })).toHaveValue(
      "Local admin draft",
    );
  }}
/>
<Story
  name="Add and remove owned emoji"
  args={{ section: "custom-emoji" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Select fixture image" }),
    );
    await userEvent.type(
      c.getByRole("textbox", { name: "Shortcode" }),
      "party",
    );
    await userEvent.click(c.getByRole("button", { name: "Save emoji" }));
    await expect(await c.findByText(":party:")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Remove :party:" }));
    await waitFor(() => expect(c.queryByText(":party:")).toBeNull());
  }}
/>
<Story name="Unavailable storage" args={{ supported: false }} />
