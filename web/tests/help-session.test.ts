import { test } from "node:test";
import assert from "node:assert/strict";
import { admittedHelpMessage } from "../src/lib/ui/store-help/help-session.js";
test("Help only accepts its exact same-origin frame message protocol", () => {
  const frame = {} as Window;
  const event = {
    source: frame,
    origin: "http://localhost:7000",
    data: { type: "jj-store-help-close" },
  } as MessageEvent;
  assert.equal(admittedHelpMessage(event, frame, event.origin), true);
  for (const data of [
    { type: "jj-store-help-loading" },
    { type: "jj-store-help-ready" },
    { type: "jj-store-help-error", message: "Documentation unavailable" },
  ]) {
    assert.equal(
      admittedHelpMessage(
        { ...event, data } as MessageEvent,
        frame,
        event.origin,
      ),
      true,
    );
    assert.equal(
      admittedHelpMessage(
        { ...event, data } as MessageEvent,
        {} as Window,
        event.origin,
      ),
      false,
    );
  }
  for (const message of [null, 123, "", "x".repeat(501)]) {
    assert.equal(
      admittedHelpMessage(
        {
          ...event,
          data: { type: "jj-store-help-error", message },
        } as MessageEvent,
        frame,
        event.origin,
      ),
      false,
    );
  }
  assert.equal(admittedHelpMessage(event, {} as Window, event.origin), false);
  assert.equal(
    admittedHelpMessage(event, frame, "https://elsewhere.test"),
    false,
  );
  assert.equal(
    admittedHelpMessage(
      { ...event, data: null } as MessageEvent,
      frame,
      event.origin,
    ),
    false,
  );
  assert.equal(
    admittedHelpMessage(
      { ...event, data: { type: "mutation" } } as MessageEvent,
      frame,
      event.origin,
    ),
    false,
  );
});
