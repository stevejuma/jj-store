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
      page = within(canvasElement.ownerDocument.body),
      button = c.getByRole("button", { name: "Prepare operation inventory" });
    await expect(button.closest(".store-settings__action-row")).not.toBeNull();
    button.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await page.findByText("Operation inventory preparation completed."),
    ).toBeVisible();
    await expect(c.queryByText(/matching identities/)).not.toBeInTheDocument();
    await expect(
      c.getByRole("button", { name: "Prepare graph index" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Cancel running copy inventory"
  args={{ phase: "running" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      c.getByRole("button", { name: "Prepare copy inventory" }),
    );
    await expect(
      c.getByRole("button", { name: "Prepare operation inventory" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("button", { name: "Prepare graph index" }),
    ).toBeDisabled();
    await expect(c.queryByText(/matching identities/)).not.toBeInTheDocument();
    await userEvent.click(
      page.getByRole("button", { name: "Cancel preparation" }),
    );
    await expect(
      page.getByText("Copy inventory preparation canceled."),
    ).toBeVisible();
  }}
/>
<Story
  name="Retry unconfirmed cancellation"
  args={{ phase: "unknown_cancel" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      c.getByRole("button", { name: "Prepare operation inventory" }),
    );
    await userEvent.click(
      page.getByRole("button", { name: "Cancel preparation" }),
    );
    await expect(page.getByText(/Cancellation is unconfirmed/)).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Retry cancellation" }),
    ).toBeEnabled();
    await userEvent.click(
      page.getByRole("button", { name: "Retry cancellation" }),
    );
    await expect(
      page.getByText("Operation inventory preparation canceled."),
    ).toBeVisible();
  }}
/>
<Story
  name="Local profile retains graph preparation only"
  args={{ local: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
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

<Story
  name="Automatic history maintenance"
  args={{ automatic: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await expect(c.getByText("Automatic")).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Prepare graph index" }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByRole("button", { name: "Prepare operation inventory" }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByRole("button", { name: "Prepare copy inventory" }),
    ).not.toBeInTheDocument();
  }}
/>
