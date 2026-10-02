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
  name="Configuration table controls"
  args={{ section: "configuration" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const enabled = c.getByRole("switch", { name: "Enable text search" });
    const budget = c.getByRole("spinbutton", { name: "Index budget (MiB)" });
    const rows = enabled.closest(".store-settings__rows")!;
    await expect(rows).toContainElement(c.getByText("Currently active"));
    await expect(rows).toContainElement(budget);
    await expect(enabled.getBoundingClientRect().left).toBeGreaterThan(
      c.getByText("Enable text search").getBoundingClientRect().right,
    );
    for (const row of canvasElement.querySelectorAll(
      ".store-settings__action-row",
    )) {
      const description = row.querySelector("p")!,
        actions = row.querySelector(".store-settings__actions")!;
      await expect(
        description.compareDocumentPosition(actions) &
          Node.DOCUMENT_POSITION_FOLLOWING,
      ).toBeTruthy();
      await expect(
        description.getBoundingClientRect().right,
      ).toBeLessThanOrEqual(actions.getBoundingClientRect().left);
    }
    await userEvent.click(enabled);
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(enabled).toBeChecked();
    await expect(budget).toHaveValue(512);
  }}
/>

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
    const c = within(canvasElement);
    const description = c.getByText(
      "Search editing is not available for this connection.",
    );
    await expect(description).toBeVisible();
    const empty = description.closest('[data-slot="empty"]')!;
    await expect(
      empty.querySelector('[data-ui-part="empty-media"] svg'),
    ).toBeVisible();
    await expect(c.queryByRole("button", { name: "Save settings" })).toBeNull();
  }}
/>
