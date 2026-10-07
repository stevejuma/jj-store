import type { Model } from "../../api/client.js";
import type { ActivityRow } from "../store-agent-execution/presentation.js";
import type { TranscriptToolEntry } from "./transcript-tool-rich.js";
import {
  displayValue,
  outputText,
  pathFromRecord,
  record,
} from "./transcript-tool-values.js";

const encode = (value: unknown): string | undefined =>
  value === undefined
    ? undefined
    : typeof value === "string"
      ? value
      : JSON.stringify(value);
const pathsIn = (value: unknown): string[] => {
  const data = record(value);
  const locations = Array.isArray(data?.locations) ? data.locations : [];
  return [
    ...new Set(
      [
        pathFromRecord(data),
        ...locations.map((v) => pathFromRecord(record(v))),
      ].filter((p): p is string => !!p),
    ),
  ];
};
/** Adapts sanitized retained ACP facts, without changing their raw representation. */
export function liveTranscriptTool(item: ActivityRow): TranscriptToolEntry {
  const value = item.value;
  const input = encode(value.rawInput);
  const output = encode(value.rawOutput ?? value.content);
  const paths = [
    ...new Set([...pathsIn(value), ...pathsIn(displayValue(input))]),
  ];
  return {
    id: item.id,
    name: item.text || String(value.name ?? "Tool call"),
    kind: typeof value.kind === "string" ? value.kind : undefined,
    paths,
    readContent:
      value.kind === "read" && value.status !== "failed"
        ? outputText(value.rawOutput)
        : undefined,
    redacted: value.redacted === true,
    truncated: value.truncated === true,
    input,
    output: value.status === "failed" ? undefined : output,
    error:
      value.status === "failed"
        ? encode(value.rawOutput ?? value.error ?? "Tool failed")
        : undefined,
  };
}
/** Existing operation-selected patches supplement captured tool evidence only. */
export function recordedTranscriptTool(
  tool: Model<"AgentToolActivity">,
  patches: Readonly<Record<string, string>> = {},
): TranscriptToolEntry {
  const input = tool.input?.content;
  const paths = [
    ...new Set([
      ...tool.code.flatMap((c) => c.paths),
      ...tool.observed_paths,
      ...pathsIn(displayValue(input)),
      ...pathsIn(displayValue(tool.output?.content)),
    ]),
  ];
  const files = paths.flatMap((path) => {
    const patch =
      patches[path] ??
      Object.entries(patches).find(([p]) => path.endsWith(`/${p}`))?.[1];
    return patch ? [{ path, patch }] : [];
  });
  const edit = ["edit", "delete", "move"].includes(tool.kind);
  return {
    id: tool.tool_call_id,
    name: tool.native_name,
    kind: edit ? "edit" : tool.kind,
    paths,
    readContent:
      tool.kind === "read" && tool.state !== "failed"
        ? tool.output?.media_type === "text"
          ? tool.output.content
          : (outputText(displayValue(tool.output?.content)) ??
            tool.output?.content)
        : undefined,
    redacted:
      record(displayValue(tool.input?.content))?.redacted === true ||
      record(displayValue(tool.output?.content))?.redacted === true,
    truncated:
      record(displayValue(tool.input?.content))?.truncated === true ||
      record(displayValue(tool.output?.content))?.truncated === true,
    input,
    output:
      tool.state === "failed"
        ? undefined
        : files.length && edit
          ? encode(files)
          : tool.output?.content,
    error:
      tool.state === "failed"
        ? (tool.output?.content ?? "Tool failed")
        : undefined,
  };
}
/** Only unwrap displayable text; unsupported blocks remain available as raw JSON. */
export function capturedToolError(
  tool: TranscriptToolEntry,
): string | undefined {
  return outputText(displayValue(tool.error)) ?? tool.error;
}
