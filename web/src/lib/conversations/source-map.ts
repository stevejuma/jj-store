import type { Model } from "../api/client.js";
import type {
  ChatBodyBlock,
  ChatCommentAnchor,
} from "../ui/store-agent-chat-body/model.js";
import { sha256 } from "@noble/hashes/sha2.js";
import { bytesToHex } from "@noble/hashes/utils.js";

export type ConversationSource = Model<"ConversationSourceV1">;
export function sourceTargetKey(value: unknown): string {
  return JSON.stringify(value, (_key, v) =>
    v && !Array.isArray(v) && typeof v === "object"
      ? Object.fromEntries(
          Object.entries(v).sort(([a], [b]) => a.localeCompare(b)),
        )
      : v,
  );
}
/** Captured text is immutable; a rendered block must prove its original slice. */
export interface ConversationTextSource {
  target: ConversationSource["target"];
  part: string | null;
  text: string;
  /** Original UTF-16 offset represented by block offset zero. */
  from: number;
  revision: string;
  wholeSection?: boolean;
}
function boundary(text: string, offset: number) {
  if (!Number.isInteger(offset) || offset < 0 || offset > text.length)
    return false;
  const previous = text.charCodeAt(offset - 1),
    next = text.charCodeAt(offset);
  return !(
    previous >= 0xd800 &&
    previous <= 0xdbff &&
    next >= 0xdc00 &&
    next <= 0xdfff
  );
}
export function exactSource(
  block: ChatBodyBlock,
  anchor: ChatCommentAnchor,
  source: ConversationTextSource,
): ConversationSource {
  if (
    source.wholeSection &&
    block.id === anchor.blockId &&
    block.revision === anchor.revision &&
    block.revision === source.revision
  )
    return { target: source.target, part: null, selection: null, quote: "" };
  if (
    block.id !== anchor.blockId ||
    block.revision !== anchor.revision ||
    block.revision !== source.revision ||
    block.text !==
      source.text.slice(source.from, source.from + block.text.length) ||
    block.text.slice(anchor.from, anchor.to) !== anchor.quote ||
    anchor.from >= anchor.to ||
    !boundary(block.text, anchor.from) ||
    !boundary(block.text, anchor.to) ||
    !boundary(source.text, source.from)
  )
    throw new Error(
      "The exact published source is unavailable. Your comment is retained.",
    );
  const encode = (s: string) => new TextEncoder().encode(s);
  const from = source.from + anchor.from,
    to = source.from + anchor.to;
  return {
    target: source.target,
    part: source.part,
    selection: {
      encoding: "utf8",
      start_byte: String(encode(source.text.slice(0, from)).length),
      end_byte: String(encode(source.text.slice(0, to)).length),
      selected_sha256: bytesToHex(sha256(encode(anchor.quote))),
    },
    quote: anchor.quote,
  };
}
export function sourceAnchor(
  block: ChatBodyBlock,
  source: ConversationTextSource,
  selector: ConversationSource,
): ChatCommentAnchor | undefined {
  if (sourceTargetKey(selector.target) !== sourceTargetKey(source.target))
    return;
  if (!selector.selection)
    return {
      blockId: block.id,
      revision: block.revision,
      from: 0,
      to: block.text.length,
      quote: block.text,
    };
  if (selector.part !== source.part) return;
  const bytes = new TextEncoder().encode(source.text),
    range = selector.selection;
  const fromByte = Number(range.start_byte),
    toByte = Number(range.end_byte);
  if (
    !Number.isSafeInteger(fromByte) ||
    !Number.isSafeInteger(toByte) ||
    fromByte < 0 ||
    toByte > bytes.length ||
    fromByte >= toByte
  )
    return;
  try {
    const decoder = new TextDecoder("utf-8", { fatal: true });
    const from = decoder.decode(bytes.slice(0, fromByte)).length - source.from;
    const to = decoder.decode(bytes.slice(0, toByte)).length - source.from;
    const anchor = {
      blockId: block.id,
      revision: block.revision,
      from,
      to,
      quote: selector.quote,
    };
    const actual = exactSource(block, anchor, source);
    return actual.selection?.selected_sha256 === range.selected_sha256
      ? anchor
      : undefined;
  } catch {
    return;
  }
}
