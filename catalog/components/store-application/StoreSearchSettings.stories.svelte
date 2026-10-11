<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
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

<Story
  name="Advanced worker limits"
  args={{ section: "advanced" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const polling = c.getByRole("spinbutton", {
      name: "Polling interval (ms)",
    });
    await waitFor(() =>
      expect(getComputedStyle(polling).pointerEvents).toBe("auto"),
    );
    await userEvent.clear(polling);
    await userEvent.type(polling, "2500");
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(c.getByText(/Saved changes need applying/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Apply saved settings" }),
    );
    await expect(c.getByText(/Matches saved settings/)).toBeVisible();
  }}
/>

<Story
  name="Repository configuration uses bottom actions"
  args={{ embedded: true, section: "configuration" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const row = c
      .getByRole("button", { name: "Save settings" })
      .closest<HTMLElement>(".store-settings__action-row")!;
    const description = row.querySelector("p")!,
      actions = row.querySelector(".store-settings__actions")!;
    await expect(description.getBoundingClientRect().right).toBeLessThanOrEqual(
      actions.getBoundingClientRect().left,
    );
    const options = c.getByRole("region", { name: "Search options" });
    await expect(options).toContainElement(row);
    await expect(
      c.getByRole("region", { name: "Index status" }),
    ).not.toContainElement(row);
    await expect(
      c.getByRole("region", { name: "Index maintenance" }),
    ).toContainElement(c.getByRole("button", { name: "Rebuild text index" }));
    await expect(actions.lastElementChild).toBe(
      c.getByRole("button", { name: "Save settings" }),
    );
    await expect(canvasElement.querySelector("dl button")).toBeNull();
    await expect(
      c
        .getByRole("spinbutton", { name: "Index budget" })
        .getBoundingClientRect().width,
    ).toBeLessThanOrEqual(180);
    await userEvent.click(
      c.getByRole("switch", { name: "Enable text search" }),
    );
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeEnabled();
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(
      c.getByRole("switch", { name: "Enable text search" }),
    ).toBeChecked();
  }}
/>

<Story
  name="Repository worker units save canonical values"
  args={{ embedded: true, section: "advanced" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      c.getByRole("button", { name: "Polling interval unit" }),
    );
    await userEvent.click(page.getByRole("option", { name: "sec" }));
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
    const polling = c.getByRole("spinbutton", {
      name: "Polling interval",
    });
    await waitFor(() =>
      expect(getComputedStyle(polling).pointerEvents).toBe("auto"),
    );
    await userEvent.clear(polling);
    await userEvent.type(polling, "2.5");
    await expect(c.getByText("Exact value: 2,500 ms")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(c.getByText(/Saved changes need applying/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Apply saved settings" }),
    );
    await expect(c.getByText(/Currently active: 2,500 ms/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    );
    await expect(polling).toHaveValue(1);
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(polling).toHaveValue(2.5);
  }}
/>

<Story
  name="Repository index budget uses exact binary units"
  args={{ embedded: true, section: "configuration" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const budget = c.getByRole("spinbutton", { name: "Index budget" });
    await expect(budget).toHaveValue(512);
    await userEvent.click(c.getByRole("button", { name: "Index budget unit" }));
    await userEvent.click(page.getByRole("option", { name: "GiB" }));
    await waitFor(() =>
      expect(getComputedStyle(budget).pointerEvents).toBe("auto"),
    );
    await expect(budget).toHaveValue(0.5);
    await expect(
      c.getByRole("button", { name: "Save settings" }),
    ).toBeDisabled();
    await userEvent.clear(budget);
    await userEvent.type(budget, "1");
    await expect(c.getByText("Exact value: 1,073,741,824 bytes")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Save settings" }));
    await expect(c.getByText(/Saved changes need applying/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Reset Index budget to default" }),
    );
    await expect(budget).toHaveValue(0.5);
    await userEvent.click(c.getByRole("button", { name: "Cancel changes" }));
    await expect(budget).toHaveValue(1);
  }}
/>

<Story
  name="Repository loading becomes a retained editable draft during refresh"
  args={{ embedded: true, loading: true, section: "configuration" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("status", { name: "Loading search settings" }),
    ).toBeVisible();
    await expect(c.queryByRole("switch")).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    const budget = c.getByRole("spinbutton", { name: "Index budget" });
    await userEvent.clear(budget);
    await userEvent.type(budget, "256");
    await userEvent.click(
      c.getByRole("button", { name: "Start background refresh" }),
    );
    await expect(budget).toBeVisible();
    await expect(budget).toHaveValue(256);
    await expect(
      c.queryByRole("status", { name: "Loading search settings" }),
    ).not.toBeInTheDocument();
  }}
/>
