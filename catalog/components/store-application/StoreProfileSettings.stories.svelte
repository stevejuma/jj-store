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
    await waitFor(async () => {
      await expect(
        await c.findByRole("heading", { name: /^Profile info$/ }),
      ).toBeVisible();
    });
    await expect(
      c.getByRole("button", { name: /^Copy full Principal ID:/ }),
    ).toBeVisible();
    await expect(c.queryByRole("heading", { name: /^Identity$/ })).toBeNull();
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
  name="Avatar actions retain profile draft"
  args={{ withAvatar: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const body = within(document.body);
    await userEvent.click(await c.findByRole("button", { name: /^Edit$/ }));
    const name = c.getByRole("textbox", { name: "Display name" });
    await userEvent.type(name, " draft");
    const clear = c.getByRole("button", { name: "Clear avatar" });
    clear.focus();
    await userEvent.keyboard("{Enter}");
    await expect(c.queryByRole("img", { name: "Selected avatar" })).toBeNull();
    await expect(c.queryByRole("button", { name: "Clear avatar" })).toBeNull();
    await expect(name).toHaveValue("Local admin draft");
    const edit = c.getByRole("button", { name: "Edit avatar" });
    await expect(edit).toBeVisible();
    edit.focus();
    await userEvent.keyboard("{Enter}");
    await waitFor(async () => {
      await expect(
        await body.findByRole("tab", { name: "Image" }),
      ).toBeVisible();
    });
    await expect(name).toHaveValue("Local admin draft");
    await userEvent.keyboard("{Escape}");
    await userEvent.click(c.getByRole("button", { name: /^Cancel$/ }));
    await expect(c.getByRole("img", { name: "Selected avatar" })).toBeVisible();
    await expect(c.getByRole("button", { name: "Clear avatar" })).toBeVisible();
    await expect(c.getByText("Local admin", { exact: true })).toBeVisible();
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
    await waitFor(async () => {
      await expect(
        await c.findByRole("button", { name: "Check save status" }),
      ).toBeVisible();
    });
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
    const shortcode = c.getByRole("textbox", { name: "Shortcode" });
    const group = shortcode.closest('[data-slot="input-group"]')!;
    await expect(
      within(group as HTMLElement).getAllByText(":", { exact: true }),
    ).toHaveLength(2);
    await userEvent.click(
      c.getByRole("button", { name: "Select fixture image" }),
    );
    await userEvent.type(shortcode, "party");
    await expect(shortcode).toHaveValue("party");
    const preview = c.getByRole("img", { name: "Preview of :party:" });
    await expect(preview).toBeVisible();
    await expect(preview.parentElement?.parentElement).toContainElement(
      c.getByRole("button", { name: "Choose emoji image" }),
    );
    await userEvent.click(c.getByRole("button", { name: "Save emoji" }));
    await waitFor(async () => {
      await expect(await c.findByText(":party:")).toBeVisible();
    });
    await userEvent.click(c.getByRole("button", { name: "Remove :party:" }));
    await waitFor(() => expect(c.queryByText(":party:")).toBeNull());
  }}
/>
<Story name="Unavailable storage" args={{ supported: false }} />
