<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import StoreComparisonParentPicker from "../../../web/src/lib/ui/store-application/StoreComparisonParentPicker.svelte";
  import { IDS, operations } from "../../fixtures/repository.js";
  const unknown = "ff".repeat(64);
  const selected = fn();
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-application/StoreComparisonParentPicker.svelte",
      },
    },
    title: "JJ Store/Comparison parent picker",
    component: StoreComparisonParentPicker,
    tags: ["visual-pending", "component"],
    args: {
      parents: [IDS.opLeft, IDS.opRight, unknown],
      revisions: operations.revisions,
      onSelect: selected,
    },
  });
</script>

<Story
  name="Author avatars and independent identity copy"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    selected.mockClear();
    try {
      await userEvent.click(
        canvas.getByRole("combobox", { name: "Comparison parent" }),
      );
      await portal.findByRole("combobox", {
        name: "Search comparison parents",
      });
      await expect(
        portal
          .getByRole("row", { name: /native object history/ })
          .querySelector(".community-member-avatar"),
      ).not.toBeNull();
      const copy = portal.getByRole("button", {
        name: `Copy full operation ID: ${IDS.opLeft}`,
      });
      await userEvent.click(copy);
      await expect(write).toHaveBeenLastCalledWith(IDS.opLeft);
      await waitFor(() => expect(copy).toHaveAttribute("data-copied", "true"));
      await expect(selected).not.toHaveBeenCalled();
      await userEvent.keyboard("{Enter}");
      await expect(write).toHaveBeenCalledTimes(2);
      await expect(selected).not.toHaveBeenCalled();
      write.mockRejectedValueOnce(new Error("Clipboard unavailable"));
      await userEvent.click(copy);
      await waitFor(() =>
        expect(copy).toHaveAttribute("data-copy-failed", "true"),
      );
      await expect(
        portal.getByRole("combobox", { name: "Search comparison parents" }),
      ).toBeVisible();
      await expect(selected).not.toHaveBeenCalled();
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>

<Story
  name="Search and select exact parent"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(canvasElement.ownerDocument.body);
    const trigger = canvas.getByRole("combobox", { name: "Comparison parent" });
    await userEvent.click(trigger);
    const search = await portal.findByRole("combobox", {
      name: "Search comparison parents",
    });
    await waitFor(() => expect(search).toHaveFocus());
    await expect(portal.getAllByRole("row")).toHaveLength(3);
    await userEvent.type(search, "no matching description");
    await expect(
      portal.getByText("No matching comparison parents."),
    ).toBeVisible();
    await userEvent.clear(search);
    await userEvent.type(search, "native object history");
    await expect(portal.getAllByRole("row")).toHaveLength(1);
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(selected).toHaveBeenLastCalledWith(IDS.opLeft);
    await waitFor(() => expect(trigger).toHaveFocus());
    await userEvent.click(trigger);
    await expect(
      await portal.findByRole("combobox", {
        name: "Search comparison parents",
      }),
    ).toHaveValue("");
    await userEvent.type(
      await portal.findByRole("combobox", {
        name: "Search comparison parents",
      }),
      unknown,
    );
    await expect(portal.getAllByRole("row")).toHaveLength(1);
    await userEvent.click(
      portal.getByRole("button", {
        name: "Select comparison parent Operation ffffffffffff",
      }),
    );
    await expect(selected).toHaveBeenLastCalledWith(unknown);
    await userEvent.click(trigger);
    await userEvent.keyboard("{Escape}");
    await expect(trigger).toHaveFocus();
  }}
/>
