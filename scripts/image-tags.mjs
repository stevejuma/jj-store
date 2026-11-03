const sourcePattern = /^[a-f0-9]{40}$/;
const architecturePattern = /^(amd64|arm64)$/;

export function shortSource(source) {
  if (!sourcePattern.test(source)) throw Error("invalid image source identity");
  return source.slice(0, 12);
}

export function sourceImageTag(source) {
  return `sha-${shortSource(source)}`;
}

export function sourceArchitectureTag(source, architecture) {
  if (!architecturePattern.test(architecture))
    throw Error("unsupported image architecture");
  return `${sourceImageTag(source)}-${architecture}`;
}

export function releaseArchitectureTag(source, architecture) {
  return `release-${sourceArchitectureTag(source, architecture)}`;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const [command, source, architecture] = process.argv.slice(2);
  if (command === "source") console.log(sourceImageTag(source));
  else if (command === "architecture")
    console.log(sourceArchitectureTag(source, architecture));
  else if (command === "release-architecture")
    console.log(releaseArchitectureTag(source, architecture));
  else throw Error("use source, architecture or release-architecture");
}
