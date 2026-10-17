import { stringify } from "lossless-json";
import { parseFrame } from "./parse.js";
import type { Contract } from "./generated.js";
import { validFrame } from "./validate.js";
export const VERSION = 1;
export const MAX_REQUEST_BYTES = 128 * 1024;
export const MAX_RESPONSE_BYTES = 16 * 1024 * 1024;
const bytes = (text: string) => new TextEncoder().encode(text).byteLength;
export class HistoryProtocolError extends Error {
  constructor() {
    super("Invalid or oversized history frame");
    this.name = "HistoryProtocolError";
  }
}
export function encode<K extends keyof Contract>(
  kind: K,
  data: Contract[K]["data"],
): string {
  try {
    const value = { version: VERSION, data };
    if (!validFrame(kind, value)) throw new HistoryProtocolError();
    const text = stringify(value);
    if (typeof text !== "string" || bytes(text) > MAX_REQUEST_BYTES)
      throw new HistoryProtocolError();
    decode(kind, text);
    return text;
  } catch {
    throw new HistoryProtocolError();
  }
}
export function decode<K extends keyof Contract>(
  kind: K,
  text: string,
): Contract[K]["data"] {
  try {
    if (typeof text !== "string" || bytes(text) > MAX_RESPONSE_BYTES)
      throw new HistoryProtocolError();
    const frame = parseFrame(text);
    if (!validFrame(kind, frame)) throw new HistoryProtocolError();
    return (frame as Contract[K]).data;
  } catch {
    throw new HistoryProtocolError();
  }
}
