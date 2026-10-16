<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, waitFor, within } from "storybook/test";
  import RepositoryPickerPendingFixture from "../../fixtures/RepositoryPickerPendingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Application/Repository picker pending",
    component: RepositoryPickerPendingFixture,
    tags: ["visual-pending"],
    args: { onChange: fn() },
  });
</script>

<Story
  name="Open options pause and resume with pending reads"
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(canvas.getByRole("combobox", { name: "Repository" }));
    const toggle = canvas.getByRole("button", { name: "Toggle pending read" });
    // Simulate an external read without dismissing the already open popover.
    toggle.click();
    await waitFor(() => {
      for (const item of body.getAllByRole("option"))
        expect(item).toHaveAttribute("aria-disabled", "true");
    });
    await waitFor(() =>
      expect(body.getByRole("option", { name: /Parser/ })).toBeVisible(),
    );
    await userEvent.click(body.getByRole("option", { name: /Parser/ }));
    expect(args.onChange).not.toHaveBeenCalled();
    expect(body.getByRole("option", { name: /Parser/ })).toBeVisible();
    toggle.click();
    await waitFor(() =>
      expect(body.getByRole("option", { name: /Parser/ })).not.toHaveAttribute(
        "aria-disabled",
        "true",
      ),
    );
    await userEvent.click(body.getByRole("option", { name: /Parser/ }));
    expect(args.onChange).toHaveBeenCalledWith("41".repeat(16));
  }}
/>
