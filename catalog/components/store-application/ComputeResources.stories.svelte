<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/LocalComputeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Compute resources",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Automatic memory remains optional with a joined fixed unit"
  args={{ memoryLimit: null }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    const input = await c.findByRole("spinbutton", {
      name: "Maximum memory / VRAM",
    });
    await expect(input).toHaveValue(null);
    await expect(input).not.toBeRequired();
    await expect(input).not.toHaveAttribute("aria-invalid", "true");
    const slider = c.getByRole("slider", {
      name: "Maximum memory / VRAM slider",
    });
    await expect(slider).toHaveAttribute("aria-valuemin", "1");
    await expect(slider).toHaveAttribute("aria-valuemax", "64");
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
    await userEvent.type(input, "6");
    await expect(slider).toHaveAttribute("aria-valuenow", "6");
    slider.focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(input).toHaveValue(7);
    await userEvent.clear(input);
    await userEvent.click(c.getByRole("tab", { name: "Models" }));
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
      ).toMatchObject([
        { action: { action: "configure", settings: { memory_gb: null } } },
      ]),
    );
  }}
/>
<Story
  name="Memory slider preserves larger typed limits and integer validation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    const input = c.getByRole("spinbutton", { name: "Maximum memory / VRAM" });
    const slider = c.getByRole("slider", {
      name: "Maximum memory / VRAM slider",
    });
    await userEvent.clear(input);
    await userEvent.type(input, "512");
    await expect(slider).toHaveAttribute("aria-valuemax", "512");
    await expect(input).toHaveValue(512);
    await userEvent.click(
      c.getByRole("button", { name: "Save compute settings" }),
    );
    await waitFor(() =>
      expect(
        JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]"),
      ).toMatchObject([
        { action: { action: "configure", settings: { memory_gb: 512 } } },
      ]),
    );
    await userEvent.clear(input);
    await userEvent.type(input, "1.5");
    await expect(input).toHaveAttribute("aria-invalid", "true");
    await expect(slider).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(c.getByRole("tab", { name: "Models" }));
    await expect(
      c.getByRole("button", { name: "Save compute settings" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Running compute disables memory input and slider"
  args={{ serving: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("tab", { name: "Resources" }));
    await expect(
      c.getByRole("spinbutton", { name: "Maximum memory / VRAM" }),
    ).toBeDisabled();
    const slider = c.getByRole("slider", {
      name: "Maximum memory / VRAM slider",
    });
    await expect(slider).toHaveAttribute("aria-disabled", "true");
    slider.focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
  }}
/>
