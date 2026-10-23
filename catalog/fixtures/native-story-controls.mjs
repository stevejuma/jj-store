/** Fixture transitions are separate from product pointer/keyboard acceptance. */
export async function nativeStoryControl(page, name) {
  await page.locator("[data-story-controls]").evaluateAll((groups, label) => {
    const button = groups
      .flatMap((group) => [...group.querySelectorAll("button")])
      .find(
        (button) =>
          (button.getAttribute("aria-label") ??
            button.textContent?.replace(/\s+/g, " ").trim()) === label,
      );
    if (!button || button.disabled)
      throw Error(`Unavailable fixture transition: ${label}`);
    button.click();
  }, name);
}
