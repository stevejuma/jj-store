<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/TagListAdoptionFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Components/Tag list adoption",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: {
      docs: {
        source: {
          code: 'import { TagList } from "@lapismd/design-core/shadcn/tag-list";\n// Map native labels and tones in the host; preserve their order.\n<TagList label="Revision tags" tags={revisionTags} />',
        },
      },
    },
  });
</script>

<Story
  name="Responsive collections and live updates"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const list = c.getByRole("list", {
      name: "Tags for Review the responsive tags",
    });
    const more = () =>
      list.querySelector<HTMLButtonElement>('[data-ui-part="overflow"]')!;
    await waitFor(() => expect(more()).not.toBeNull());
    const width = c.getByRole("spinbutton", { name: "Container width" });
    more().focus();
    await userEvent.keyboard("{Enter}");
    const popup = await page.findByRole("dialog", {
      name: /Tags for Review the responsive tags/,
    });
    await waitFor(() =>
      expect(within(popup).getByText("ready", { exact: true })).toBeVisible(),
    );
    expect(within(popup).getAllByRole("listitem")).toHaveLength(5);
    expect(c.getByLabelText("Threads opened")).toHaveTextContent("0");
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(more()).toHaveFocus());
    await userEvent.clear(width);
    await userEvent.type(width, "900");
    await waitFor(() =>
      expect(list.querySelector('[data-ui-part="overflow"]')).toBeNull(),
    );
    await userEvent.clear(width);
    await userEvent.type(width, "350");
    await waitFor(() => expect(more()).not.toBeNull());
    expect(list.getBoundingClientRect().width).toBeLessThanOrEqual(350);
    const row = list.querySelector<HTMLElement>('[data-ui-part="row"]')!;
    expect(row.scrollHeight).toBe(row.clientHeight);
    await userEvent.click(c.getByRole("button", { name: "Toggle live tag" }));
    await userEvent.click(more());
    const updated = await page.findByRole("dialog", {
      name: /Tags for Review the responsive tags/,
    });
    await waitFor(() =>
      expect(within(updated).getByText("new live tag")).toBeVisible(),
    );
    expect(within(updated).getAllByRole("listitem")).toHaveLength(6);
    await userEvent.keyboard("{Escape}");
    await userEvent.clear(width);
    await userEvent.type(width, "1200");
    await waitFor(() =>
      expect(list.querySelector('[data-ui-part="overflow"]')).toBeNull(),
    );
  }}
/>

<Story
  name="Commit overflow is independent of selection"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(canvasElement.ownerDocument.body);
    const trigger = c.getByRole("combobox", { name: "Choose tagged commit" });
    expect(trigger.querySelector("button")).toBeNull();
    expect(trigger).toHaveAccessibleDescription(/Working copy/);
    const list = c.getByRole("list", { name: "Commit tags" });
    await waitFor(() =>
      expect(list.querySelector('[data-ui-part="overflow"]')).not.toBeNull(),
    );
    const more = list.querySelector<HTMLButtonElement>(
      '[data-ui-part="overflow"]',
    )!;
    await userEvent.click(more);
    const popup = await page.findByRole("dialog", { name: "Commit tags (14)" });
    await waitFor(() =>
      expect(within(popup).getByText("review", { exact: true })).toBeVisible(),
    );
    expect(trigger).toHaveAttribute("aria-expanded", "false");
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(more).toHaveFocus());
    await userEvent.click(trigger);
    const options = await page.findAllByRole("option");
    const option = options[1]!;
    await userEvent.hover(option);
    const preview = page.getByRole("region", { name: "Commit tag preview" });
    await waitFor(() =>
      expect(preview).toHaveAttribute(
        "data-commit",
        option.getAttribute("data-value"),
      ),
    );
    await waitFor(() =>
      expect(preview.querySelector('[data-ui-part="overflow"]')).not.toBeNull(),
    );
    const previewMore = preview.querySelector<HTMLButtonElement>(
      '[data-ui-part="overflow"]',
    )!;
    await userEvent.click(previewMore);
    const nested = await page.findByRole("dialog", {
      name: "Preview commit tags (14)",
    });
    await waitFor(() =>
      expect(within(nested).getByText("ready", { exact: true })).toBeVisible(),
    );
    expect(trigger).toHaveAttribute("aria-expanded", "true");
    expect(option).not.toHaveAttribute("data-current", "true");
    await userEvent.keyboard("{Escape}{Escape}");
  }}
/>
