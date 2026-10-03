<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, fn, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ReadOnlyRowFixture.svelte";
  const value = "ab".repeat(64);
  const open = fn();
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Identifier",
    component: Fixture,
    tags: ["visual-pending"],
    args: { value },
  });
  function clipboard(write: ReturnType<typeof fn>) {
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    return () => {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    };
  }
</script>

<Story
  name="Whole row copy and independent navigation"
  args={{ onOpen: open, copyable: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      write = fn(async (_value: string) => {}),
      restore = clipboard(write);
    try {
      const button = canvas.getByRole("button", {
        name: `Copy full Operation ID: ${value}`,
      });
      const label = canvas.getByText("Operation ID", { exact: true });
      const rect = label.getBoundingClientRect();
      const hit = document.elementFromPoint(rect.left + 4, rect.top + 4)!;
      await expect(hit.closest("button")).toBe(button);
      await userEvent.click(hit);
      await expect(write).toHaveBeenCalledWith(value);
      await expect(button).toHaveAttribute("data-copied", "true");
      await expect(button.querySelector("svg circle")).not.toBeNull();
      const display = canvas.getByText(value, { exact: true });
      await expect(display.tagName).not.toBe("CODE");
      await expect(getComputedStyle(display).textOverflow).toBe("ellipsis");
      await userEvent.click(
        canvas.getByRole("button", { name: "Open Operation ID" }),
      );
      await expect(open).toHaveBeenCalledOnce();
      await expect(write).toHaveBeenCalledTimes(1);
      write.mockRejectedValueOnce(new Error("denied"));
      button.focus();
      await userEvent.keyboard("{Enter}");
      await expect(button).toHaveAttribute("data-copy-failed", "true");
      await expect(canvas.getByRole("status")).toHaveTextContent(
        "Could not copy Operation ID. Try again.",
      );
      await userEvent.keyboard(" ");
      await expect(button).toHaveAttribute("data-copied", "true");
      await expect(button).not.toHaveAttribute("data-copy-failed");
      await waitFor(() => expect(button).not.toHaveAttribute("data-copied"), {
        timeout: 2500,
      });
    } finally {
      restore();
    }
  }}
/>
<Story
  name="Missing value"
  args={{ value: null, onOpen: open }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(canvas.getByText("Unavailable")).toBeVisible();
    await expect(canvas.queryByRole("button")).not.toBeInTheDocument();
  }}
/>
<Story
  name="Zero is copyable"
  args={{ value: 0 }}
  play={async ({ canvasElement }) => {
    const write = fn(async (_value: string) => {}),
      restore = clipboard(write);
    try {
      await userEvent.click(
        within(canvasElement).getByRole("button", {
          name: "Copy full Operation ID: 0",
        }),
      );
      await expect(write).toHaveBeenCalledWith("0");
    } finally {
      restore();
    }
  }}
/>
<Story
  name="Value replacement and disposal ignore late clipboard completion"
  args={{ lifecycle: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    let finish!: () => void;
    const write = fn(
        () =>
          new Promise<void>((resolve) => {
            finish = resolve;
          }),
      ),
      restore = clipboard(write);
    try {
      await userEvent.click(
        canvas.getByRole("button", {
          name: `Copy full Operation ID: ${value}`,
        }),
      );
      await userEvent.click(
        canvas.getByRole("button", { name: "Replace value" }),
      );
      finish();
      const replacement = canvas.getByRole("button", {
        name: "Copy full Operation ID: replacement",
      });
      await expect(replacement).not.toHaveAttribute("data-copied");
      await userEvent.click(replacement);
      await userEvent.click(canvas.getByRole("button", { name: "Toggle row" }));
      finish();
      await userEvent.click(canvas.getByRole("button", { name: "Toggle row" }));
      await expect(
        canvas.getByRole("button", {
          name: "Copy full Operation ID: replacement",
        }),
      ).not.toHaveAttribute("data-copied");
    } finally {
      restore();
    }
  }}
/>
