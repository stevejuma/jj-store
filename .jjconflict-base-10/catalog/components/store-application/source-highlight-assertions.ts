import { expect, waitFor } from "storybook/test";

export async function expectSourceHighlight(
  root: HTMLElement,
  language: string,
  heading: string | RegExp,
) {
  await waitFor(() => {
    const block = root.querySelector<HTMLElement>(
      `[data-ui-component="code-block"][data-language="${language}"]`,
    )!;
    const token = block.querySelector<HTMLElement>(".ui-code-token-keyword")!;
    expect(token).toHaveTextContent(heading);
    expect(getComputedStyle(token).color).not.toBe(
      getComputedStyle(block.querySelector("code")!).color,
    );
  });
}
