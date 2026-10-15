<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
  import ForgeWorkspaceFixture from "../../fixtures/ForgeWorkspaceFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Forge/Inline checks",
    component: ForgeWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    beforeEach: async () => {
      await views.stacks();
    },
  });
  async function headerHover({
    canvasElement,
  }: {
    canvasElement: HTMLElement;
  }) {
    const section = await within(canvasElement).findByRole("region", {
      name: "Pull request for feature/api",
    });
    const header = section.querySelector("header")!;
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const sample = document.createElement("canvas");
    sample.width = sample.height = 1;
    const context = sample.getContext("2d")!;
    const color = (value: string) => {
      context.clearRect(0, 0, 1, 1);
      context.fillStyle = value;
      context.fillRect(0, 0, 1, 1);
      return [...context.getImageData(0, 0, 1, 1).data];
    };
    const difference = (a: number[], b: number[]) =>
      Math.max(...a.slice(0, 3).map((v, i) => Math.abs(v - b[i]!)));
    for (const button of within(
      header.querySelector<HTMLElement>(".forge-inline__actions")!,
    ).getAllByRole("button")) {
      if ((button as HTMLButtonElement).disabled) continue;
      button.scrollIntoView({ block: "nearest", inline: "nearest" });
      await pointer.hover(header, { position: { x: 4, y: 4 } });
      await pointer.hover(button);
      await waitFor(() => {
        expect(button.matches(":hover")).toBe(true);
        const hover = color(getComputedStyle(button).backgroundColor);
        expect(hover[3]).toBe(255);
        expect(
          difference(hover, color(getComputedStyle(header).backgroundColor)),
          button.getAttribute("aria-label") ?? "Header action",
        ).toBeGreaterThanOrEqual(12);
      });
      button.focus();
      await expect(button).toHaveFocus();
    }
    await pointer.hover(canvasElement, { position: { x: 0, y: 0 } });
  }
</script>

<Story
  name="Checks and logs stay in the PR body through float and redock"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const details = await canvas.findByRole("region", {
      name: "Selected changes from stack-api",
    });
    const section = within(details).getByRole("region", {
      name: "Pull request for feature/api",
    });
    const file = within(details).getByRole("region", {
      name: "Selected file diff",
    });
    const inspect = within(section).getByRole("button", {
      name: "Inspect checks",
    });
    await expect(
      within(section).queryByRole("region", { name: "CI checks" }),
    ).not.toBeInTheDocument();
    await userEvent.click(inspect);
    const checks = await within(section).findByRole("region", {
      name: "CI checks",
    });
    await expect(inspect).toHaveAttribute("aria-expanded", "true");
    await expect(
      within(section).getByRole("region", { name: "PR 42" }),
    ).toContainElement(checks);
    await expect(
      canvas.queryByRole("region", { name: "Pull requests" }),
    ).not.toBeInTheDocument();
    await expect(file).toBeVisible();
    await userEvent.click(
      within(checks).getAllByRole("button", { name: "Jobs" })[0]!,
    );
    await userEvent.click(
      (await within(checks).findAllByRole("button", { name: "View logs" }))[0]!,
    );
    const logs = await within(checks).findByLabelText("Job logs");
    await expect(logs).toHaveTextContent("42 passed");
    await userEvent.click(
      within(section).getByRole("button", { name: "Edit details" }),
    );
    const title = within(section).getByRole("textbox", { name: "Title" });
    await userEvent.clear(title);
    await userEvent.type(title, "Retained review title");
    await userEvent.click(
      within(section).getByRole("button", { name: "Float PR details" }),
    );
    const floating = await within(canvasElement.ownerDocument.body).findByRole(
      "dialog",
      { name: "PR #42 · feature/api" },
    );
    await expect(
      within(floating).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await expect(within(floating).getByLabelText("Job logs")).toBe(logs);
    await expect(within(floating).getByRole("textbox", { name: "Title" })).toBe(
      title,
    );
    await expect(title).toHaveValue("Retained review title");
    await expect(file).toBeVisible();
    await userEvent.click(
      within(floating).getByRole("button", { name: "Redock floating pane" }),
    );
    await expect(
      within(section).getByRole("region", { name: "CI checks" }),
    ).toBe(checks);
    await expect(
      within(details).getByRole("region", { name: "Selected file diff" }),
    ).toBe(file);
    await userEvent.click(inspect);
    await expect(inspect).toHaveAttribute("aria-expanded", "false");
    await expect(
      within(section).queryByRole("region", { name: "CI checks" }),
    ).not.toBeInTheDocument();
    await userEvent.click(inspect);
    await expect(within(section).getByLabelText("Job logs")).toHaveTextContent(
      "42 passed",
    );
    await expect(title).toHaveValue("Retained review title");
  }}
/>

<Story
  name="Inspect checks expands a collapsed PR in the same column"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const section = await canvas.findByRole("region", {
      name: "Pull request for feature/api",
    });
    await userEvent.click(
      within(section).getByRole("button", {
        name: "Collapse pull request #42",
      }),
    );
    const inspect = within(section).getByRole("button", {
      name: "Inspect checks",
    });
    inspect.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await within(section).findByRole("region", { name: "CI checks" }),
    ).toBeVisible();
    await expect(
      within(section).getByRole("button", {
        name: "Collapse pull request #42",
      }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("region", { name: "Pull requests" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      within(section).getByRole("button", {
        name: "Collapse pull request #42",
      }),
    );
    inspect.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      await within(section).findByRole("region", { name: "CI checks" }),
    ).toBeVisible();
  }}
/>

<Story
  name="PR header actions have visible hover in light mode"
  globals={{ colorMode: "light" }}
  play={headerHover}
/>
<Story
  name="PR header actions have visible hover in dark mode"
  globals={{ colorMode: "dark" }}
  play={headerHover}
/>
