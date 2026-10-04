<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    userEvent,
    within,
    waitFor,
    fireEvent,
  } from "storybook/test";
  import Fixture from "../../fixtures/TerminalFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Desktop terminal",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Repository sessions"
  play={async ({ canvasElement }) => {
    const ui = within(canvasElement);
    await userEvent.type(
      ui.getByRole("textbox", { name: "Retained draft" }),
      "keep draft",
    );
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    await expect(
      ui.getByRole("button", { name: "Hide terminal" }),
    ).toHaveAttribute("aria-expanded", "true");
    await expect(
      canvasElement.querySelector('[data-ui-part="terminal-toggle-icon"]'),
    ).toHaveAttribute("data-expanded", "true");
    await expect(
      ui.getByRole("textbox", { name: "Terminal output" }),
    ).toHaveTextContent("Ready");
    await userEvent.click(ui.getByRole("button", { name: "New terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(2));
    await userEvent.click(
      ui.getByRole("button", { name: "Hide terminal panel" }),
    );
    await expect(
      ui.queryByRole("region", { name: "Repository terminal" }),
    ).not.toBeInTheDocument();
    await expect(
      ui.getByRole("textbox", { name: "Retained draft" }),
    ).toHaveValue("keep draft");
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(2));
    await userEvent.click(ui.getByRole("button", { name: "Repository B" }));
    await waitFor(() => expect(ui.queryAllByRole("tab")).toHaveLength(0));
    await waitFor(() =>
      expect(ui.getByRole("button", { name: "Open terminal" })).toBeEnabled(),
    );
    await userEvent.click(ui.getByRole("button", { name: "Open terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    await userEvent.click(ui.getByRole("button", { name: "Repository A" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(2));
    await userEvent.click(
      ui.getByRole("button", { name: "Maximize terminal" }),
    );
    await expect(
      ui.getByRole("button", { name: "Restore terminal" }),
    ).toBeVisible();
    await userEvent.click(ui.getByRole("button", { name: "Restore terminal" }));
    await userEvent.click(ui.getByRole("button", { name: "Close terminal 2" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    const handle = ui.getByRole("separator", { name: "Resize terminal" });
    handle.focus();
    await userEvent.keyboard("{ArrowUp}");
    await expect(handle).toHaveFocus();
    await userEvent.keyboard("{ArrowDown>30/}");
    await waitFor(() =>
      expect(ui.getByRole("button", { name: "Show terminal" })).toHaveFocus(),
    );
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    await userEvent.keyboard("{Meta>}j{/Meta}");
    await waitFor(() =>
      expect(ui.getByRole("button", { name: "Show terminal" })).toHaveFocus(),
    );
  }}
/>
<Story
  name="Choose linked folder"
  args={{ folders: 2 }}
  play={async ({ canvasElement }) => {
    const ui = within(canvasElement);
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await expect(
      ui.getByRole("button", { name: "Open terminal" }),
    ).toBeDisabled();
    await userEvent.selectOptions(
      ui.getByRole("combobox", { name: "Terminal folder" }),
      "repo-a:1",
    );
    await userEvent.click(ui.getByRole("button", { name: "Open terminal" }));
    await waitFor(() =>
      expect(ui.getByRole("tab")).toHaveAttribute(
        "title",
        "/workspace/repo-a/2",
      ),
    );
  }}
/>
<Story
  name="No local folder"
  args={{ folders: 0 }}
  play={async ({ canvasElement }) => {
    const ui = within(canvasElement);
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await expect(
      ui.queryByRole("textbox", { name: "Terminal input" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      ui.getByRole("button", { name: "Link local folder" }),
    );
    await expect(ui.getByRole("status")).toHaveTextContent(
      "Folder settings opened",
    );
  }}
/>

<Story
  name="Input and selection"
  args={{ inputProbe: true }}
  play={async ({ canvasElement }) => {
    const ui = within(canvasElement);
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await waitFor(() =>
      expect(
        ui.getByRole("textbox", { name: "Terminal output" }),
      ).toHaveTextContent("Ready"),
    );
    const input = ui.getByRole("textbox", {
      name: "Terminal input",
    }) as HTMLTextAreaElement;
    fireEvent.compositionStart(input);
    input.value = "日";
    fireEvent.input(input, { isComposing: true });
    await expect(
      ui.getByLabelText("Received terminal input"),
    ).toHaveTextContent("[]");
    input.value = "日本語";
    fireEvent.compositionEnd(input);
    await waitFor(() =>
      expect(ui.getByLabelText("Received terminal input")).toHaveTextContent(
        '["日本語"]',
      ),
    );
    const clipboard = new DataTransfer();
    clipboard.setData("text/plain", "one\ntwo");
    input.dispatchEvent(
      new ClipboardEvent("paste", {
        bubbles: true,
        cancelable: true,
        clipboardData: clipboard,
      }),
    );
    await waitFor(() =>
      expect(ui.getByLabelText("Received terminal input")).toHaveTextContent(
        JSON.stringify(["日本語", "\u001b[200~one\ntwo\u001b[201~"]),
      ),
    );
    const row = canvasElement.querySelector("[data-terminal-row]")!;
    const range = document.createRange();
    range.selectNodeContents(row);
    window.getSelection()!.removeAllRanges();
    window.getSelection()!.addRange(range);
    const copied = new DataTransfer();
    const copy = new ClipboardEvent("copy", {
      bubbles: true,
      cancelable: true,
      clipboardData: copied,
    });
    row.dispatchEvent(copy);
    await expect(copy.defaultPrevented).toBe(true);
    await expect(copied.getData("text/plain")).toBe(row.textContent?.trim());
    window.getSelection()!.removeAllRanges();
  }}
/>
