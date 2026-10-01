<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import SearchSettingsFixture from "../../fixtures/SearchSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Search configuration",
    component: SearchSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Save then apply"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.type(
      c.getByRole("textbox", { name: "Exclude path prefixes" }),
      "private",
    );
    await expect(
      c.getByRole("button", { name: "Apply saved settings" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(c.getByText(/Saved changes need applying/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Apply saved settings" }),
    );
    await expect(c.getByText(/Matches saved settings/)).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Rebuild text index" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Failure retains draft"
  args={{ fail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const input = c.getByRole("textbox", { name: "Exclude path prefixes" });
    await userEvent.type(input, "private");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(input).toHaveValue("private");
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Refresh settings" }));
    await expect(input).toHaveValue("private");
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(input).toHaveValue("");
  }}
/>
<Story
  name="Unsupported"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByText(
        "Search editing is not available for this connection.",
      ),
    ).toBeVisible();
  }}
/>
