import document from "./schema.json" with { type: "json" };

type Schema =
  | boolean
  | {
      $ref?: string;
      type?: string | string[];
      const?: unknown;
      enum?: unknown[];
      oneOf?: Schema[];
      anyOf?: Schema[];
      properties?: Record<string, Schema>;
      required?: string[];
      additionalProperties?: Schema;
      patternProperties?: Record<string, Schema>;
      items?: Schema;
      prefixItems?: Schema[];
      minItems?: number;
      maxItems?: number;
      minimum?: number;
      maximum?: number;
      format?: string;
      pattern?: string;
      minLength?: number;
      maxLength?: number;
    };
const definitions = document.$defs as unknown as Record<string, Schema>;
const integerRanges: Record<string, [bigint, bigint]> = {
  uint8: [0n, 255n],
  uint32: [0n, 4294967295n],
  int32: [-2147483648n, 2147483647n],
  uint: [0n, 18446744073709551615n],
  uint64: [0n, 18446744073709551615n],
  int64: [-9223372036854775808n, 9223372036854775807n],
};
function matches(s: Schema, value: unknown, depth: number): boolean {
  if (depth > 128) return false;
  if (typeof s === "boolean") return s;
  if (s.$ref) {
    const target = definitions[s.$ref.replace("#/$defs/", "")];
    const { $ref, ...siblings } = s;
    return (
      target !== undefined &&
      matches(target, value, depth + 1) &&
      (Object.keys(siblings).every((key) =>
        ["title", "description", "default"].includes(key),
      ) ||
        matches(siblings, value, depth + 1))
    );
  }
  if ("const" in s) return value === s.const;
  if (s.enum) return s.enum.includes(value);
  if (s.oneOf)
    return s.oneOf.filter((v) => matches(v, value, depth + 1)).length === 1;
  if (s.anyOf) return s.anyOf.some((v) => matches(v, value, depth + 1));
  if (Array.isArray(s.type))
    return s.type.some((type) => matches({ ...s, type }, value, depth + 1));
  switch (s.type) {
    case "null":
      return value === null;
    case "boolean":
      return typeof value === "boolean";
    case "string": {
      if (typeof value !== "string") return false;
      const length = Array.from(value).length;
      return (
        (s.minLength === undefined || length >= s.minLength) &&
        (s.maxLength === undefined || length <= s.maxLength) &&
        (!s.pattern || new RegExp(s.pattern, "u").test(value))
      );
    }
    case "integer": {
      if (
        typeof value !== "bigint" &&
        (typeof value !== "number" || !Number.isSafeInteger(value))
      )
        return false;
      if (
        typeof value === "bigint" &&
        !["uint64", "int64", "uint"].includes(s.format ?? "")
      )
        return false;
      const n = BigInt(value);
      const range = integerRanges[s.format ?? ""];
      return (
        (!range || (n >= range[0] && n <= range[1])) &&
        (s.minimum === undefined || n >= BigInt(s.minimum)) &&
        (s.maximum === undefined || n <= BigInt(s.maximum))
      );
    }
    case "array": {
      if (
        !Array.isArray(value) ||
        (s.minItems !== undefined && value.length < s.minItems) ||
        (s.maxItems !== undefined && value.length > s.maxItems)
      )
        return false;
      return value.every((v, i) =>
        matches(s.prefixItems?.[i] ?? s.items ?? true, v, depth + 1),
      );
    }
    case "object": {
      if (value === null || typeof value !== "object" || Array.isArray(value))
        return false;
      const object = value as Record<string, unknown>;
      if (s.required?.some((key) => !Object.hasOwn(object, key))) return false;
      return Object.entries(object).every(([key, v]) => {
        const own =
          s.properties && Object.hasOwn(s.properties, key)
            ? s.properties[key]
            : undefined;
        const patterns = Object.entries(s.patternProperties ?? {}).filter(
          ([p]) => new RegExp(p, "u").test(key),
        );
        if (own !== undefined && !matches(own, v, depth + 1)) return false;
        if (!patterns.every(([, schema]) => matches(schema, v, depth + 1)))
          return false;
        return (
          own !== undefined ||
          patterns.length > 0 ||
          matches(s.additionalProperties ?? true, v, depth + 1)
        );
      });
    }
    default:
      return false;
  }
}
export function validFrame(
  name: keyof typeof document.properties,
  value: unknown,
): boolean {
  return matches(document.properties[name], value, 0);
}
