<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow console",
    component: WorkflowsFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function closeMenu() {
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  }
  const selected = `#/workflows?tab=history&home=${"41".repeat(16)}&run=${"51".repeat(16)}&at=1`;
</script>

<Story
  name="Recorded step has a Forgejo console and collapsed inline output"
  beforeEach={async () => {
    if (import.meta.env.MODE !== "test") return;
    const { vi } = await import("vitest");
    const clock = vi.spyOn(Date, "now").mockReturnValue(1792052800000);
    return () => clock.mockRestore();
  }}
  args={{ initialHash: selected }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("region", { name: "Step console" }),
    ).toBeVisible();
    const step = await canvas.findByRole("button", {
      name: "Step build · completed",
    });
    await expect(step).toHaveAttribute("aria-expanded", "false");
    const console = canvas.getByRole("region", { name: "Step console" });
    await expect(getComputedStyle(console).backgroundColor).toBe(
      "rgb(31, 33, 43)",
    );
    await expect(
      console.querySelectorAll(".workflow-console-step[data-step]"),
    ).toHaveLength(1);
    const gear = canvas.getByRole("button", { name: "Log settings" });
    await expect(gear.getBoundingClientRect().right).toBeLessThanOrEqual(
      console.getBoundingClientRect().right,
    );
    await expect(
      canvas.getByRole("combobox", { name: "Run attempt" }),
    ).toHaveTextContent(/Run attempt #1 \(.+ ago\)/);
    await userEvent.click(canvas.getByRole("button", { name: "Run output" }));
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "12 tests passed",
    );
  }}
/>

<Story
  name="Three actions load logs on demand and retain expansion across refresh"
  args={{ initialHash: selected, consoleMode: "success", logReadDelay: 200 }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const prepare = await canvas.findByRole("button", {
      name: "Step prepare · completed",
    });
    await expect(canvas.getByLabelText("Log read count")).toHaveTextContent(
      "0",
    );
    await expect(
      canvasElement.querySelectorAll(".workflow-console-step[data-step]"),
    ).toHaveLength(3);
    await userEvent.click(prepare);
    await expect(
      await canvas.findByLabelText("Loading logs for prepare"),
    ).toBeVisible();
    const line = await canvas.findByLabelText("stdout log");
    await expect(line).toHaveTextContent("Pinned source ready");
    await expect(line.querySelector(".ansi-green-fg")).not.toBeNull();
    await expect(getComputedStyle(line).lineHeight).toBe("20px");
    await expect(
      getComputedStyle(line.querySelector(".workflow-console-line__number")!)
        .width,
    ).toBe("48px");
    await userEvent.keyboard("{Enter}");
    await expect(prepare).toHaveAttribute("aria-expanded", "false");
    await userEvent.keyboard("{Enter}");
    await expect(canvas.getByLabelText("Log read count")).toHaveTextContent(
      "1",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Refresh run" }));
    await waitFor(() =>
      expect(
        canvas.getByRole("button", { name: "Step prepare · completed" }),
      ).toHaveAttribute("aria-expanded", "true"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Step test · completed" }),
    );
    const group = await canvas.findByRole("button", { name: "Test group" });
    await expect(canvas.getByText(/^Nested output/)).not.toBeVisible();
    await userEvent.click(group);
    await expect(canvas.getByText(/^Nested output/)).toBeVisible();
    await userEvent.keyboard(" ");
    await expect(canvas.getByText(/^Nested output/)).not.toBeVisible();
    await userEvent.keyboard("{Enter}");
    await expect(canvas.getByText(/^Nested output/)).toBeVisible();
    await expect(canvas.getByLabelText("stderr log")).toHaveTextContent(
      "⚙ [runner]: diagnostic",
    );
    await userEvent.click(canvas.getByRole("button", { name: "Log settings" }));
    await expect(
      await within(document.body).findByRole("menuitem", {
        name: "Download demo-result.txt",
      }),
    ).toBeVisible();
    await closeMenu();
    await waitFor(() =>
      expect(within(document.body).queryByRole("menu")).toBeNull(),
    );
  }}
/>

<Story
  name="Console settings show times and full screen preserves selected steps"
  args={{ initialHash: selected, consoleMode: "success" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Step prepare · completed" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Log settings" }));
    await userEvent.click(
      await body.findByRole("menuitemcheckbox", { name: "Show seconds" }),
    );
    await userEvent.click(
      body.getByRole("menuitemcheckbox", { name: "Show timestamps" }),
    );
    await closeMenu();
    await expect(
      canvasElement.querySelector(".workflow-console-line__seconds"),
    ).toHaveTextContent("0s");
    await expect(
      canvasElement.querySelector(".workflow-console-line__time"),
    ).toBeVisible();
    await waitFor(() =>
      expect(
        getComputedStyle(canvas.getByRole("button", { name: "Log settings" }))
          .pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Log settings" }));
    await userEvent.click(
      await body.findByRole("menuitemcheckbox", { name: "Show full screen" }),
    );
    const fullscreen = within(
      await body.findByRole("dialog", { name: "Workflow logs full screen" }),
    );
    await expect(await fullscreen.findByLabelText("stdout log")).toBeVisible();
    await expect(
      fullscreen.getByRole("button", { name: "Step prepare · completed" }),
    ).toHaveAttribute("aria-expanded", "true");
    const fullPanel = body.getByRole("dialog", {
      name: "Workflow logs full screen",
    });
    const exit = fullscreen.getByRole("button", { name: "Exit full screen" });
    const bounds = exit.getBoundingClientRect();
    await expect(
      fullPanel.contains(
        document.elementFromPoint(
          bounds.x + bounds.width / 2,
          bounds.y + bounds.height / 2,
        ),
      ),
    ).toBe(true);
    await userEvent.click(
      fullscreen.getByRole("button", { name: "Exit full screen" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toBeVisible();
    await waitFor(() =>
      expect(
        getComputedStyle(canvas.getByRole("button", { name: "Log settings" }))
          .pointerEvents,
      ).not.toBe("none"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Log settings" }));
    await expect(
      await body.findByRole("menuitem", { name: "Download logs" }),
    ).toBeVisible();
    await closeMenu();
  }}
/>

<Story
  name="Historical attempts stay within the console and keep exact URL pins"
  args={{ initialHash: selected, consoleMode: "success" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const picker = await canvas.findByRole("combobox", { name: "Run attempt" });
    await expect(picker.closest(".workflow-console-header")).not.toBeNull();
    await userEvent.click(picker);
    await userEvent.click(
      await body.findByRole("option", { name: /Run attempt #1/ }),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "Step prepare · failed" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "Old attempt only",
    );
    await expect(canvas.queryByText("Pinned source ready")).toBeNull();
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      `attempt=${"52".repeat(16)}`,
    );
  }}
/>

<Story
  name="Failed and cancelled steps keep their outcome without inventing completion"
  args={{ initialHash: selected, consoleMode: "failed" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("button", { name: "Step test · failed" }),
    ).toHaveAttribute("aria-expanded", "false");
    await expect(
      canvas.getByRole("button", { name: "Step summary · not run" }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Step test · failed" }),
    );
    await expect(await canvas.findByLabelText("stderr log")).toHaveTextContent(
      "diagnostic",
    );
  }}
/>

<Story
  name="Cancellation and active execution retain native statuses"
  args={{ initialHash: selected, consoleMode: "cancelled" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("button", { name: "Step test · cancelled" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Step summary · not run" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Active step remains running with later actions pending"
  args={{ initialHash: selected, consoleMode: "running" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      await canvas.findByRole("button", { name: "Step test · running" }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Step summary · pending" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Paged logs load the selected step before later evidence"
  args={{ initialHash: selected, consoleMode: "paged" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Step prepare · pending" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "Pinned source ready",
    );
    await expect(
      canvas.getByRole("button", { name: "Step prepare · completed" }),
    ).toHaveAttribute("aria-expanded", "true");
  }}
/>

<Story
  name="Step read failure uses a retry toast and keeps the selected run"
  args={{ initialHash: selected, consoleMode: "success", logReadFailure: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Step prepare · completed" }),
    );
    const toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent("Demo step read failed");
    await expect(toast.closest('[data-ui-part="toast"]')).not.toBeNull();
    await userEvent.click(
      within(toast).getByRole("button", { name: "Retry step logs" }),
    );
    await expect(await canvas.findByLabelText("stdout log")).toHaveTextContent(
      "Pinned source ready",
    );
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "run=" + "51".repeat(16),
    );
  }}
/>

<Story
  name="Log download failure uses a retry toast"
  args={{
    initialHash: selected,
    consoleMode: "success",
    downloadFailure: true,
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Log settings" }),
    );
    await userEvent.click(
      await body.findByRole("menuitem", { name: "Download logs" }),
    );
    const toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent("Demo download failed");
    await expect(toast.closest('[data-ui-part="toast"]')).not.toBeNull();
    await userEvent.click(
      within(toast).getByRole("button", { name: "Retry download" }),
    );
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
  }}
/>

<Story
  name="Re-run all jobs is a right header action guarded by the latest attempt"
  args={{ initialHash: selected, consoleMode: "success" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const retry = await canvas.findByRole("button", {
      name: "Re-run all jobs",
    });
    await expect(retry).toHaveTextContent("Re-run all jobs");
    await expect(retry.closest(".workflow-run-actions")).not.toBeNull();
    await expect(retry.querySelector("svg")).not.toBeNull();
    await waitFor(() =>
      expect(getComputedStyle(retry).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run attempt" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("option", {
        name: /Run attempt #2/,
      }),
    );
    await userEvent.click(
      await canvas.findByRole("button", { name: "Re-run all jobs" }),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Last workflow command")).toHaveTextContent(
        JSON.stringify({
          type: "retry",
          run: "51".repeat(16),
          expected: "89".repeat(16),
        }),
      ),
    );
    await waitFor(() =>
      expect(canvas.getByLabelText("Workflow location")).not.toHaveTextContent(
        "at=",
      ),
    );
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run attempt" }),
    );
    await userEvent.click(
      await within(document.body).findByRole("option", {
        name: /Run attempt #1/,
      }),
    );
    await expect(
      canvas.queryByRole("button", { name: "Re-run all jobs" }),
    ).toBeNull();
  }}
/>
