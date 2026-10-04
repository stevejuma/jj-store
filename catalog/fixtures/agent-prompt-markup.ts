export const agentPromptMarkup = `Before **context**.

<harness:context source="cli > desktop">
## Session notes

- Keep **original** records.
- Preserve [safe links](https://example.com).

<arbitrary_limits category="host">
Use \`cargo test\`.
</arbitrary_limits>

<environment_context>
**Folder**: \`/workspace/demo\`

\`\`\`xml
<literal><keep_this /></literal>
\`\`\`
</environment_context>
</harness:context>

Between **contexts**.

<another-tool.context>Keep **Markdown** inside one-line sections.</another-tool.context>

After **context**. Inline \`<example />\` stays code.
`;

export const inertPromptMarkup =
  '<payload onclick="alert(1)">\n\n[reference][doc]\n\n<image src="https://example.invalid/leak" />\n\n<script>window.promptExecuted = true</script>\n\n</payload>\n\n[doc]: https://example.com\n';
