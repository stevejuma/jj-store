import type { Exact, Head, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";

export type StoreMutationRequest = Exact<
  operations["mutation_preview"]["requestBody"]["content"]["application/json"]
>;
export type StoreMutationCommand = StoreMutationRequest["command"];
export type StoreMutationKind = StoreMutationCommand["type"];
export type StoreMutationPrepared = Model<"Prepared">;
export type StoreMutationReceipt = Model<"Receipt">;
export type StoreMutationPreview = Model<"Preview">;
export type StoreMutationOutcome = Model<"Outcome">;
export type StoreMutationStatusResponse = Model<"Status">;

export type StoreMutationStatus =
  | "idle"
  | "previewing"
  | "prepared"
  | "applying"
  | "unknown"
  | "committed"
  | "failed";

export interface StoreMutationState {
  readonly available: readonly StoreMutationKind[];
  readonly status: StoreMutationStatus;
  readonly requestId: string | null;
  readonly command: StoreMutationCommand | null;
  readonly result: StoreMutationPrepared | null;
  readonly error: string;
}

export type MutableStoreMutationState = {
  -readonly [K in keyof StoreMutationState]: StoreMutationState[K];
};

export function initialStoreMutationState(): MutableStoreMutationState {
  return {
    available: [],
    status: "idle",
    requestId: null,
    command: null,
    result: null,
    error: "",
  };
}

export function mutationRequest(
  head: Head,
  command: StoreMutationCommand,
  requestId = randomRequestId(),
): StoreMutationRequest {
  return {
    version: 1,
    request_id: requestId,
    expected: {
      epoch: head.epoch,
      position: head.published_seq,
      code_state_version: head.code_state_version,
      heads: [...head.heads],
    },
    command,
  };
}

export function randomRequestId(): string {
  const bytes = new Uint8Array(16);
  crypto.getRandomValues(bytes);
  return [...bytes]
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");
}

export function mutationResult(
  value:
    StoreMutationPreview | StoreMutationOutcome | StoreMutationStatusResponse,
): StoreMutationPrepared | null {
  if (value.status === "prepared") return value.preview.result;
  if (value.status === "committed") return value.receipt.result;
  return null;
}

export function mutationReceipt(
  value:
    StoreMutationPreview | StoreMutationOutcome | StoreMutationStatusResponse,
): StoreMutationReceipt | null {
  return value.status === "committed" ? value.receipt : null;
}
