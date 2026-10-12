<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import NumericSettingFixture from "../../fixtures/NumericSettingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Numeric setting row",
    component: NumericSettingFixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Unit selection preserves canonical draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await expect(
      c.queryByRole("button", { name: "Reset Polling interval to default" }),
    ).toBeNull();
    const unit = c.getByRole("button", { name: "Polling interval unit" });
    await userEvent.click(unit);
    await userEvent.click(page.getByRole("option", { name: "sec" }));
    await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
    await expect(c.getByRole("spinbutton")).toHaveValue(1);
    await expect(c.getByTestId("changes")).toHaveTextContent("0");
    const split = c.getByRole("group", { name: "Polling interval" });
    const addon = split.querySelector('[data-slot="input-group-addon"]')!;
    await expect(addon.getBoundingClientRect().right).toBeLessThanOrEqual(
      split.getBoundingClientRect().right - 1,
    );
    await expect(getComputedStyle(split).borderRightWidth).toBe("1px");
    await waitFor(() =>
      expect(getComputedStyle(c.getByRole("spinbutton")).pointerEvents).toBe(
        "auto",
      ),
    );
    await userEvent.clear(c.getByRole("spinbutton"));
    await userEvent.type(c.getByRole("spinbutton"), "2.5");
    await expect(c.getByTestId("canonical")).toHaveTextContent("2500");
    await userEvent.click(c.getByRole("button", { name: "Save" }));
    await expect(c.getByTestId("saved")).toHaveTextContent("2500");
    await userEvent.click(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    );
    await expect(c.getByRole("spinbutton")).toHaveValue(1);
    await expect(c.getByTestId("canonical")).toHaveTextContent("1000");
    await expect(
      c.queryByRole("button", { name: "Reset Polling interval to default" }),
    ).toBeNull();
    await expect(c.getByRole("spinbutton")).toHaveFocus();
    await userEvent.clear(c.getByRole("spinbutton"));
    const reset = c.getByRole("button", {
      name: "Reset Polling interval to default",
    });
    await expect(reset.textContent?.trim()).toBe("");
    await expect(
      c.getByRole("group", { name: "Polling interval" }),
    ).toContainElement(reset);
    await expect(reset).toBeEnabled();
    await userEvent.click(reset);
    await expect(c.getByRole("spinbutton")).toHaveValue(1);
  }}
/>

<Story
  name="Byte units reject rounding and invalid saves"
  args={{ bytes: true, value: "1048576", defaultValue: 1048576 }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(
      c.getByRole("button", { name: "Maximum text file size unit" }),
    );
    await userEvent.click(page.getByRole("option", { name: "KiB" }));
    await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
    const input = c.getByRole("spinbutton");
    await expect(input).toHaveValue(1024);
    await waitFor(() =>
      expect(getComputedStyle(input).pointerEvents).toBe("auto"),
    );
    await userEvent.clear(input);
    await userEvent.type(input, "0.1");
    await expect(input).toHaveAttribute("aria-invalid", "true");
    await expect(c.getByRole("alert")).toHaveTextContent(
      "whole number of bytes",
    );
    await userEvent.click(c.getByRole("button", { name: "Save" }));
    await expect(c.getByTestId("saved").textContent).toBe("");
    await waitFor(() =>
      expect(getComputedStyle(input).pointerEvents).toBe("auto"),
    );
    await userEvent.clear(input);
    await userEvent.type(input, "1.5");
    await expect(c.queryByRole("alert")).toBeNull();
    await expect(c.getByTestId("canonical")).toHaveTextContent("1536");
    await userEvent.click(c.getByRole("button", { name: "Save" }));
    await expect(c.getByTestId("saved")).toHaveTextContent("1536");
  }}
/>

<Story
  name="Keyboard and optional slider"
  args={{ slider: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    await userEvent.click(c.getByRole("spinbutton"));
    await userEvent.tab();
    await expect(
      c.getByRole("button", { name: "Polling interval unit" }),
    ).toHaveFocus();
    await userEvent.keyboard("{Enter}");
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
    const slider = c.getByRole("slider", { name: "Polling interval slider" });
    slider.focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(c.getByTestId("canonical")).toHaveTextContent("1001");
  }}
/>

<Story
  name="Disabled"
  args={{ disabled: true, value: "2000", slider: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByRole("spinbutton")).toBeDisabled();
    await expect(
      c.getByRole("button", { name: "Polling interval unit" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Read only"
  args={{ readonly: true, value: "2000" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByRole("spinbutton")).toHaveAttribute("readonly");
    await expect(
      c.getByRole("button", { name: "Polling interval unit" }),
    ).toBeDisabled();
    await expect(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Compact narrow and changed"
  args={{ narrow: true, density: "compact", value: "2000" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      row = c.getByRole("spinbutton").closest(".store-numeric-setting")!;
    await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth);
    await expect(c.queryByText("Changed from default")).toBeNull();
    await expect(c.queryByText("Using default value")).toBeNull();
    await expect(
      c.getByRole("button", { name: "Reset Polling interval to default" }),
    ).toBeVisible();
  }}
/>
<Story
  name="Dark"
  globals={{ colorMode: "dark" }}
  args={{ bytes: true, value: "524288", defaultValue: 1048576 }}
/>

<Story
  name="One group focus ring and a neutral border"
  args={{ value: "2000" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      group = c.getByRole("group", { name: "Polling interval" });
    const border = getComputedStyle(group).borderRightColor;
    await userEvent.click(c.getByRole("spinbutton"));
    await expect(getComputedStyle(group).borderRightColor).toBe(border);
    await waitFor(() =>
      expect(getComputedStyle(group).boxShadow).toContain("0px 0px 0px 3px"),
    );
    await expect(getComputedStyle(group).outlineStyle).toBe("none");
    await waitFor(() =>
      expect(getComputedStyle(c.getByRole("spinbutton")).boxShadow).toBe(
        "none",
      ),
    );
    await userEvent.tab();
    const unit = c.getByRole("button", { name: "Polling interval unit" });
    await expect(unit).toHaveFocus();
    await waitFor(() => expect(getComputedStyle(unit).boxShadow).toBe("none"));
    await expect(getComputedStyle(group).borderRightColor).toBe(border);
    await userEvent.tab();
    const reset = c.getByRole("button", {
      name: "Reset Polling interval to default",
    });
    await expect(reset).toHaveFocus();
    await waitFor(() => expect(getComputedStyle(reset).boxShadow).toBe("none"));
    await waitFor(() =>
      expect(getComputedStyle(group).boxShadow).toContain("0px 0px 0px 3px"),
    );
  }}
/>

<Story
  name="Unit and reset segments have hover feedback"
  args={{ value: "2000" }}
  play={async ({ canvasElement }) => {
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const c = within(canvasElement);
    await pointer.hover(c.getByRole("button", { name: "Save" }));
    for (const name of [
      "Polling interval unit",
      "Reset Polling interval to default",
    ]) {
      const control = c.getByRole("button", { name });
      const background = getComputedStyle(control).backgroundColor;
      await pointer.hover(control);
      await waitFor(() =>
        expect(getComputedStyle(control).backgroundColor).not.toBe(background),
      );
      await pointer.hover(c.getByRole("button", { name: "Save" }));
      await waitFor(() =>
        expect(getComputedStyle(control).backgroundColor).toBe(background),
      );
    }
  }}
/>

<Story
  name="Coarse slider preserves exact typed drafts"
  args={{ bytes: true, slider: true, sliderStep: 65536, value: "1536" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(c.getByTestId("canonical")).toHaveTextContent("1536");
    await expect(c.getByTestId("changes")).toHaveTextContent("0");
    const slider = c.getByRole("slider", {
      name: "Maximum text file size slider",
    });
    await expect(slider).toHaveAttribute("aria-valuenow", "1536");
    const entry = c.getByRole("spinbutton");
    await userEvent.clear(entry);
    await userEvent.type(entry, "1793");
    await expect(c.getByTestId("canonical")).toHaveTextContent("1793");
    await expect(slider).toHaveAttribute("aria-valuenow", "1793");
    slider.focus();
    await userEvent.keyboard("{End}");
    await expect(c.getByTestId("canonical")).toHaveTextContent("1048576");
  }}
/>

<Story
  name="Slider labels follow time units without changing the draft"
  args={{ slider: true, min: 1 }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const range = canvasElement.querySelector(".store-numeric-setting__range")!;
    const slider = c.getByRole("slider", { name: "Polling interval slider" });
    await expect(range).toHaveTextContent("1 ms");
    await expect(range).toHaveTextContent("60,000 ms");
    for (const [unit, minimum, maximum] of [
      ["sec", "0 sec", "60 sec"],
      ["min", "0 min", "1 min"],
      ["ms", "1 ms", "60,000 ms"],
    ]) {
      await waitFor(() =>
        expect(
          getComputedStyle(
            c.getByRole("button", { name: "Polling interval unit" }),
          ).pointerEvents,
        ).toBe("auto"),
      );
      await userEvent.click(
        c.getByRole("button", { name: "Polling interval unit" }),
      );
      await userEvent.click(page.getByRole("option", { name: unit }));
      await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
      await expect(range.children[0]).toHaveTextContent(minimum);
      await expect(range.children[1]).toHaveTextContent(maximum);
      await expect(slider).toHaveAttribute("aria-valuemin", "1");
      await expect(slider).toHaveAttribute("aria-valuemax", "60000");
      await expect(slider).toHaveAttribute("aria-valuenow", "1000");
      await expect(c.getByTestId("canonical")).toHaveTextContent("1000");
      await expect(c.getByTestId("changes")).toHaveTextContent("0");
    }
  }}
/>

<Story
  name="Narrow byte slider labels follow units without overflow"
  args={{
    bytes: true,
    slider: true,
    sliderStep: 65536,
    min: 1,
    narrow: true,
    value: "1536",
  }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const range = canvasElement.querySelector(".store-numeric-setting__range")!;
    for (const [unit, minimum, maximum] of [
      ["KiB", "0 KiB", "1,024 KiB"],
      ["MiB", "0 MiB", "1 MiB"],
      ["GiB", "0 GiB", "0.00098 GiB"],
    ]) {
      await waitFor(() =>
        expect(
          getComputedStyle(
            c.getByRole("button", { name: "Maximum text file size unit" }),
          ).pointerEvents,
        ).toBe("auto"),
      );
      await userEvent.click(
        c.getByRole("button", { name: "Maximum text file size unit" }),
      );
      await userEvent.click(page.getByRole("option", { name: unit }));
      await waitFor(() => expect(page.queryByRole("listbox")).toBeNull());
      await expect(range.children[0]).toHaveTextContent(minimum);
      await expect(range.children[1]).toHaveTextContent(maximum);
      await expect(range.scrollWidth).toBeLessThanOrEqual(range.clientWidth);
      await expect(c.getByTestId("canonical")).toHaveTextContent("1536");
      await expect(c.getByTestId("changes")).toHaveTextContent("0");
    }
  }}
/>
