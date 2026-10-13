import { AnsiUp } from "ansi_up";
import type { WorkflowEvidence } from "../../controller/workflow-types.js";

export type ConsoleLine = {
  id: string;
  stream: string;
  html: string;
  text: string;
  at: number;
};
type Stream = {
  ansi: AnsiUp;
  current?: ConsoleLine;
  carriageReturn: boolean;
  linkBuffer: string;
};

/** Retain parsed lines across refresh; only append newly received immutable chunks. */
export class WorkflowLogLines {
  private ids: string[] = [];
  private streams = new Map<string, Stream>();
  private lines: ConsoleLine[] = [];
  private view: ConsoleLine[] = [];
  read(chunks: readonly WorkflowEvidence[]) {
    if (
      chunks.length < this.ids.length ||
      this.ids.some((id, i) => id !== chunks[i]?.id)
    ) {
      this.ids = [];
      this.streams.clear();
      this.lines = [];
      this.view = [];
    }
    if (chunks.length === this.ids.length) return this.view;
    for (const chunk of chunks.slice(this.ids.length)) {
      this.ids.push(chunk.id);
      if (chunk.data.type !== "log") continue;
      const name = chunk.data.stream;
      let stream = this.streams.get(name);
      if (!stream) {
        const ansi = new AnsiUp();
        ansi.escape_html = true;
        ansi.use_classes = true;
        // Logs are output text, not executable links supplied by a process.
        ansi.url_allowlist = {};
        stream = { ansi, carriageReturn: false, linkBuffer: "" };
        this.streams.set(name, stream);
      }
      let offset = 0;
      const text = this.stripLinks(stream, chunk.data.text);
      for (const part of text.split(/(\r|\n)/)) {
        if (!part) continue;
        if (part === "\n") {
          if (!stream.carriageReturn) {
            stream.current ??= this.line(
              chunk.id,
              offset++,
              name,
              Number(chunk.at),
            );
            stream.current = undefined;
          }
          stream.carriageReturn = false;
        } else if (part === "\r") {
          stream.current = undefined;
          stream.carriageReturn = true;
        } else {
          stream.carriageReturn = false;
          const html = stream.ansi.ansi_to_html(part);
          if (!html) continue;
          stream.current ??= this.line(
            chunk.id,
            offset++,
            name,
            Number(chunk.at),
          );
          stream.current.html += html;
          stream.current.text += part;
        }
      }
    }
    this.view = this.lines.map((line) => ({ ...line }));
    return this.view;
  }
  private line(chunk: string, offset: number, stream: string, at: number) {
    const line = { id: `${chunk}:${offset}`, stream, html: "", text: "", at };
    this.lines.push(line);
    return line;
  }
  private stripLinks(stream: Stream, text: string) {
    text = (stream.linkBuffer + text).replace(
      /\x1b\]8;[^\x07]*?(?:\x07|\x1b\\)/g,
      "",
    );
    stream.linkBuffer = "";
    const index = text.lastIndexOf("\x1b"),
      suffix = text.slice(index);
    if (
      index >= 0 &&
      ("\x1b]8;".startsWith(suffix) || suffix.startsWith("\x1b]8;"))
    ) {
      stream.linkBuffer = suffix;
      return text.slice(0, index);
    }
    return text;
  }
}

export function consoleLogRows(lines: readonly ConsoleLine[]) {
  const groups: string[] = [];
  return lines.flatMap((line) => {
    if (line.text.startsWith("##[endgroup]")) {
      groups.pop();
      return [];
    }
    const group = line.text.startsWith("##[group]");
    const row = {
      ...line,
      group,
      parents: [...groups],
      html: group ? line.html.replace("##[group]", "") : line.html,
    };
    if (group) groups.push(line.id);
    return [row];
  });
}
