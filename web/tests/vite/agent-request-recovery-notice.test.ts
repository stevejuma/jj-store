import { expect, test, vi } from "vitest";
import { requestRecoveryNotice } from "../../src/lib/ui/store-agent-execution/request-recovery-notice.js";
import { NotificationService } from "../../src/lib/notifications/notification-service.svelte.js";
import { retainUnknownAgentOutcome } from "../../src/lib/controller/agent-execution-errors.js";

function fixture() {
  const owner: Parameters<typeof requestRecoveryNotice>[0] = {
    pending: {
      path: "agents/command",
      body: {
        request_id: "original",
        session_id: "session-a",
        action: { action: "prompt", text: "Retained draft" },
      },
      draft: "session-a",
    },
    outcome: {
      version: 1,
      request_id: "original",
      session_id: "session-a",
      state: "outcome_unknown",
      error: "connection_lost",
    },
    busy: false,
    error: "Connection lost",
    retainAsUnresolved: vi.fn(() => retainUnknownAgentOutcome(owner)),
    retryRetainedRequest: vi.fn(async () => {}),
    openSession: vi.fn(),
  };
  return owner;
}

test("review retains the unknown outcome and opens its original session without retrying", () => {
  const owner = fixture();
  const receipt = owner.outcome;
  const notice = requestRecoveryNotice(owner)!;
  expect(notice.message).toContain("Connection lost");
  notice.action!.run();
  expect(owner.pending).toBeNull();
  expect(owner.outcome).toBe(receipt);
  expect(owner.openSession).toHaveBeenCalledWith("session-a");
  expect(owner.retryRetainedRequest).not.toHaveBeenCalled();
  notice.action!.run();
  expect(owner.retainAsUnresolved).toHaveBeenCalledTimes(1);
});

test("recovery actions refuse stale requests, changed outcomes and busy dispatch", () => {
  const owner = fixture();
  const notice = requestRecoveryNotice(owner)!;
  const original = owner.pending;
  owner.busy = true;
  notice.action!.run();
  owner.busy = false;
  owner.pending = {
    ...original!,
    body: { ...original!.body, request_id: "new" },
  };
  expect(requestRecoveryNotice(owner)).toBeUndefined();
  notice.action!.run();
  owner.pending = original;
  owner.outcome!.state = "not_known";
  notice.action!.run();
  expect(owner.retainAsUnresolved).not.toHaveBeenCalled();
  expect(owner.openSession).not.toHaveBeenCalled();
  const retry = requestRecoveryNotice(owner)!;
  expect(retry.action!.label).toBe("Submit retained request");
  retry.action!.run();
  expect(owner.retryRetainedRequest).toHaveBeenCalledTimes(1);
  expect(owner.pending).toBe(original);
});

test("dismissal survives refresh and navigation while a different request creates a new notice", () => {
  const owner = fixture();
  const service = new NotificationService();
  service.publish(requestRecoveryNotice(owner)!);
  const id = service.records[0]!.id;
  service.dismiss(id);
  service.leaveScope("agent-execution");
  service.publish(requestRecoveryNotice(owner)!);
  expect(service.records).toHaveLength(0);
  expect(owner.pending!.body.request_id).toBe("original");
  expect(owner.retainAsUnresolved).not.toHaveBeenCalled();
  expect(owner.retryRetainedRequest).not.toHaveBeenCalled();
  owner.pending = {
    ...owner.pending!,
    body: { ...owner.pending!.body, request_id: "next" },
  };
  owner.outcome = { ...owner.outcome!, request_id: "next" };
  service.publish(requestRecoveryNotice(owner)!);
  expect(service.records).toHaveLength(1);
  service.resolve("agent-execution", "next");
  expect(service.records).toHaveLength(0);
  service.dispose();
});
