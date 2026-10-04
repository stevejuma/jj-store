import { ApiError } from "../api/client.js";

export function requireCapture(valid: unknown): asserts valid {
  if (!valid) throw new ApiError("unexpected_capture_response");
}
export const object = (v: unknown): v is Record<string, unknown> =>
  !!v && typeof v === "object" && !Array.isArray(v);
export const bool = (v: unknown) => typeof v === "boolean";
export const uint = (v: unknown, max = 18446744073709551615n) =>
  (typeof v === "bigint" ||
    (typeof v === "number" && Number.isSafeInteger(v))) &&
  BigInt(v) >= 0n &&
  BigInt(v) <= max;
export const count = (v: unknown) => uint(v, 4096n);
