import { cpp } from "@codemirror/lang-cpp";
import { css } from "@codemirror/lang-css";
import { html } from "@codemirror/lang-html";
import { markdown } from "@codemirror/lang-markdown";
import { php } from "@codemirror/lang-php";
import { rust } from "@codemirror/lang-rust";
import { xml } from "@codemirror/lang-xml";
import { yaml } from "@codemirror/lang-yaml";
import { StreamLanguage } from "@codemirror/language";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { toml } from "@codemirror/legacy-modes/mode/toml";
import type { Extension } from "@codemirror/state";
import { resolveDiffLanguage } from "@lapismd/design-core/diff";

const LANGUAGE_NAMES: Readonly<Record<string, string>> = {
  bash: "Shell",
  c: "C",
  cpp: "C++",
  css: "CSS",
  html: "HTML",
  java: "Java",
  javascript: "JavaScript",
  json: "JSON",
  jsx: "JSX",
  less: "Less",
  markdown: "Markdown",
  php: "PHP",
  plaintext: "Plain text",
  python: "Python",
  rust: "Rust",
  scss: "SCSS",
  svg: "SVG",
  toml: "TOML",
  tsx: "TSX",
  typescript: "TypeScript",
  xml: "XML",
  yaml: "YAML",
  zsh: "Shell",
};

export type RepositoryEditorLanguage = {
  id: string;
  name: string;
  extension: Extension;
};

export function repositoryEditorLanguage(
  path: string,
): RepositoryEditorLanguage {
  const id = path.toLowerCase().endsWith(".toml")
    ? "toml"
    : (resolveDiffLanguage(path) ?? "plaintext");
  return {
    id,
    name: LANGUAGE_NAMES[id] ?? id,
    extension: repositoryEditorLanguageExtension(id),
  };
}

function repositoryEditorLanguageExtension(language: string): Extension {
  if (language === "markdown") return markdown();
  if (language === "rust") return rust();
  if (language === "toml") return StreamLanguage.define(toml);
  if (language === "bash" || language === "zsh")
    return StreamLanguage.define(shell);
  if (language === "yaml") return yaml();
  if (language === "c" || language === "cpp") return cpp();
  if (language === "css" || language === "less" || language === "scss")
    return css();
  if (language === "html") return html();
  if (language === "php") return php();
  if (language === "xml" || language === "svg") return xml();
  return [];
}
