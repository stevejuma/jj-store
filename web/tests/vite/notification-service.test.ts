import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  NotificationService,
  type NotificationEvent,
  type NotificationInput,
} from "../../src/lib/notifications/notification-service.svelte.js";

describe("application notification lifetime", () => {
  let service: NotificationService;
  const notice = (
    patch: Partial<NotificationInput> = {},
  ): NotificationInput => ({
    scope: "repository-a",
    key: "search",
    title: "Search unavailable",
    message: "Try again.",
    ...patch,
  });
  beforeEach(() => {
    vi.useFakeTimers();
    service = new NotificationService();
  });
  afterEach(() => {
    service.dispose();
    vi.useRealTimers();
  });

  it("updates one condition and suppresses a dismissed episode until recovery", () => {
    const events: NotificationEvent[] = [];
    service.subscribe((event) => events.push(event));
    service.publish(notice());
    service.publish(notice());
    expect(service.records).toHaveLength(1);
    expect(events).toHaveLength(1);
    service.dismiss(service.records[0]!.id);
    service.publish(notice());
    expect(service.records).toHaveLength(0);
    service.publish(notice({ message: "" }));
    service.publish(notice());
    expect(service.records).toHaveLength(1);
    service.publish(notice({ message: "Repository is offline." }));
    expect(service.records).toHaveLength(1);
    expect(service.records[0]!.message).toBe("Repository is offline.");
  });

  it("retires obsolete conditions while retaining sticky outcomes across navigation", () => {
    service.publish(notice());
    service.publish(notice({ key: "failure", lifetime: "dismiss" }));
    service.leaveScope("repository-a");
    expect(service.records.map((item) => item.key)).toEqual(["failure"]);
    service.publish(notice());
    const condition = service.records.find((item) => item.key === "search")!;
    service.dismiss(condition.id);
    service.leaveScope("repository-a");
    service.publish(notice());
    expect(service.records.map((item) => item.key)).toEqual(["failure"]);
  });

  it("times information once and pauses the remaining duration during interaction", () => {
    const input = notice({ severity: "info", lifetime: "timed" });
    service.publish(input);
    const id = service.records[0]!.id;
    vi.advanceTimersByTime(1500);
    service.publish(input);
    service.pause(id);
    vi.advanceTimersByTime(20000);
    expect(service.records).toHaveLength(1);
    service.resume(id);
    vi.advanceTimersByTime(2499);
    expect(service.records).toHaveLength(1);
    vi.advanceTimersByTime(1);
    expect(service.records).toHaveLength(0);
    service.publish(input);
    expect(service.records).toHaveLength(0);
  });

  it("keeps warnings and errors until resolved or explicitly dismissed", () => {
    service.publish(notice({ severity: "warning" }));
    service.publish(
      notice({ key: "write", severity: "error", lifetime: "dismiss" }),
    );
    vi.advanceTimersByTime(60000);
    expect(service.records).toHaveLength(2);
    service.resolve("repository-a", "search");
    expect(service.records.map((item) => item.key)).toEqual(["write"]);
  });

  it("keeps actions local and never executes a dismissed or stale action", () => {
    const run = vi.fn();
    const events: NotificationEvent[] = [];
    service.subscribe((event) => events.push(event));
    service.publish(notice({ action: { id: "retry", label: "Retry", run } }));
    const id = service.records[0]!.id;
    service.invoke(id, "wrong");
    service.invoke(id, "retry");
    service.dismiss(id);
    service.invoke(id, "retry");
    expect(run).toHaveBeenCalledTimes(1);
    expect(JSON.parse(JSON.stringify(events[0]))).toEqual(events[0]);
    expect(events[0]!.notification.action).toEqual({
      id: "retry",
      label: "Retry",
    });
  });

  it("isolates owners and clears records, callbacks and timers on reset or disposal", () => {
    const other = new NotificationService();
    const run = vi.fn();
    service.publish(
      notice({
        lifetime: "timed",
        action: { id: "retry", label: "Retry", run },
      }),
    );
    const id = service.records[0]!.id;
    other.publish(notice());
    service.reset();
    service.invoke(id, "retry");
    expect(run).not.toHaveBeenCalled();
    expect(service.records).toHaveLength(0);
    expect(other.records).toHaveLength(1);
    service.dispose();
    service.publish(notice());
    expect(service.records).toHaveLength(0);
    expect(vi.getTimerCount()).toBe(0);
    other.dispose();
  });
});
