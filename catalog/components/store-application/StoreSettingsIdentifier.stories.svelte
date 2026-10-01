<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, fn } from "storybook/test";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import "../../../web/src/lib/ui/store-settings/StoreSettings.css";
  const value = "ab".repeat(64);
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Identifier",
    component: CopyableValue,
    tags: ["visual-pending"],
    args: { label: "Operation ID", value },
  });
</script>

<Story
  name="Copy full ID with compact shared feedback"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.queryByRole("textbox")).not.toBeInTheDocument();
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    const write = fn(async (_value: string) => {});
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      const button = canvas.getByRole("button", {
        name: `Copy full Operation ID: ${value}`,
      });
      await expect(button).toHaveTextContent("abababab…abab");
      await expect(button.getBoundingClientRect().height).toBeLessThan(30);
      button.focus();
      await userEvent.keyboard("{Enter}");
      await expect(write).toHaveBeenCalledWith(value);
      await expect(button).toHaveAttribute("data-copied", "true");
      write.mockRejectedValueOnce(new Error("denied"));
      await userEvent.click(button);
      await expect(button).toHaveAttribute("data-copy-failed", "true");
      await expect(button).toHaveAccessibleName(
        `Copy full Operation ID: ${value}`,
      );
      await userEvent.click(button);
      await expect(button).toHaveAttribute("data-copied", "true");
      await expect(button).not.toHaveAttribute("data-copy-failed");
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
>
  {#snippet template(args)}<main class="store-settings">
      <CopyableValue {...args} />
    </main>{/snippet}
</Story>

<Story
  name="Missing value"
  args={{ value: null, emptyLabel: "Unavailable" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("Unavailable")).toBeVisible();
    await expect(canvas.queryByRole("button")).not.toBeInTheDocument();
  }}
>
  {#snippet template(args)}<main class="store-settings">
      <CopyableValue {...args} />
    </main>{/snippet}
</Story>
