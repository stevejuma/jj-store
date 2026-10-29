import { expect, userEvent, waitFor, within } from "storybook/test";
export const browseGguf = async ({
  canvasElement,
}: {
  canvasElement: HTMLElement;
}) => {
  const c = within(canvasElement);
  await userEvent.click(await c.findByRole("button", { name: "Browse…" }));
  await expect(c.getByLabelText("Import GGUF")).toHaveValue(
    "/models/My model.gguf",
  );
  await expect(c.getByLabelText("Compute commands")).toHaveTextContent("[]");
  await userEvent.click(c.getByRole("button", { name: "Import model" }));
  await waitFor(() =>
    expect(
      JSON.parse(c.getByLabelText("Compute commands").textContent ?? "[]")[0]
        .action,
    ).toEqual({ action: "import", path: "/models/My model.gguf" }),
  );
};
