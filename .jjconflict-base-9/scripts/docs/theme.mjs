import { readFile, writeFile, mkdir, copyFile } from "node:fs/promises";
import { dirname, join, basename } from "node:path";
import { createRequire } from "node:module";

/** Bundle the owning application's palettes and released fonts, without a second theme catalog. */
export async function themeAssets() {
  const destination = "docs/site/assets/generated";
  const appearance = "web/src/lib/appearance";
  await mkdir(destination, { recursive: true });
  const index = await readFile(join(appearance, "theme-palettes.css"), "utf8");
  const sheets = [];
  for (const match of index.matchAll(
    /@import "(\.\/palettes\/[a-z0-9-]+\.css)";/g,
  )) {
    sheets.push(await readFile(join(appearance, match[1]), "utf8"));
  }
  if (!sheets.length) throw Error("Application theme catalog is empty");
  sheets.push(await readFile(join(appearance, "appearance.css"), "utf8"));
  const require = createRequire(
    import.meta.resolve("@lapismd/design-core/styles.css"),
  );
  for (const name of ["dm-sans", "source-code-pro"]) {
    const source = require.resolve(`@fontsource-variable/${name}/index.css`);
    let css = await readFile(source, "utf8");
    for (const match of css.matchAll(
      /url\((\.\/files\/[a-z0-9-]+\.woff2)\)/g,
    )) {
      await copyFile(
        join(dirname(source), match[1]),
        join(destination, basename(match[1])),
      );
    }
    css = css.replaceAll("./files/", "./");
    const license = await readFile(join(dirname(source), "LICENSE"), "utf8");
    sheets.push(`/* ${name}: ${license.replaceAll("*/", "* /")} */\n${css}`);
  }
  await writeFile(join(destination, "app-theme.css"), sheets.join("\n"));
}
