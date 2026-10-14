<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/WorkflowRunFixture.svelte";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  import {
    runOperation,
    previousOperation,
    runCommit,
  } from "../../fixtures/workflow-run-source.js";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow run",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function choose(label: string, option: string) {
    const body = within(document.body);
    await userEvent.click(body.getByRole("combobox", { name: label }));
    await userEvent.click(
      await body.findByRole("option", { name: new RegExp(`^${option}`) }),
    );
  }
  async function failurePlay({ args }: { args: { failure?: string } }) {
    const body = within(document.body);
    const dialog = await body.findByRole("dialog", {
      name: "Run Build parser",
    });
    if (args.failure === "commits" || args.failure === "bookmarks")
      await choose("Operation", "Publish source");
    const toast = await body.findByRole("alert");
    await expect(toast).toHaveTextContent(/lookup failed/);
    await expect(toast.closest('[data-ui-part="toast"]')).not.toBeNull();
    await expect(
      within(
        within(dialog).getByRole("region", { name: "Source revision" }),
      ).queryByRole("alert"),
    ).toBeNull();
    await userEvent.click(within(toast).getByRole("button", { name: /Retry/ }));
    await waitFor(() => expect(body.queryByRole("alert")).toBeNull());
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Cancel" }),
    );
  }
</script>

<Story
  name="Settings source pickers pin selection and clear dependent values"
  play={async () => {
    const body = within(document.body),
      dialog = within(
        await body.findByRole("dialog", { name: "Run Build parser" }),
      );
    const card = dialog.getByRole("region", { name: "Source revision" });
    await expect(card.querySelectorAll(".store-settings-icon")).toHaveLength(4);
    await expect(
      dialog.queryByRole("textbox", { name: "Operation" }),
    ).toBeNull();
    await expect(
      dialog.getByRole("button", { name: "Run workflow" }),
    ).toBeDisabled();
    await choose("Operation", "Publish source");
    await choose("Bookmark (optional)", "main");
    await expect(
      dialog.getByRole("combobox", { name: "Commit" }),
    ).toHaveAttribute("title", runCommit);
    await choose("Operation", "Previous source");
    await expect(
      dialog.getByRole("combobox", { name: "Commit" }),
    ).toHaveTextContent("Choose commit");
    await expect(
      dialog.getByRole("combobox", { name: "Bookmark (optional)" }),
    ).toHaveTextContent("No bookmark");
    await choose("Operation", "Publish source");
    await userEvent.click(
      dialog.getByRole("combobox", { name: "Bookmark (optional)" }),
    );
    await expect(
      await body.findByRole("option", { name: /^ambiguous/ }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.keyboard("{Escape}");
    await choose("Commit", "Fixture commit");
    await expect(
      dialog.getByRole("button", { name: "Run workflow" }),
    ).toBeEnabled();
    await userEvent.click(dialog.getByRole("button", { name: "Run workflow" }));
    const request = JSON.parse(
      body.getByLabelText("Recorded workflow request").textContent!,
    );
    await expect(request.command.source).toEqual({
      repository: "42".repeat(16),
      operation: runOperation,
      commit: runCommit,
      bookmark: null,
    });
  }}
/>
<Story
  name="Unknown run keeps exact request and readonly source"
  args={{ unknown: true }}
  play={async () => {
    const body = within(document.body),
      dialog = within(
        await body.findByRole("dialog", { name: "Run Build parser" }),
      );
    await choose("Operation", "Publish source");
    await choose("Bookmark (optional)", "main");
    await userEvent.click(dialog.getByRole("button", { name: "Run workflow" }));
    const request = body.getByLabelText(
      "Recorded workflow request",
    ).textContent;
    await expect(await body.findByRole("alert")).toHaveTextContent(
      "outcome is still unknown",
    );
    await expect(
      dialog.getByRole("combobox", { name: "Operation" }),
    ).toBeDisabled();
    await expect(
      dialog.getByRole("combobox", { name: "Commit" }),
    ).toBeDisabled();
    await userEvent.click(
      body.getByRole("button", { name: "Check original request" }),
    );
    await expect(
      body.getByLabelText("Recorded workflow request"),
    ).toHaveTextContent(request!);
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
  }}
/>
<Story
  name="Operation failure uses retry toast"
  args={{ failure: "operations" }}
  play={failurePlay}
/>
<Story
  name="Commit failure uses retry toast"
  args={{ failure: "commits" }}
  play={failurePlay}
/>
<Story
  name="Bookmark failure uses retry toast"
  args={{ failure: "bookmarks" }}
  play={failurePlay}
/>
<Story
  name="Source acquisition failure uses retry toast"
  args={{ failure: "connection" }}
  play={failurePlay}
/>

<Story
  name="Overview menu icons and width keep action labels on one line"
  play={async ({ canvasElement }) => {
    const body = within(document.body),
      canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Actions for Build parser" }),
    );
    const menu = body.getByRole("menu"),
      items = within(menu).getAllByRole("menuitem");
    await expect(items).toHaveLength(5);
    await expect(menu.getBoundingClientRect().width).toBeGreaterThanOrEqual(
      15 * parseFloat(getComputedStyle(document.documentElement).fontSize),
    );
    for (const item of items) {
      await expect(
        item.querySelector('svg[aria-hidden="true"]'),
      ).not.toBeNull();
      const walker = document.createTreeWalker(item, NodeFilter.SHOW_TEXT);
      let text = walker.nextNode();
      while (text && !text.textContent?.trim()) text = walker.nextNode();
      const range = document.createRange();
      range.selectNodeContents(text!);
      await expect(range.getClientRects()).toHaveLength(1);
    }
    await userEvent.keyboard("{Escape}");
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>
