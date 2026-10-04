import { describe, expect, test } from "vitest";
import { repositoryEditorLanguage } from "../../../web/src/lib/ui/community-projects/repository-editor-language.js";

describe("repository editor language", () => {
  test("selects source highlighters from repository file names", () => {
    expect(repositoryEditorLanguage("README.md").name).toBe("Markdown");
    expect(repositoryEditorLanguage("Cargo.toml").name).toBe("TOML");
    expect(repositoryEditorLanguage("src/main.rs").name).toBe("Rust");
    expect(repositoryEditorLanguage("scripts/release.sh").name).toBe("Shell");
    expect(repositoryEditorLanguage("web/app.ts").name).toBe("TypeScript");
  });

  test("falls back to an unhighlighted editor for extensionless prose", () => {
    const language = repositoryEditorLanguage("LICENSE");
    expect(language.name).toBe("Plain text");
    expect(language.extension).toEqual([]);
  });
});
