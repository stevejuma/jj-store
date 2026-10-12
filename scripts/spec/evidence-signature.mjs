export const signature = (value) =>
  JSON.stringify(
    Array.isArray(value)
      ? value.map((v) => JSON.parse(signature(v)))
      : value && typeof value === "object"
        ? Object.fromEntries(
            Object.keys(value)
              .sort()
              .map((k) => [k, JSON.parse(signature(value[k]))]),
          )
        : value,
  );
