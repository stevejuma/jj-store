<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/LocalComputeFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Local compute",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Explicit preparation download start and stop"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(
      c.getByRole("heading", { name: "Compute", level: 1 }),
    ).toBeVisible();
    const start = await c.findByRole("button", { name: "Start model" });
    await expect(start).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Prepare runtime" }));
    await expect(start).toBeDisabled();
    await userEvent.click(c.getByRole("button", { name: "Download model" }));
    await waitFor(() => expect(start).toBeEnabled());
    await userEvent.click(start);
    await waitFor(() => expect(c.getByLabelText("Model")).toBeDisabled());
    await userEvent.click(c.getByRole("button", { name: "Stop compute" }));
    await waitFor(() => expect(c.getByLabelText("Model")).toBeEnabled());
    await expect(
      JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]").map(
        (row: { action: { action: string } }) => row.action.action,
      ),
    ).toEqual([
      "prepare",
      "configure",
      "download",
      "configure",
      "start",
      "stop",
    ]);
  }}
/>

<Story
  name="Rich historical context and exact raw version"
  args={{ context: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const summary = c.getByText("Sent context", { exact: true });
    await expect(
      c.getByText("fn captured_file() {}", { exact: true }),
    ).not.toBeVisible();
    await userEvent.click(summary);
    await expect(
      c.getByText("fn captured_file() {}", { exact: true }),
    ).toBeVisible();
    await expect(
      c.getByText("captured-revision", { exact: true }),
    ).toBeVisible();
    await expect(
      c.getByText("Captured task · version 9007199254740993"),
    ).toBeVisible();
    await expect(c.getByText("Historical", { exact: true }).tagName).toBe(
      "STRONG",
    );
    await userEvent.click(c.getByText("Raw context payload", { exact: true }));
    await expect(
      c.getByText(/9007199254740993/, { selector: "pre" }),
    ).toBeVisible();
  }}
/>
