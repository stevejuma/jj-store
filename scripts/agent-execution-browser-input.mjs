export function validateAgentAcceptanceOrigin(origin, repository) {
  if (
    !/^http:\/\/127\.0\.0\.1:\d+$/.test(origin ?? "") ||
    !/^[a-f0-9]{32}$/.test(repository ?? "")
  )
    throw Error("explicit disposable native origin and repository required");
}
