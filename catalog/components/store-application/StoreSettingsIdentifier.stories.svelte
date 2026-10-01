<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, fn } from "storybook/test";
  import StoreSettingsIdentifier from "../../../web/src/lib/ui/store-settings/StoreSettingsIdentifier.svelte";
  import "../../../web/src/lib/ui/store-settings/StoreSettings.css";
  const value = "ab".repeat(64);
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Identifier",
    component: StoreSettingsIdentifier,
    tags: ["visual-pending"],
    args: { label: "Operation ID", value },
  });
</script>

<Story
  name="Copy full ID and preserve manual fallback"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      const input = canvas.getByRole("textbox", { name: "Operation ID" });
      const button = canvas.getByRole("button", { name: "Copy Operation ID" });
      await expect(input).toHaveValue(value);
      await expect(input).toHaveAttribute("readonly");
      await expect(
        Math.abs(
          input.getBoundingClientRect().right -
            button.getBoundingClientRect().left,
        ),
      ).toBeLessThan(1);
      await userEvent.click(button);
      await expect(write).toHaveBeenCalledWith(value);
      await expect(canvas.getByRole("status")).toHaveTextContent(
        "Operation ID copied",
      );
      write.mockRejectedValueOnce(new Error("denied"));
      await userEvent.click(button);
      await expect(canvas.getByRole("status")).toHaveTextContent(
        "Could not copy",
      );
      await expect(input).toHaveValue(value);
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
>
  {#snippet template(args)}<main class="store-settings">
      <StoreSettingsIdentifier {...args} />
    </main>{/snippet}
</Story>
