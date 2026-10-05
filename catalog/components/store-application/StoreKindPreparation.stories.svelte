<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import KindPreparationFixture from "../../fixtures/KindPreparationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Inventory preparation",
    component: KindPreparationFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Complete selected inventory"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      button = c.getByRole("button", { name: "Prepare operation inventory" });
    await expect(button.closest(".store-settings__action-row")).not.toBeNull();
    button.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await c.findByText("Operation inventory preparation completed."),
    ).toBeVisible();
    await expect(c.getByText(/2 matching identities/)).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Prepare graph index" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Cancel running copy inventory"
  args={{ phase: "running" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Prepare copy inventory" }),
    );
    await expect(
      c.getByRole("button", { name: "Prepare operation inventory" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("button", { name: "Prepare graph index" }),
    ).toBeDisabled();
    await expect(c.getByText(/2 matching identities/)).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Cancel preparation" }),
    );
    await expect(
      c.getByText("Copy inventory preparation canceled."),
    ).toBeVisible();
  }}
/>
<Story
  name="Retry unconfirmed cancellation"
  args={{ phase: "unknown_cancel" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      c.getByRole("button", { name: "Prepare operation inventory" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Cancel preparation" }),
    );
    await expect(c.getByText(/Cancellation is unconfirmed/)).toBeVisible();
    await expect(
      c.getByRole("button", { name: "Cancel preparation" }),
    ).toBeEnabled();
    await userEvent.click(
      c.getByRole("button", { name: "Cancel preparation" }),
    );
    await expect(
      c.getByText("Operation inventory preparation canceled."),
    ).toBeVisible();
  }}
/>
<Story
  name="Local profile retains graph preparation only"
  args={{ local: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("button", { name: "Prepare graph index" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Prepare operation inventory" }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByRole("button", { name: "Prepare copy inventory" }),
    ).not.toBeInTheDocument();
  }}
/>
