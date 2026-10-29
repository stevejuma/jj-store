import { expect, userEvent, within, waitFor } from "storybook/test";
/** Follow the production overflow menu when a viewport cannot show every tab. */
export async function selectViewTab(element: HTMLElement, name: string) {
  const canvas = within(element);
  let tab: HTMLElement | null = null;
  let more: HTMLElement | null = null;
  await waitFor(() => {
    tab = canvas.queryByRole("tab", { name });
    more = canvas.queryByRole("button", {
      name: /^More (?:repository sections|local repository views|project views|projects views)$/i,
    });
    expect(tab ?? more).not.toBeNull();
  });
  if (tab) await userEvent.click(tab);
  else {
    await userEvent.click(more!);
    await userEvent.click(
      await within(element.ownerDocument.body).findByRole("menuitem", {
        name,
      }),
    );
  }
}

/** Assert the actual selected destination even when its tab lives in More. */
export async function expectSelectedViewTab(
  element: HTMLElement,
  name: string,
) {
  const canvas = within(element);
  await waitFor(() =>
    expect(
      canvas.queryByRole("tab", { name }) ??
        canvas.queryByRole("button", {
          name: /^More (?:repository sections|local repository views|project views|projects views)$/i,
        }),
    ).not.toBeNull(),
  );
  const tab = canvas.queryByRole("tab", { name });
  if (tab) {
    await expect(tab).toHaveAttribute("aria-selected", "true");
    return;
  }
  const more = canvas.getByRole("button", {
    name: /^More (?:repository sections|local repository views|project views|projects views)$/i,
  });
  await expect(more).toHaveAttribute("aria-current", "page");
  await userEvent.click(more);
  await expect(
    await within(element.ownerDocument.body).findByRole("menuitem", {
      name,
    }),
  ).toHaveAttribute("aria-current", "page");
  await userEvent.keyboard("{Escape}");
  await waitFor(() =>
    expect(within(element.ownerDocument.body).queryByRole("menu")).toBeNull(),
  );
  await waitFor(() =>
    expect(element.ownerDocument.body).not.toHaveStyle({
      pointerEvents: "none",
    }),
  );
}
