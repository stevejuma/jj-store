<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow choices",
    component: WorkflowsFixture,
    parameters: {
      jjStoreStory: { kind: "application" },
      layout: "fullscreen",
      visualDelta: { tags: ["visual-pending"] },
    },
  });
  async function open(canvasElement: HTMLElement) {
    const body = within(document.body);
    await userEvent.click(
      await within(canvasElement).findByRole("button", {
        name: "Create workflow",
      }),
    );
    return {
      body,
      dialog: within(
        await body.findByRole("dialog", { name: "Create workflow" }),
      ),
    };
  }
  async function trigger(
    dialog: ReturnType<typeof within>,
    body: ReturnType<typeof within>,
    name: string,
  ) {
    await userEvent.click(dialog.getByRole("button", { name: "When" }));
    await userEvent.click(
      body.getByRole("option", { name: new RegExp(`^${name}$`) }),
    );
  }
</script>

<Story
  name="Cold authors load profiles and agents and threads retain their actual owner"
  play={async ({ canvasElement }) => {
    const { body, dialog } = await open(canvasElement);
    await userEvent.click(
      dialog.getByRole("combobox", { name: "Source repository" }),
    );
    await userEvent.click(body.getByRole("option", { name: /Parser/ }));
    await trigger(dialog, body, "Thread message posted");
    await userEvent.click(dialog.getByRole("combobox", { name: /^Thread$/ }));
    const thread = await body.findByRole("option", {
      name: /general Automation home/,
    });
    await userEvent.click(thread);
    await expect(
      dialog.getByRole("combobox", { name: "Source repository" }),
    ).toHaveTextContent("Automation home");
    await userEvent.click(dialog.getByRole("combobox", { name: /^Authors$/ }));
    const human = await body.findByRole("option", { name: /Server author/ });
    await expect(
      human.querySelector(
        '[data-ui-component="community-member-avatar"] [data-content="emoji"]',
      ),
    ).not.toBeNull();
    const agent = await body.findByRole("option", {
      name: /Repository helper Agent message triggers are not supported yet/,
    });
    await expect(agent).toHaveAttribute("aria-disabled", "true");
    await expect(
      agent.querySelector('[data-ui-component="community-member-avatar"]'),
    ).not.toBeNull();
    await userEvent.click(human);
    await userEvent.keyboard("{Escape}");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    const yaml = dialog.getByRole("textbox", { name: "Workflow YAML" });
    await expect(yaml).toHaveTextContent("41".repeat(16));
    await expect(yaml).toHaveTextContent("33".repeat(16));
    await expect(yaml).toHaveTextContent("22".repeat(16));
    await expect(yaml).not.toHaveTextContent("helper");
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Revision match highlights revsets and source changes replace commit authors"
  play={async ({ canvasElement }) => {
    const { body, dialog } = await open(canvasElement);
    await trigger(dialog, body, "Code published");
    await userEvent.type(
      dialog.getByRole("searchbox", { name: "Revision match" }),
      'bookmarks("main") & all()',
    );
    await expect(
      dialog
        .getByRole("searchbox", { name: "Revision match" })
        .querySelector(".history-query-function"),
    ).not.toBeNull();
    await expect(
      dialog
        .getByRole("searchbox", { name: "Revision match" })
        .querySelector(".history-query-string"),
    ).not.toBeNull();
    await userEvent.click(dialog.getByRole("combobox", { name: /^Authors$/ }));
    await waitFor(async () => {
      await expect(
        await body.findByRole("option", { name: /Home author/ }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
    await userEvent.click(
      dialog.getByRole("combobox", { name: "Source repository" }),
    );
    await userEvent.click(body.getByRole("option", { name: /Parser/ }));
    await userEvent.click(dialog.getByRole("combobox", { name: /^Authors$/ }));
    await waitFor(async () => {
      await expect(
        await body.findByRole("option", { name: /Code author/ }),
      ).toBeVisible();
    });
    await expect(
      body.queryByRole("option", { name: /Home author/ }),
    ).toBeNull();
    await userEvent.keyboard("{Escape}");
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent('bookmarks("main") & all()');
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
  }}>{#snippet template()}<WorkflowsFixture />{/snippet}</Story
>

<Story
  name="Profile thread agent and code lookup failures use toasts and retain the draft"
  play={async ({ canvasElement }) => {
    const { body, dialog } = await open(canvasElement);
    await trigger(dialog, body, "Thread message posted");
    for (const title of [
      "Author profiles could not be loaded",
      "Agents could not be loaded",
      "Threads could not be loaded from Automation home",
    ])
      await waitFor(async () => {
        await expect(
          await body.findByText(title, { exact: true }),
        ).toBeVisible();
      });
    await expect(
      within(
        dialog.getByRole("complementary", { name: "Workflow inspector" }),
      ).queryByText("Profile lookup failed", { exact: true }),
    ).toBeNull();
    await expect(
      within(
        dialog.getByRole("complementary", { name: "Workflow inspector" }),
      ).queryByText("Thread lookup failed", { exact: true }),
    ).toBeNull();
    await trigger(dialog, body, "Code published");
    await waitFor(async () => {
      await expect(
        await body.findByText("Trigger choices could not be loaded", {
          exact: true,
        }),
      ).toBeVisible();
    });
    await userEvent.type(
      dialog.getByRole("searchbox", { name: "Revision match" }),
      "all()",
    );
    await expect(
      within(
        dialog.getByRole("complementary", { name: "Workflow inspector" }),
      ).queryByText("Source lookup failed", { exact: true }),
    ).toBeNull();
    await userEvent.click(dialog.getByRole("tab", { name: "YAML" }));
    await expect(
      dialog.getByRole("textbox", { name: "Workflow YAML" }),
    ).toHaveTextContent("all()");
    await userEvent.click(dialog.getByRole("button", { name: "Cancel" }));
    await waitFor(() => expect(body.queryByRole("dialog")).toBeNull());
  }}>{#snippet template()}<WorkflowsFixture choiceFailures />{/snippet}</Story
>
