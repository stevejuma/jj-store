import {
  requireCapture,
  object,
  bool,
  uint,
  count,
} from "./capture-response-validation.js";

export function validateSnapshotPublication(v: unknown) {
  requireCapture(object(v) && Object.keys(v).length === 7);
  requireCapture(
    bool(v.history_available) &&
      uint(v.captured_sequence) &&
      uint(v.captured_logical_records) &&
      uint(v.committed_sequence) &&
      count(v.outstanding_ranges) &&
      uint(v.receipts) &&
      (v.committed_logical_records === null ||
        uint(v.committed_logical_records)),
  );
  const committed = BigInt(v.committed_sequence as number | bigint);
  requireCapture(committed <= BigInt(v.captured_sequence as number | bigint));
  requireCapture(
    committed === 0n
      ? v.committed_logical_records === null
      : v.history_available &&
          v.committed_logical_records !== null &&
          BigInt(v.receipts as number | bigint) > 0n,
  );
}
