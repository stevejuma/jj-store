/** Delta exports byte positions; CodeMirror and JS strings use UTF-16. */
export function utf8ToUtf16(text: string, byteOffset: number): number {
  const bytes = new TextEncoder().encode(text);
  if (
    !Number.isInteger(byteOffset) ||
    byteOffset < 0 ||
    byteOffset > bytes.length
  )
    throw new RangeError("Delta offset is outside the captured buffer");
  return new TextDecoder("utf-8", { fatal: true }).decode(
    bytes.slice(0, byteOffset),
  ).length;
}
export function byteSlice(text: string, start: number, end: number) {
  return text.slice(utf8ToUtf16(text, start), utf8ToUtf16(text, end));
}
