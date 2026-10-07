import { diffSequences, DEFAULT_OPTIONS } from "@lapismd/design-core/diff";
import {
  displayValue,
  outputText,
  stderrText,
  queryFromValue,
  queryFromCommand,
  patchText,
  pathFromRecord,
  pathFromPatch,
  arrayFromRecord,
  record,
  stringValue,
  normalized,
} from "./transcript-tool-values.js";
import type {
  RichFileEdit,
  RichSearchMatch,
  RichTranscriptToolDetail,
  TranscriptToolEntry,
} from "./transcript-tool-rich.js";

const SHELL_TOOLS = new Set([
  "bash",
  "command",
  "exec",
  "execute",
  "run",
  "shell",
  "sh",
  "terminal",
]);
const SEARCH_COMMAND = /^(?:rg|grep|find)\b/u;
const EDIT_NAME =
  /(?:apply[_ -]?patch|edit|write|replace|update).*(?:file|source|patch)?|(?:file|source|patch).*(?:edit|write|replace|update)/iu;

export function detectRichTranscriptToolDetail(
  tool: TranscriptToolEntry,
): RichTranscriptToolDetail | undefined {
  const input = displayValue(tool.input);
  const output = displayValue(tool.output);
  const error = displayValue(tool.error);
  const body = tool.readContent ?? outputText(output);
  if (tool.kind === "read" && body !== undefined) {
    const paths = tool.paths ?? [];
    return {
      kind: "file-read",
      ...(paths.length === 1 ? { path: paths[0] } : {}),
      content: body,
    };
  }
  const fileEdit = fileEditDetail(tool, input, output);
  if (fileEdit) return fileEdit;
  const command = commandForTool(tool, input);
  const search = searchDetail(command, input, output);
  if (search) return search;
  const shell = shellDetail(tool, command, output, error);
  if (shell) return shell;
}

function fileEditDetail(
  tool: TranscriptToolEntry,
  input: unknown,
  output: unknown,
): RichTranscriptToolDetail | undefined {
  if (tool.kind !== "edit" && !EDIT_NAME.test(tool.name)) return;
  const candidates = dedupeFileEdits([
    ...fileEditsFromValue(input),
    ...fileEditsFromValue(output),
  ]);
  if (candidates.length) return { kind: "file-edit", files: candidates };
  const patch = patchText(output) ?? patchText(input);
  if (!patch) return;
  const path =
    pathFromPatch(patch) ||
    pathFromRecord(record(input)) ||
    pathFromRecord(record(output));
  if (!path) return;
  return { kind: "file-edit", files: [fileEdit(path, { patch })] };
}

function dedupeFileEdits(files: RichFileEdit[]): RichFileEdit[] {
  return [...new Map(files.map((file) => [file.path, file])).values()];
}

function fileEditsFromValue(value: unknown): RichFileEdit[] {
  const direct = fileEditFromRecord(record(value));
  if (direct) return [direct];
  const files = Array.isArray(value)
    ? value
    : (arrayFromRecord(value, "files") ?? arrayFromRecord(value, "changes"));
  if (!files) return [];
  return files.flatMap((item) => {
    const edit = fileEditFromRecord(record(item));
    return edit ? [edit] : [];
  });
}

function fileEditFromRecord(value: Record<string, unknown> | undefined) {
  if (!value) return;
  const content = stringValue(value.content ?? value.text);
  const patch =
    stringValue(
      value.patch ?? value.diff ?? value.unifiedPatch ?? value.unified_diff,
    ) ?? patchText(content);
  const path =
    pathFromRecord(value) || (patch ? pathFromPatch(patch) : undefined);
  if (!path) return;
  if (patch) return fileEdit(path, { patch });
  const oldText = textValue(
    value.oldText ??
      value.old_string ??
      value.oldString ??
      value.before ??
      value.original ??
      value.previous,
  );
  const newText = textValue(
    value.newText ??
      value.new_string ??
      value.newString ??
      value.after ??
      value.updated ??
      content,
  );
  if (oldText === undefined && newText === undefined) return;
  return {
    ...fileEdit(path, { oldText: oldText ?? null, newText: newText ?? "" }),
    capturedText: newText ?? oldText ?? "",
    partial:
      !(oldText !== undefined || value.oldText === null) ||
      !(newText !== undefined || value.newText === null),
  };
}

function fileEdit(
  path: string,
  input: { patch?: string; oldText?: string | null; newText?: string | null },
): RichFileEdit {
  const stats = input.patch
    ? patchStats(input.patch)
    : textStats(input.oldText ?? "", input.newText ?? "");
  return { path, ...input, ...stats };
}

function shellDetail(
  tool: TranscriptToolEntry,
  command: string | undefined,
  output: unknown,
  error: unknown,
): RichTranscriptToolDetail | undefined {
  const name = normalized(tool.name);
  const shellTool = SHELL_TOOLS.has(name) || command !== undefined;
  if (!shellTool || !command) return;
  const body = outputText(output);
  const errorText = outputText(error) ?? stderrText(output);
  return {
    kind: "shell",
    command,
    ...(body ? { output: body } : {}),
    ...(errorText ? { error: errorText } : {}),
  };
}

function searchDetail(
  command: string | undefined,
  input: unknown,
  output: unknown,
): RichTranscriptToolDetail | undefined {
  if (!command || !SEARCH_COMMAND.test(command.trim())) return;
  const body = outputText(output);
  const matches = body ? searchMatches(body) : [];
  if (matches.length === 0) return;
  const query = queryFromValue(input) ?? queryFromCommand(command);
  return {
    kind: "search",
    command,
    ...(query ? { query } : {}),
    matches,
    ...(body ? { output: body } : {}),
  };
}

function searchMatches(text: string): RichSearchMatch[] {
  return text.split(/\r?\n/u).flatMap((line) => {
    const match = /^(.*?):(\d+)(?::(\d+))?:(.*)$/u.exec(line);
    if (!match) return [];
    return [
      {
        path: match[1]!,
        line: Number(match[2]),
        ...(match[3] ? { column: Number(match[3]) } : {}),
        text: match[4]!.trim(),
      },
    ];
  });
}

function commandForTool(
  tool: TranscriptToolEntry,
  input: unknown,
): string | undefined {
  const source = record(input);
  const command = stringValue(source?.command ?? source?.cmd);
  if (command) return command;
  const name = tool.name.trim();
  const ran = /^Ran\s+(.+)$/iu.exec(name);
  if (ran?.[1]) return ran[1].trim();
  if (SHELL_TOOLS.has(normalized(name)) && typeof input === "string")
    return input.trim() || undefined;
}

function patchStats(patch: string) {
  return {
    additions: (patch.match(/^\+(?!\+\+)/gmu) ?? []).length,
    deletions: (patch.match(/^-(?!--)/gmu) ?? []).length,
  };
}

function textStats(oldText: string, newText: string) {
  const lines = (text: string) =>
    text ? text.replace(/\r\n/g, "\n").replace(/\n$/, "").split("\n") : [];
  const changes = diffSequences(
    lines(oldText),
    lines(newText),
    DEFAULT_OPTIONS,
  );
  return {
    additions: changes.filter((r) => r.kind === "right").length,
    deletions: changes.filter((r) => r.kind === "left").length,
  };
}

function textValue(value: unknown): string | undefined {
  return typeof value === "string" ? value : undefined;
}
