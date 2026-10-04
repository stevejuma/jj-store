export function historicalTaskMapping(source, tasks, errors) {
  const rows = [
    ...source.matchAll(
      /^\|\s*(E-[A-Z0-9-]+\*)\s*\|\s*(M\d+(?:\.\d+)?)\s*\|\s*(M\d+(?:\.\d+)?)\s*\|\s*$/gm,
    ),
  ];
  const mappings = new Map();
  for (const [, family, original, current] of rows) {
    const key = `${family}:${original}`;
    if (mappings.has(key) || !tasks.has(current) || original === current) {
      errors.push(`invalid historical task mapping ${key} -> ${current}`);
    }
    mappings.set(key, current);
  }
  return (entry) =>
    (entry.tasks ?? []).map((task) => {
      const matches = [...mappings].filter(([key]) => {
        const [family, original] = key.split(":");
        return original === task && entry.id?.startsWith(family.slice(0, -1));
      });
      if (matches.length > 1)
        errors.push(`ambiguous historical task mapping ${entry.id}:${task}`);
      return matches[0]?.[1] ?? task;
    });
}
