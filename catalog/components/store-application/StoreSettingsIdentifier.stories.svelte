<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, fn, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ReadOnlyRowFixture.svelte";
  const value = "ab".repeat(64);
  const open = fn();
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-settings/StoreReadOnlyRow.svelte",
      },
    },
    title: "JJ Store/Settings/Identifier",
    component: Fixture,
    tags: ["visual-pending", "component"],
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
      const link = canvas.getByRole("button", { name: "Open Operation ID" });
      expect(link.getBoundingClientRect().right).toBeLessThanOrEqual(
        button.getBoundingClientRect().left,
      );
      expect(button.parentElement!.lastElementChild).toBe(button);
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
      const button = within(canvasElement).getByRole("button", {
        name: "Copy full Operation ID: 0",
      });
      expect(button.parentElement!.getBoundingClientRect().width).toBe(
        button.getBoundingClientRect().width,
      );
      await userEvent.click(button);
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

<Story
  name="Repository copyable values truncate and copy the exact full identifier"
  args={{ presentation: "repository", onOpen: open }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      write = fn(async (_value: string) => {}),
      restore = clipboard(write);
    const main = canvasElement.querySelector("main")!;
    main.style.width = "320px";
    try {
      const text = c.getByText(value, { exact: true });
      await expect(getComputedStyle(text).whiteSpace).toBe("nowrap");
      await expect(getComputedStyle(text).textOverflow).toBe("ellipsis");
      await expect(text).toHaveAttribute("title", value);
      await expect(text.getBoundingClientRect().height).toBeLessThanOrEqual(
        parseFloat(getComputedStyle(text).lineHeight),
      );
      const copy = c.getByRole("button", {
        name: `Copy full Operation ID: ${value}`,
      });
      const link = c.getByRole("button", { name: "Open Operation ID" });
      await expect(copy).toHaveAttribute("data-size", "icon-xs");
      await expect(link).toHaveAttribute("data-size", "icon-xs");
      const centre = (element: Element) => {
        const bounds = element.getBoundingClientRect();
        return bounds.top + bounds.height / 2;
      };
      await expect(Math.abs(centre(text) - centre(copy))).toBeLessThan(1);
      await expect(Math.abs(centre(text) - centre(link))).toBeLessThan(1);
      await expect(copy.getBoundingClientRect().height).toBeLessThanOrEqual(24);
      await userEvent.click(copy);
      await expect(write).toHaveBeenCalledWith(value);
      await expect(copy).toHaveAttribute("data-copied", "true");
      await userEvent.click(
        c.getByRole("button", { name: "Open Operation ID" }),
      );
      await expect(open).toHaveBeenCalledOnce();
      await expect(write).toHaveBeenCalledTimes(1);
      await expect(main.scrollWidth).toBeLessThanOrEqual(320);
    } finally {
      restore();
    }
  }}
/>
