<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StorageSettingsFixture from "../../fixtures/StorageSettingsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Storage",
    component: StorageSettingsFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Fixed checkpoint target"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByText("Selected manifest is readable")).toBeVisible();
    await expect(
      c
        .getByText("Selected manifest is readable")
        .closest(".store-settings__rows"),
    ).not.toBeNull();
    await expect(
      c.getByRole("button", { name: "Refresh storage" }),
    ).toHaveTextContent("");
    c.getByRole("tab", { name: "Overview" }).focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(c.getByRole("tab", { name: "Maintenance" })).toHaveFocus();
    await userEvent.keyboard("{Enter}");
    await expect(
      c.getByRole("button", { name: "Check integrity" }),
    ).toBeDisabled();
    await userEvent.click(
      c.getByRole("button", { name: "Publish checkpoint" }),
    );
    await expect(
      c.getByText("The exact captured checkpoint is selected."),
    ).toBeVisible();
  }}
/>
<Story
  name="Incomplete check"
  args={{ phase: "incomplete" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByText(
        "The check did not verify all captured data within its limits.",
      ),
    ).toBeVisible();
    await expect(
      c.queryByText("Captured data verified"),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Interrupted check"
  args={{ phase: "interrupted" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByText(/The service stopped during this check/),
    ).toBeVisible();
  }}
/>
<Story
  name="Unknown checkpoint"
  args={{ phase: "unknown" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByRole("button", { name: "Retry checkpoint" }),
    ).toBeEnabled();
    await expect(
      c.queryByRole("button", { name: "Cancel checkpoint" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Unsupported connection"
  args={{ supported: false }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByText("Storage unavailable")).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Check integrity" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Repository overview separates readiness and position"
  args={{ embedded: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("region", { name: "Storage readiness" }),
    ).toHaveAttribute("data-slot", "card");
    await expect(
      c.getByRole("region", { name: "Published position" }),
    ).toHaveAttribute("data-slot", "card");
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByRole("button", { name: "Publish checkpoint" }),
    ).toBeEnabled();
    await expect(
      c.getByRole("region", { name: "Maintenance operation" }),
    ).toHaveAttribute("data-slot", "card");
  }}
/>

<Story
  name="Repository initial load follows subtabs and refresh retains captured target"
  args={{ embedded: true, loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("status", { name: "Loading storage overview" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByRole("status", { name: "Loading storage maintenance" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Check integrity" }),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Finish loading" }));
    const publish = c.getByRole("button", { name: "Publish checkpoint" });
    await expect(publish).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Start background refresh" }),
    );
    await expect(publish).toBeVisible();
    await expect(
      c.queryByRole("status", { name: "Loading storage maintenance" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Retained checkpoint report is never replaced by initial placeholders"
  args={{ embedded: true, loading: true, retainedOperation: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Maintenance" }));
    await expect(
      c.getByRole("button", { name: "Publish checkpoint" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("status", { name: "Loading storage maintenance" }),
    ).not.toBeInTheDocument();
  }}
/>
