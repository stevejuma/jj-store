import type { Model } from "../api/client.js";
import { answerId } from "./discussions.js";
function position(text: string, bytes: string | number | bigint) {
  const all = new TextEncoder().encode(text),
    n = Number(bytes);
  if (!Number.isSafeInteger(n) || n < 0 || n > all.length) return undefined;
  try {
    return new TextDecoder("utf-8", { fatal: true }).decode(all.slice(0, n))
      .length;
  } catch {
    return undefined;
  }
}
/** Only publication-selected answer ranges remove protocol wrappers. */
export function segments(
  text: string,
  from: number,
  to: number,
  answers: Model<"CommentReplyLinkV1">[],
) {
  const ranges = [
    ...new Map(answers.map((answer) => [answerId(answer), answer])).values(),
  ]
    .flatMap((answer) => {
      const range = answer.answer.selection;
      if (!range) return [];
      const start = position(text, range.start_byte),
        end = position(text, range.end_byte);
      if (
        start === undefined ||
        end === undefined ||
        text.slice(start, end) !== answer.answer.quote
      )
        return [];
      const prefix = text
        .slice(0, start)
        .match(
          /<comment-reply discussion="[a-zA-Z0-9-]+" comments="[a-zA-Z0-9 -]+">\s*$/,
        );
      const suffix = /^\s*<\/comment-reply>/.exec(text.slice(end));
      if (!prefix || !suffix) return [];
      return [
        {
          start,
          end,
          outerStart: start - prefix[0].length,
          outerEnd: end + suffix[0].length,
          answer,
        },
      ];
    })
    .sort((a, b) => a.start - b.start);
  const result: {
    from: number;
    to: number;
    answer?: Model<"CommentReplyLinkV1">;
  }[] = [];
  let cursor = from;
  for (const range of ranges) {
    if (range.outerEnd <= from || range.outerStart >= to) continue;
    if (cursor < range.outerStart)
      result.push({ from: cursor, to: Math.min(to, range.outerStart) });
    if (range.end > from && range.start < to)
      result.push({
        from: Math.max(from, range.start),
        to: Math.min(to, range.end),
        answer: range.answer,
      });
    cursor = Math.max(cursor, range.outerEnd);
  }
  if (cursor < to) result.push({ from: cursor, to });
  return result;
}
