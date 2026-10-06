export function archivedSource(source, read) {
  return source.parts ? source.parts.map(read).join("") : read(source.archive);
}
