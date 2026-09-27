import { expect, test } from "vitest";
import { abbreviateRevisionId } from "../../../web/src/lib/ui/community-projects/revision-id.js";

test("abbreviates native identifiers with a prefix and final two characters", () => {
  expect(abbreviateRevisionId("565656b3e4a1c9")).toBe("565656...c9");
  expect(abbreviateRevisionId("ututututututut")).toBe("ututut...ut");
  expect(abbreviateRevisionId("565656b3e4a1c9", 2)).toBe("56...c9");
  expect(abbreviateRevisionId("0123456789abcdef", 8)).toBe("01234567...ef");
});

test("keeps short and absent identifiers whole without duplicating characters", () => {
  for (const value of ["", "a", "ab", "abcdef", "01234567890"])
    expect(abbreviateRevisionId(value)).toBe(value);
  expect(abbreviateRevisionId("012345678901")).toBe("012345...01");
});

test("normalizes invalid prefix lengths and preserves the requested compact bound", () => {
  const value = "0123456789abcdef";
  expect(abbreviateRevisionId(value, 3.9)).toBe("012...ef");
  expect(abbreviateRevisionId(value, 0)).toBe("0...ef");
  expect(abbreviateRevisionId(value, -1)).toBe("0...ef");
  expect(abbreviateRevisionId(value, Infinity)).toBe("012345...ef");
  expect(abbreviateRevisionId(value, NaN)).toBe("012345...ef");
  expect(abbreviateRevisionId(value, 100)).toBe(value);
});
