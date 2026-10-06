import assert from "node:assert/strict";
import test from "node:test";
import { tokenizeRepositorySource } from "../src/lib/ui/community-projects/repository-source-tokenizer.js";

test("C and C++ source use genuine syntax tokens including preprocessor directives", () => {
  for (const language of ["c", "cpp"]) {
    const code = "#include <linux/module.h>\nstatic int value = 42;\n";
    const tokens = tokenizeRepositorySource(code, language);
    assert.ok(
      tokens.some(
        (token) =>
          token.type === "keyword" &&
          code.slice(token.start, token.end) === "static",
      ),
    );
    assert.ok(
      tokens.some(
        (token) =>
          token.type === "number" &&
          code.slice(token.start, token.end) === "42",
      ),
    );
    assert.ok(
      tokens.some(
        (token) =>
          token.type === "keyword" &&
          code.slice(token.start, token.end).includes("include"),
      ),
    );
  }
});

test("multiline comments retain their type without crossing source line boundaries", () => {
  for (const language of ["c", "cpp", "rust"]) {
    const code = "/* first\r\nsecond 😀 */\n";
    const tokens = tokenizeRepositorySource(code, language);
    assert.equal(tokens.filter((token) => token.type === "comment").length, 2);
    let previousEnd = 0;
    for (const token of tokens) {
      assert.ok(token.start >= previousEnd);
      assert.ok(token.end > token.start && token.end <= code.length);
      assert.ok(!code.slice(token.start, token.end).includes("\n"));
      previousEnd = token.end;
    }
  }
});

test("Rust lifetimes and raw strings preserve source offsets", () => {
  const code = `pub fn borrow<'a>(input: &'a str) -> &'a str {\n    let raw = r#"<sample>"#;\n    input\n}`;
  const tokens = tokenizeRepositorySource(code, "rust");
  assert.ok(
    tokens.some(
      (token) =>
        token.type === "keyword" &&
        code.slice(token.start, token.end) === "pub",
    ),
  );
  assert.ok(
    tokens.some(
      (token) =>
        token.type === "string" &&
        code.slice(token.start, token.end) === 'r#"<sample>"#',
    ),
  );
});

test("unknown and built-in languages stay with the default tokenizer", () => {
  assert.deepEqual(
    tokenizeRepositorySource("Read <documentation>", "plaintext"),
    [],
  );
  assert.deepEqual(
    tokenizeRepositorySource("const value = 42", "typescript"),
    [],
  );
  assert.deepEqual(tokenizeRepositorySource("", "c"), []);
});
