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
  }}>{#snippet template()}<HistoryQueryFixture operations />{/snippet}</Story
>
