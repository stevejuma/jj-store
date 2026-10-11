<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/RepositorySettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Native categories are scoped with no picker"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("heading", { name: "Identity and snapshot" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("navigation", { name: "Settings page navigation" }),
    ).not.toBeInTheDocument();
    await expect(
      canvasElement.querySelector(".store-settings__page-title"),
    ).toBeNull();
    for (const name of [
      "General",
      "Local folders",
      "Search",
      "Storage",
      "Sync",
      "Advanced",
    ])
      await expect(c.getByRole("tab", { name })).toBeVisible();
    await expect(
      c.queryByRole("combobox", { name: "Settings repository" }),
    ).not.toBeInTheDocument();
    await expect(c.getByText("Server Hosted", { exact: true })).toHaveAttribute(
      "data-chip-tone",
      "accent",
    );
  }}
/>
<Story
  name="Local JJ omits unsupported management"
  args={{ local: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByText("Local JJ", { exact: true })).toHaveAttribute(
      "data-chip-tone",
      "success",
    );
    for (const name of ["Search", "Storage", "Sync", "Protection"])
      await expect(c.queryByRole("tab", { name })).not.toBeInTheDocument();
    await expect(c.queryByText("Unavailable")).not.toBeInTheDocument();
  }}
/>
<Story
  name="Reader search link resolves to Health"
  args={{ reader: true, section: "search", tab: "scopes" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByRole("tab", { name: "Health" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      c.getByRole("status", { name: "Settings route" }),
    ).toHaveTextContent("settings:search:health");
    await expect(
      c.queryByRole("tab", { name: "Configuration" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Native folder draft survives categories"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Local folders" }));
    await userEvent.type(
      c.getByRole("textbox", { name: "Folder path" }),
      "/retained/draft",
    );
    await userEvent.click(c.getByRole("tab", { name: "General" }));
    await userEvent.click(c.getByRole("tab", { name: "Local folders" }));
    await expect(c.getByRole("textbox", { name: "Folder path" })).toHaveValue(
      "/retained/draft",
    );
  }}
/>
<Story
  name="Unsupported category returns to General"
  args={{ local: true, section: "storage" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      await c.findByRole("tab", { name: "General" }),
    ).toHaveAttribute("aria-selected", "true");
    await expect(
      c.getByRole("status", { name: "Settings route" }),
    ).toHaveTextContent("settings:general:");
  }}
/>
