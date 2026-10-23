<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within, waitFor } from "storybook/test";
  import StoreRepositoryPicker from "../../../web/src/lib/ui/store-application/StoreRepositoryPicker.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject:
          "web/src/lib/ui/store-application/StoreRepositoryPicker.svelte",
      },
    },
    title: "JJ Store/Application/Repository picker",
    component: StoreRepositoryPicker,
    tags: ["visual-pending", "component"],
    args: {
      label: "Repository",
      repositories: [
        { id: "41".repeat(16), name: "Parser", permission: "admin" },
        { id: "42".repeat(16), name: "Parser", permission: "read" },
      ],
      onChange: fn(),
    },
  });
</script>

<Story
  name="Search exact identity and select with keyboard"
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const trigger = canvas.getByRole("combobox", { name: "Repository" });
    await userEvent.click(trigger);
    const search = body.getByRole("combobox", { name: "Select repository" });
    const close = body.getByRole("button", {
      name: "Close repository selector",
    });
    for (const title of body.getAllByText("Select repository", { exact: true }))
      await expect(title.getBoundingClientRect().height).toBeLessThanOrEqual(1);
    await expect(
      Math.abs(
        search.getBoundingClientRect().top +
          search.getBoundingClientRect().height / 2 -
          close.getBoundingClientRect().top -
          close.getBoundingClientRect().height / 2,
      ),
    ).toBeLessThan(4);
    await expect(close.getBoundingClientRect().left).toBeGreaterThan(
      search.getBoundingClientRect().right,
    );
    await userEvent.type(search, "42".repeat(16));
    await waitFor(() => expect(body.getAllByRole("option")).toHaveLength(1));
    await userEvent.keyboard("{ArrowDown}{Enter}");
    await expect(args.onChange).toHaveBeenCalledWith("42".repeat(16));
    await expect(trigger).toHaveTextContent("Parser");
    await expect(trigger).toHaveAttribute("title", "42".repeat(16));
    await expect(trigger).toHaveFocus();
    await userEvent.click(trigger);
    await userEvent.click(
      body.getByRole("button", { name: "Close repository selector" }),
    );
    await expect(trigger).toHaveFocus();
  }}
/>
<Story
  name="Unavailable selection and empty search"
  args={{
    value: "43".repeat(16),
    allowEmpty: true,
    emptyLabel: "Project only",
  }}
  play={async ({ canvasElement, args }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const trigger = canvas.getByRole("combobox", { name: "Repository" });
    await expect(trigger).toHaveAttribute("title", "43".repeat(16));
    await userEvent.click(trigger);
    const search = body.getByPlaceholderText("Search repositories");
    await userEvent.type(search, "missing repository");
    await expect(body.getByText("No matching repositories.")).toBeVisible();
    await userEvent.clear(search);
    await userEvent.click(body.getByRole("option", { name: "Project only" }));
    await expect(args.onChange).toHaveBeenCalledWith("");
    await expect(trigger).toHaveTextContent("Project only");
  }}
/>
<Story
  name="Disabled picker"
  args={{ disabled: true }}
  play={async ({ canvasElement }) => {
    await expect(
      within(canvasElement).getByRole("combobox", { name: "Repository" }),
    ).toBeDisabled();
  }}
/>

<Story
  name="Multiple selections retain search state and enforce the eight repository bound"
  args={{
    multiple: true,
    repositories: Array.from({ length: 9 }, (_, i) => ({
      id: (i + 1).toString(16).padStart(32, "0"),
      name: `Codebase ${i + 1}`,
    })),
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const trigger = canvas.getByRole("combobox", { name: "Repository" });
    await userEvent.click(trigger);
    for (let i = 1; i <= 8; i++)
      await userEvent.click(
        body.getByRole("option", { name: new RegExp(`^Codebase ${i} `) }),
      );
    await expect(trigger).toHaveTextContent("8 repositories selected");
    await expect(
      body.getByRole("option", { name: /^Codebase 9 / }),
    ).toHaveAttribute("aria-disabled", "true");
    const search = body.getByPlaceholderText("Search repositories");
    await userEvent.type(search, "Codebase 1");
    await expect(body.getAllByRole("option")).toHaveLength(1);
    await userEvent.click(body.getByRole("option", { name: /^Codebase 1 / }));
    await expect(trigger).toHaveTextContent("7 repositories selected");
    await userEvent.clear(search);
    await userEvent.click(body.getByRole("option", { name: /^Codebase 9 / }));
    await userEvent.click(
      body.getByRole("button", { name: "Close repository selector" }),
    );
    await userEvent.click(trigger);
    await expect(
      body.getByRole("option", { name: /^Codebase 1 / }),
    ).toHaveAttribute("data-repository-selected", "false");
    await expect(
      body.getByRole("option", { name: /^Codebase 9 / }),
    ).toHaveAttribute("data-repository-selected", "true");
    await userEvent.click(
      body.getByRole("button", { name: "Close repository selector" }),
    );
  }}
/>
