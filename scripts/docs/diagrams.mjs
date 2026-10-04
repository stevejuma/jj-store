import { createHash } from "node:crypto";
import { readFile, writeFile, readdir, mkdir, rm } from "node:fs/promises";
import { chromium } from "@playwright/test";
export async function diagrams() {
  const entries = new Map();
  async function walk(dir) {
    for (const file of await readdir(dir, { withFileTypes: true })) {
      const path = dir + "/" + file.name;
      if (file.isDirectory() && file.name !== "assets") await walk(path);
      if (file.isFile() && file.name.endsWith(".md")) {
        const text = await readFile(path, "utf8");
        for (const match of text.matchAll(/```mermaid\n([\s\S]*?)\n```/g)) {
          const code = match[1];
          entries.set(
            createHash("sha256").update(code).digest("hex").slice(0, 20),
            code,
          );
        }
      }
    }
  }
  await walk("docs/site");
  const dir = "docs/site/assets/generated";
  await rm(dir, { recursive: true, force: true });
  await mkdir(dir, { recursive: true });
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage();
    await page.route("**/*", (route) => route.abort());
    await page.setContent("<!doctype html><html><body></body></html>");
    await page.addScriptTag({
      content: await readFile(
        "node_modules/mermaid/dist/mermaid.min.js",
        "utf8",
      ),
    });
    for (const dark of [false, true]) {
      await page.evaluate(
        (dark) =>
          window.mermaid.initialize({
            startOnLoad: false,
            securityLevel: "strict",
            theme: dark ? "dark" : "neutral",
            fontFamily: "sans-serif",
          }),
        dark,
      );
      for (const [key, code] of entries) {
        const svg = await page.evaluate(
          async ({ key, code }) =>
            (await window.mermaid.render("diagram" + key, code)).svg,
          { key, code },
        );
        await writeFile(`${dir}/${key}${dark ? ".dark" : ""}.svg`, svg);
      }
    }
  } finally {
    await browser.close();
  }
  console.log(
    `Rendered ${entries.size} Mermaid diagrams in light and dark without network access`,
  );
}
