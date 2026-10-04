<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreRepositoryConversionDialog from "../../../web/src/lib/ui/store-settings/StoreRepositoryConversionDialog.svelte";
  import "../../../web/src/lib/ui/store-work/store-work.css";
  const op = "22".repeat(64);
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Repository conversion",
    component: StoreRepositoryConversionDialog,
    tags: ["visual-pending"],
    args: {
      open: true,
      operation: op,
      operationHeads: [op],
      name: "Independent",
      local: true,
      onEdit: () => {},
      onCheck: () => {},
      onStart: () => {},
      onStatus: () => {},
      onDownload: () => {},
    },
  });
  const dialog = (canvasElement: HTMLElement) =>
    within(canvasElement.ownerDocument.body).findByRole("dialog", {
      name: "Convert repository",
    });
</script>

<Story
  name="Server destination"
  play={async ({ canvasElement }) => {
    const c = within(await dialog(canvasElement));
    const name = c.getByRole("textbox", { name: "Destination name" });
    await userEvent.clear(name);
    await userEvent.type(name, "Server copy");
    await expect(name).toHaveValue("Server copy");
    await expect(c.queryByRole("textbox", { name: "New folder" })).toBeNull();
    await expect(
      c.getByRole("button", { name: "Check conversion" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Local destination requires new folder"
  args={{ local: false }}
  play={async ({ canvasElement }) => {
    const c = within(await dialog(canvasElement));
    await expect(
      c.getByRole("button", { name: "Check conversion" }),
    ).toBeDisabled();
    await userEvent.type(
      c.getByRole("textbox", { name: "New folder" }),
      "/new/folder",
    );
    await expect(
      c.getByRole("button", { name: "Check conversion" }),
    ).toBeEnabled();
  }}
/>
<Story
  name="Unsupported evidence blocks conversion"
  args={{
    preview: {
      version: 1,
      prepared_source: null,
      source_bytes: 0,
      history_selections: 0,
      blocker: "Unsupported signed commit",
    },
  }}
  play={async ({ canvasElement }) => {
    const c = within(await dialog(canvasElement));
    await expect(
      c.getByText("Conversion is unavailable: Unsupported signed commit"),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Convert and open destination" }),
    ).toBeNull();
  }}
/>
<Story
  name="Pending request locks original destination"
  args={{
    pending: true,
    local: false,
    pendingIntent: {
      version: 1,
      source: "11".repeat(16),
      source_profile: "local_jj_git",
      operation: op,
      destination_profile: "native",
      name: "Original",
      path: null,
    },
  }}
  play={async ({ canvasElement }) => {
    const c = within(await dialog(canvasElement));
    await expect(
      c.getByRole("textbox", { name: "Destination name" }),
    ).toBeDisabled();
    await expect(c.queryByRole("textbox", { name: "New folder" })).toBeNull();
    await expect(c.getByRole("button", { name: "Check status" })).toBeEnabled();
    await expect(
      c.getByRole("button", { name: "Retry conversion" }),
    ).toBeEnabled();
  }}
/>
