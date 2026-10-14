const name = (v: unknown) =>
  typeof v === "string" && /^[a-zA-Z0-9_.-]{1,128}$/.test(v);
const path = (v: unknown) =>
  typeof v === "string" &&
  v.startsWith("/") &&
  v.length <= 4096 &&
  !/\p{Cc}/u.test(v);
function object(v: unknown): Record<string, unknown> {
  if (!v || typeof v !== "object" || Array.isArray(v))
    throw Error("Host configuration must be an object.");
  return v as Record<string, unknown>;
}
function keys(v: Record<string, unknown>, allowed: string[]) {
  if (Object.keys(v).some((k) => !allowed.includes(k)))
    throw Error("Host configuration contains unsupported fields.");
}

export function validateForgejoEnvironment(value: unknown) {
  const f = object(value);
  keys(f, [
    "program",
    "docker_program",
    "image",
    "architecture",
    "environment",
    "secrets",
  ]);
  if (
    !path(f.program) ||
    !path(f.docker_program) ||
    typeof f.image !== "string" ||
    !f.image ||
    f.image.startsWith("-") ||
    f.image.length > 1024 ||
    /\p{Cc}/u.test(f.image) ||
    !["", "linux/arm64", "linux/amd64"].includes(String(f.architecture ?? ""))
  )
    throw Error("Check Forgejo executable paths, job image and architecture.");
  const env = object(f.environment ?? {}),
    secrets = object(f.secrets ?? {});
  if (
    Object.keys(env).length + Object.keys(secrets).length > 64 ||
    Object.keys({ ...env, ...secrets }).some(
      (k) => !name(k) || k.startsWith("JJ_WORKFLOW_"),
    ) ||
    Object.values(env).some(
      (v) => typeof v !== "string" || v.length > 4096 || v.includes("\0"),
    ) ||
    Object.values(secrets).some((v) => !name(v))
  )
    throw Error("Forgejo secrets must reference operator environment names.");
}
