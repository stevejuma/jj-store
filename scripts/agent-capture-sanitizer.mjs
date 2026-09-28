import { createNodeRedaction } from "@lapismd/agent-redaction/node";

const integrations = new Set(["codex", "cursor", "opencode"]);
const sourceKinds = new Set([
  "hook",
  "plugin",
  "native_api",
  "session_export",
  "transcript",
  "controller",
  "manual",
  "workspace_observation",
]);

const omitted = Object.freeze({ status: "omitted", reason: "invalid_input" });

function isRecord(value) {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return false;
  }
  const prototype = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

function isLabel(value, maximum) {
  return (
    typeof value === "string" &&
    value.length > 0 &&
    value.length <= maximum &&
    !/[\u0000-\u001f\u007f]/u.test(value)
  );
}

function captureReady(value, summary) {
  if (
    !isRecord(value) ||
    !integrations.has(value.integration) ||
    !sourceKinds.has(value.sourceKind) ||
    !isLabel(value.sourceInstanceId, 256) ||
    !Number.isSafeInteger(value.sourceSequence) ||
    value.sourceSequence < 0 ||
    !Object.hasOwn(value, "payload") ||
    (Object.hasOwn(value, "occurredAtMillis") &&
      (!Number.isSafeInteger(value.occurredAtMillis) ||
        value.occurredAtMillis < 0))
  ) {
    return omitted;
  }

  const observation = {
    integration: value.integration,
    sourceKind: value.sourceKind,
    sourceInstanceId: value.sourceInstanceId,
    sourceSequence: value.sourceSequence,
    payload: value.payload,
    sanitizerVersion: summary.policy,
    sanitizationSummary: summary,
  };
  if (Object.hasOwn(value, "occurredAtMillis")) {
    observation.occurredAtMillis = value.occurredAtMillis;
  }
  return Object.freeze({
    status: "sanitized",
    value: Object.freeze(observation),
  });
}

export async function sanitizeCapturedObservation(input, policy = {}) {
  let runtime;
  try {
    runtime = createNodeRedaction(policy);
    const result = await runtime.scope.value(input);
    if (result.status === "omitted") return result;
    return captureReady(result.value, result.summary);
  } catch {
    return omitted;
  } finally {
    runtime?.dispose();
  }
}
