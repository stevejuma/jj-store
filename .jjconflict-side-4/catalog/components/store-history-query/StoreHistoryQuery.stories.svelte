<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreHistoryQuery from "../../../web/src/lib/ui/store-history-query/StoreHistoryQuery.svelte";
  import HistoryQueryFixture from "../../fixtures/HistoryQueryFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/History query",
    component: StoreHistoryQuery,
    tags: ["visual-pending"],
    parameters: {
      docs: {
        source: {
          language: "tsx",
          type: "code",
          code: "<StoreHistoryQuery {controller} />",
        },
      },
    },
  });
</script>

<Story
  name="Commit revsets and keyboard completion"
  play={async ({ canvas, canvasElement }) => {
    await expect(
      canvas.getByRole("button", { name: "Run history query" }),
    ).toHaveTextContent("");
    const input = canvas.getByRole("searchbox", { name: "Filter JJ history" });
    await userEvent.click(input);
    await userEvent.type(input, "mut");
    const page = within(canvasElement.ownerDocument.body);
    await waitFor(() =>
      expect(page.getByRole("option", { name: /mutable/ })).toBeVisible(),
    );
    await userEvent.keyboard("{Enter}");
    await waitFor(() => expect(input).toHaveTextContent("mutable()"));
    await expect(
      canvasElement.querySelector(".history-query-alias"),
    ).not.toBeNull();
    await userEvent.keyboard("{Control>}{Enter}{/Control}");
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Applied history query" }),
      ).toHaveTextContent("mutable()"),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Clear search" }));
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Applied history query" }),
      ).toHaveTextContent("All history"),
    );
  }}>{#snippet template()}<HistoryQueryFixture />{/snippet}</Story
>
<Story
  name="Operation fields and retained invalid draft"
  play={async ({ canvas }) => {
    const input = canvas.getByRole("searchbox", {
      name: "Filter operation history",
    });
    await userEvent.click(input);
    await userEvent.type(input, 'user:steve description:"rebase branch"');
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      canvas.getByRole("button", { name: "Run history query" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Applied history query" }),
      ).toHaveTextContent('description:"rebase branch"'),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Clear search" }));
    await userEvent.type(input, 'description:"');
    await userEvent.click(
      canvas.getByRole("button", { name: "Run history query" }),
    );
    await waitFor(() =>
      expect(canvas.getByRole("alert")).toHaveTextContent(
        "Showing previous results",
      ),
    );
    await expect(input).toHaveTextContent('description:"');
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh current head" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Displayed history operation" }),
      ).toHaveTextContent("44".repeat(64)),
    );
    await expect(input).toHaveTextContent('description:"');
  }}>{#snippet template()}<HistoryQueryFixture operations />{/snippet}</Story
>

<Story
  name="Head refresh preserves the draft and accepted query"
  play={async ({ canvas, canvasElement }) => {
    const input = canvas.getByRole("searchbox", { name: "Filter JJ history" });
    const applied = canvas.getByRole("status", {
      name: "Applied history query",
    });
    await userEvent.click(input);
    await userEvent.type(input, "mut");
    await waitFor(() =>
      expect(
        canvasElement.ownerDocument.body.querySelector("[role='option']"),
      ).not.toBeNull(),
    );
    await userEvent.keyboard("{Enter}");
    await waitFor(() => expect(input).toHaveTextContent("mutable()"));
    await userEvent.click(
      canvas.getByRole("button", { name: "Run history query" }),
    );
    await waitFor(() => expect(applied).toHaveTextContent("mutable()"));
    await userEvent.click(input);
    await userEvent.keyboard("{Control>}a{/Control}");
    await userEvent.keyboard("{Backspace}");
    await userEvent.type(input, "bad(");
    await userEvent.click(
      canvas.getByRole("button", { name: "Run history query" }),
    );
    await waitFor(() =>
      expect(canvas.getByRole("alert")).toHaveTextContent(
        "Showing previous results",
      ),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh current head" }),
    );
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Displayed history operation" }),
      ).toHaveTextContent("44".repeat(64)),
    );
    await expect(input).toHaveTextContent("bad(");
    await expect(applied).toHaveTextContent("mutable()");
    await expect(canvas.getByRole("alert")).toHaveTextContent(
      "Showing previous results",
    );
  }}>{#snippet template()}<HistoryQueryFixture />{/snippet}</Story
>

<Story
  name="Multiline query and session email override"
  play={async ({ canvas, canvasElement }) => {
    await userEvent.click(
      canvas.getByRole("button", { name: "Query context and syntax" }),
    );
    const picker = canvas.getByRole("combobox", { name: "Workspace for @" });
    await expect(
      picker.closest(".community-settings__option-group"),
    ).not.toBeNull();
    await userEvent.click(picker);
    const workspaceMenu = within(canvasElement.ownerDocument.body);
    const workspace = workspaceMenu.getByRole("option", { name: /default/ });
    await expect(workspace.querySelector("svg")).not.toBeNull();
    await userEvent.type(
      workspaceMenu.getByRole("combobox", { name: "Select workspace" }),
      "default",
    );
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(picker).toHaveTextContent("default");
    const email = canvas.getByRole("textbox", {
      name: "Author email for mine()",
    });
    await waitFor(() => {
      for (const notice of canvasElement.querySelectorAll(
        '.store-history-query__context [data-slot="alert"]',
      )) {
        const icon = notice.querySelector("svg")!.getBoundingClientRect();
        const row = notice.getBoundingClientRect();
        expect(
          Math.abs(icon.y + icon.height / 2 - row.y - row.height / 2),
        ).toBeLessThan(1);
      }
    });
    await waitFor(() => expect(email).toHaveValue("fixture@example.invalid"));
    await userEvent.clear(email);
    await userEvent.type(email, "manual@example.invalid");
    const input = canvas.getByRole("searchbox", { name: "Filter JJ history" });
    await userEvent.click(input);
    await userEvent.type(input, "mutable() &");
    await userEvent.keyboard("{Escape}{Enter}");
    await userEvent.keyboard('description("fix")');
    await userEvent.keyboard("{Escape}{Control>}{Enter}{/Control}");
    await waitFor(() =>
      expect(
        canvas.getByRole("status", { name: "Applied history query" }),
      ).toHaveTextContent('mutable() & description("fix")'),
    );
    await expect(canvasElement.querySelectorAll(".cm-line").length).toBe(2);
    const alias = canvasElement.querySelector(".history-query-alias")!;
    await expect(
      canvasElement.ownerDocument.defaultView!.getComputedStyle(alias).color,
    ).not.toBe(
      canvasElement.ownerDocument.defaultView!.getComputedStyle(input).color,
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh current head" }),
    );
    await expect(email).toHaveValue("manual@example.invalid");
    await userEvent.clear(email);
    await userEvent.click(
      canvas.getByRole("button", { name: "Refresh current head" }),
    );
    await expect(email).toHaveValue("");
  }}
>
  {#snippet template()}<HistoryQueryFixture />{/snippet}
</Story>
