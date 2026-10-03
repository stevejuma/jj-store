<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreLocalFolderDialog from "../../../web/src/lib/ui/store-settings/StoreLocalFolderDialog.svelte";
  import "../../../web/src/lib/ui/store-work/store-work.css";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Folder dialog",
    component: StoreLocalFolderDialog,
    tags: ["visual-pending"],
    args: {
      open: true,
      reconnect: false,
      busy: false,
      pending: false,
      inspected: null,
      message: "Choose a folder to inspect.",
      onPathChange: () => {},
      onBrowse: () => {},
      onInspect: () => {},
      onSave: () => {},
    },
  });
</script>

<Story
  name="Bordered full-width folder control"
  play={async ({ canvasElement }) => {
    const body = within(canvasElement.ownerDocument.body);
    const dialog = await body.findByRole("dialog", {
      name: "Add local JJ folder",
    });
    const input = within(dialog).getByRole("textbox", { name: "Folder" });
    const group = input.closest('[data-ui-component="button-group"]')!;
    await expect(getComputedStyle(input).borderTopWidth).toBe("1px");
    await expect(group.getBoundingClientRect().width).toBeCloseTo(
      group.parentElement!.getBoundingClientRect().width,
      0,
    );
    await expect(within(dialog).getByRole("status")).toHaveAttribute(
      "data-ui-component",
      "alert",
    );
    await userEvent.type(input, "/workspaces/example");
    await expect(input).toHaveValue("/workspaces/example");
    await expect(
      within(dialog).getByRole("button", { name: "Inspect folder" }),
    ).toBeEnabled();
  }}
/>
