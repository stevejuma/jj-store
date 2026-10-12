<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreServerSwitcher from "../../../web/src/lib/ui/store-connections/StoreServerSwitcher.svelte";
  import Fixture from "../../fixtures/ServerConnectionsFixture.svelte";
  const { Story } = defineMeta({
    title: "Store/Server connections",
    component: StoreServerSwitcher,
    tags: ["autodocs", "visual-pending"],
    parameters: {
      docs: { source: { code: "<StoreServerSwitcher compact />" } },
    },
  });
</script>

<Story
  asChild
  name="Expanded"
  args={{ compact: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Select Build host server" }),
    );
    await expect(
      canvas.getByRole("button", { name: "Select Build host server" }),
    ).toHaveAttribute("aria-pressed", "true");
    await expect(canvas.getByText("Selected Build host")).toBeVisible();
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>
<Story
  asChild
  name="Compact keyboard"
  args={{ compact: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const research = canvas.getByRole("button", {
      name: "Select Research server",
    });
    research.focus();
    await userEvent.keyboard("{Enter}");
    await expect(research).toHaveAttribute("aria-pressed", "true");
  }}>{#snippet children()}<Fixture compact />{/snippet}</Story
>
<Story
  asChild
  name="Connection dialog"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Add server" }));
    const page = within(document.body);
    await userEvent.type(page.getByRole("textbox", { name: "Name" }), "Worker");
    await userEvent.type(
      page.getByRole("textbox", { name: "Server address" }),
      "https://worker.example",
    );
    await userEvent.type(
      page.getByLabelText("Access token"),
      "session-only-credential",
    );
    await userEvent.click(page.getByRole("button", { name: /^Connect$/ }));
    await expect(canvas.getByText("Connected Worker")).toBeVisible();
  }}>{#snippet children()}<Fixture />{/snippet}</Story
>
<Story asChild name="Mobile"
  >{#snippet children()}<div style="width:320px">
      <Fixture />
    </div>{/snippet}</Story
>
