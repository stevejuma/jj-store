<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/AgentMentionPreferenceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Settings/Agent mention recovery",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Lost preference save retries exactly and refresh updates both composers"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      first = within(c.getByRole("region", { name: "First conversation" })),
      second = within(c.getByRole("region", { name: "Second conversation" }));
    await userEvent.click(
      first.getByRole("button", { name: "Mention someone" }),
    );
    const toggle = await first.findByRole("switch", {
      name: "Automatically mention agents",
    });
    await waitFor(() => expect(toggle).toBeEnabled());
    await userEvent.click(toggle);
    await first.findByText("Preference response lost");
    await expect(toggle).not.toBeChecked();
    await expect(
      first.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Draft stays here");
    await userEvent.click(first.getByRole("button", { name: "Retry" }));
    await waitFor(() => expect(toggle).toBeChecked());
    const requests = JSON.parse(
      c.getByLabelText("Preference requests").textContent ?? "[]",
    );
    await expect(requests).toHaveLength(2);
    await expect(requests[0]).toEqual(requests[1]);
    await userEvent.click(
      second.getByRole("button", { name: "Mention someone" }),
    );
    const other = await second.findByRole("switch", {
      name: "Automatically mention agents",
    });
    await expect(other).toBeChecked();
    await userEvent.click(
      c.getByRole("button", { name: "External preference off" }),
    );
    await waitFor(() => expect(toggle).not.toBeChecked());
    await expect(other).not.toBeChecked();
    await expect(
      first.getByRole("textbox", { name: "Markdown editor" }),
    ).toHaveTextContent("Draft stays here");
    await expect(
      JSON.parse(c.getByLabelText("Preference requests").textContent ?? "[]"),
    ).toEqual(requests);
  }}
/>
