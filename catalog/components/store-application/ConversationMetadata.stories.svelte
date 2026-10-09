<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ConversationMetadataFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Agents/Conversation metadata",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Interrupted generation can stop waiting without replay"
  args={{ interrupted: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: "Metadata" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Stop waiting for generation" }),
    );
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Refresh metadata" })).toBeEnabled(),
    );
    const commands = JSON.parse(
      c.getByLabelText("Metadata commands").textContent!,
    );
    expect(commands).toHaveLength(1);
    expect(commands[0].action).toEqual({
      action: "cancel",
      job: "bb".repeat(16),
    });
  }}
/>

<Story
  name="Details retain drafts through remount and refresh preserves custom titles"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: "Overview" }));
    const title = await c.findByRole("textbox", {
      name: "Rename conversation",
    });
    await userEvent.type(title, "My custom title");
    await userEvent.click(
      c.getByRole("button", { name: "Toggle details mount" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle details mount" }),
    );
    await userEvent.click(await c.findByRole("button", { name: "Overview" }));
    await expect(
      await c.findByRole("textbox", { name: "Rename conversation" }),
    ).toHaveValue("My custom title");
    await userEvent.click(c.getByRole("button", { name: /^Rename$/ }));
    await waitFor(() => expect(c.getByText("My custom title")).toBeVisible());
    await userEvent.click(c.getByRole("button", { name: "Metadata" }));
    await userEvent.click(c.getByRole("button", { name: "Refresh metadata" }));
    await c.findByText("Fixture generated summary.");
    await expect(c.getByText("My custom title")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Regenerate title" }));
    await waitFor(() =>
      expect(c.queryByText("My custom title")).not.toBeInTheDocument(),
    );
    const commands = JSON.parse(
      c.getByLabelText("Metadata commands").textContent!,
    );
    await expect(
      commands.map((v: { action: { action: string } }) => v.action.action),
    ).toEqual(["rename", "refresh", "regenerate_title"]);
  }}
/>
<Story
  name="Lost responses retain the exact request across remount"
  args={{ lost: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: "Overview" }));
    await userEvent.type(
      await c.findByRole("textbox", { name: "Rename conversation" }),
      "Retained title",
    );
    await userEvent.click(c.getByRole("button", { name: /^Rename$/ }));
    await within(document.body).findByText(/Simulated lost response/);
    await userEvent.click(
      c.getByRole("button", { name: "Toggle details mount" }),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle details mount" }),
    );
    await userEvent.click(await c.findByRole("button", { name: "Overview" }));
    await userEvent.click(c.getByRole("button", { name: "Metadata" }));
    await userEvent.click(
      await c.findByRole("button", { name: "Check retained request" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("textbox", { name: "Rename conversation" }),
      ).toHaveValue(""),
    );
    const commands = JSON.parse(
      c.getByLabelText("Metadata commands").textContent!,
    );
    await expect(commands).toHaveLength(2);
    await expect(commands[0]).toEqual(commands[1]);
  }}
/>
<Story
  name="Dedicated settings omit coding harness and execution controls"
  args={{ settingsMode: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByLabelText("Metadata provider", { exact: true });
    await expect(
      c.queryByRole("combobox", { name: "Harness" }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByRole("combobox", { name: "Execution mode" }),
    ).not.toBeInTheDocument();
    await c.findByRole("button", { name: "Save metadata model" });
    await userEvent.click(
      c.getByRole("button", { name: "Save metadata model" }),
    );
    await expect(
      c.getByLabelText("Saved metadata configuration"),
    ).toHaveTextContent("fixture-model");
    await userEvent.click(
      c.getByRole("button", { name: "Disable generation" }),
    );
    await waitFor(() =>
      expect(
        c.getByLabelText("Saved metadata configuration"),
      ).toHaveTextContent("null"),
    );
  }}
/>

<Story
  name="Settings sections start closed and collapse retains the rename draft"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const overview = await c.findByRole("button", {
      name: "Overview",
    });
    const metadata = c.getByRole("button", { name: "Metadata" });
    expect(overview).toHaveAttribute("aria-expanded", "false");
    expect(metadata).toHaveAttribute("aria-expanded", "false");
    expect(
      c.queryByRole("textbox", { name: "Rename conversation" }),
    ).toBeNull();
    expect(c.queryByRole("button", { name: "Refresh metadata" })).toBeNull();
    await userEvent.click(overview);
    const input = c.getByRole("textbox", { name: "Rename conversation" });
    await userEvent.type(input, "Retained overview draft");
    expect(input.closest(".community-settings__option-group")).not.toBeNull();
    expect(
      c.getByText("Repository ID").closest(".store-settings__summary-rows"),
    ).not.toBeNull();
    const content = overview
      .closest(".store-settings-disclosure")!
      .querySelector(".store-settings-disclosure__content")!;
    expect(getComputedStyle(content).padding).toBe("0px");
    expect(content.querySelector(".store-settings__rows")).toBeNull();
    expect(metadata.closest(".community-settings__option-group")).toBe(
      overview.closest(".community-settings__option-group"),
    );
    await userEvent.click(overview);
    expect(overview).toHaveAttribute("aria-expanded", "false");
    expect(
      c.queryByRole("textbox", { name: "Rename conversation" }),
    ).toBeNull();
    await userEvent.click(metadata);
    expect(c.getByRole("button", { name: "Refresh metadata" })).toBeVisible();
    overview.focus();
    await userEvent.keyboard("{Enter}");
    expect(
      await c.findByRole("textbox", { name: "Rename conversation" }),
    ).toHaveValue("Retained overview draft");
  }}
/>
