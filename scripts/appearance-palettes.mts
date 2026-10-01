import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { createThemeVars } from "../web/src/lib/appearance/adaptive-theme.js";
import { format } from "prettier";
import { themeNames } from "../web/src/lib/appearance/theme-catalog.js";

// Buzz palette extraction at ef2aa1ae (Apache-2.0); npm Shiki themes 4.0.2.
const syntaxScopes: Record<string, string[]> = {
  keyword: ["keyword"],
  string: ["string"],
  comment: ["comment"],
  number: ["constant.numeric"],
  function: ["entity.name.function"],
  type: ["entity.name.type", "support.type"],
  variable: ["variable"],
  operator: ["keyword.operator"],
  constant: ["constant"],
  tag: ["entity.name.tag"],
  attribute: ["entity.other.attribute-name"],
  property: ["variable.other.property"],
  punctuation: ["punctuation"],
};
const output: string[] = [
  "/* Generated from Shiki 4.0.2 and Buzz ef2aa1ae; run pnpm exec tsx scripts/appearance-palettes.mts. */",
];
for (const name of themeNames) {
  const base =
    name === "buzz"
      ? "github-light"
      : name === "buzz-dark"
        ? "github-dark"
        : name;
  const theme = (await import(`@shikijs/themes/${base}`)).default;
  const colors = theme.colors ?? {};
  const bg = colors["editor.background"] ?? theme.bg ?? "#1e1e1e";
  const fg = colors["editor.foreground"] ?? theme.fg ?? "#d4d4d4";
  const settings = theme.tokenColors ?? theme.settings ?? [];
  function scopeColor(scopes: string[], fallback: string) {
    for (const scope of scopes) {
      const setting = settings.find(
        (s: {
          scope?: string | string[];
          settings?: { foreground?: string };
        }) =>
          (Array.isArray(s.scope)
            ? s.scope
            : (s.scope?.split(",").map((x) => x.trim()) ?? [])
          ).includes(scope) && s.settings?.foreground,
      );
      if (setting) return setting.settings.foreground;
    }
    return fallback;
  }
  const git = (keys: string[]) => {
    for (const key of keys) if (colors[key]) return colors[key].slice(0, 7);
    return null;
  };
  const result = createThemeVars(bg, fg, scopeColor(["comment"], fg), {
    added: git([
      "gitDecoration.addedResourceForeground",
      "editorGutter.addedBackground",
      "diffEditor.insertedTextBackground",
    ]),
    deleted: git([
      "gitDecoration.deletedResourceForeground",
      "editorGutter.deletedBackground",
      "diffEditor.removedTextBackground",
    ]),
    modified: git([
      "gitDecoration.modifiedResourceForeground",
      "editorGutter.modifiedBackground",
    ]),
  });
  const vars: Record<string, string> = {};
  for (const [key, value] of Object.entries(result.vars)) {
    vars[key === "--sidebar-background" ? "--sidebar" : key] = /^\d/.test(value)
      ? `hsl(${value})`
      : value;
  }
  vars["--primary"] = "var(--store-accent-color, #3b82f6)";
  vars["--ring"] = "var(--primary)";
  vars["--sidebar-primary"] = "var(--primary)";
  vars["--sidebar-primary-foreground"] = "var(--primary-foreground)";
  vars["--primary-foreground"] = "var(--store-accent-foreground, white)";
  if (name === "buzz" || name === "buzz-dark") {
    vars["--primary"] = vars["--foreground"]!;
    vars["--primary-foreground"] = vars["--background"]!;
  }
  vars["--store-theme-chrome"] = vars["--sidebar"]!;
  vars["--store-theme-gradient"] =
    name === "buzz"
      ? "linear-gradient(180deg, #e6e6b6, #c4d0da)"
      : name === "buzz-dark"
        ? "linear-gradient(180deg, #4a4616, #0a1423)"
        : vars["--sidebar"]!;
  for (const [token, scopes] of Object.entries(syntaxScopes))
    vars[`--ui-code-syntax-${token}`] = scopeColor(scopes, fg);
  vars["--ui-code-syntax-background"] = bg;
  for (const token of [
    "keyword",
    "string",
    "comment",
    "number",
    "type",
    "variable",
    "operator",
    "property",
  ])
    vars[`--mira-syntax-${token}`] = vars[`--ui-code-syntax-${token}`]!;
  vars["--mira-syntax-heading"] = vars["--ui-code-syntax-function"]!;
  vars["--mira-syntax-link"] = vars["--ui-code-syntax-string"]!;
  const path = new URL(
    `../web/src/lib/appearance/palettes/${name}.css`,
    import.meta.url,
  );
  const content = await format(
    `/* Generated Shiki 4.0.2 / Buzz ef2aa1ae palette. */\n[data-store-theme="${name}"] {\n${Object.entries(
      vars,
    )
      .map(([key, value]) => `  ${key}: ${value};`)
      .join("\n")}\n}\n`,
    { parser: "css" },
  );
  if (process.argv.includes("--check")) {
    if (readFileSync(path, "utf8") !== content)
      throw new Error(`Stale palette ${name}`);
  } else {
    mkdirSync(new URL("../web/src/lib/appearance/palettes/", import.meta.url), {
      recursive: true,
    });
    writeFileSync(path, content);
  }
  output.push(`@import "./palettes/${name}.css";`);
}
const index = new URL(
  "../web/src/lib/appearance/theme-palettes.css",
  import.meta.url,
);
const content = output.join("\n") + "\n";
if (process.argv.includes("--check")) {
  if (readFileSync(index, "utf8") !== content)
    throw new Error("Stale palette index");
} else writeFileSync(index, content);
