/** Invoke a fixture transition without adding synthetic controls to the application canvas. */
export async function storyControl(canvas: HTMLElement, name: string) {
  const controls = canvas.querySelectorAll<HTMLElement>(
    "[data-story-controls]",
  );
  for (const group of controls) {
    const button = [
      ...group.querySelectorAll<HTMLButtonElement>("button"),
    ].find(
      (button) =>
        (button.getAttribute("aria-label") ??
          button.textContent?.replace(/\s+/g, " ").trim()) === name,
    );
    if (button) {
      if (button.disabled)
        throw Error(`Fixture transition is disabled: ${name}`);
      button.click();
      await Promise.resolve();
      return;
    }
  }
  throw Error(`Missing fixture transition: ${name}`);
}
