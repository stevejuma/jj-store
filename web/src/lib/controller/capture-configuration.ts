import type { operations } from "../api/generated.js";
import type { Exact } from "../api/client.js";
import type { Model } from "../api/client.js";
import { pretty } from "../api/client.js";
import { requireCapture } from "./capture-validation.js";
export const configurableProfiles = [
  "codex-cli-jsonl-v1",
  "codex-desktop-jsonl-v1",
] as const;
export type CaptureSaveSettings = Exact<
  NonNullable<
    operations["local_capture_save_settings"]["requestBody"]
  >["content"]["application/json"]
>;
export type CaptureDraft = {
  enabled: boolean;
  cli: boolean;
  desktop: boolean;
  cliRoots: string;
  desktopRoots: string;
  readers: string;
  interval: string;
  sources: string;
  bytes: string;
};
export type CaptureSettingsState = {
  value: Model<"LocalCaptureSettings"> | null;
  draft: CaptureDraft | null;
  dirty: boolean;
  saving: boolean;
  baseRevision: string;
  conflict: boolean;
  unknownObserved: boolean;
  validation: string;
  error: string;
  message: string;
  unknown: CaptureSaveSettings | null;
};
export const emptyCaptureSettings = (): CaptureSettingsState => ({
  value: null,
  draft: null,
  dirty: false,
  saving: false,
  baseRevision: "",
  conflict: false,
  unknownObserved: false,
  validation: "",
  error: "",
  message: "",
  unknown: null,
});
export function captureConfigDraft(
  c: Model<"CaptureConfiguration">,
): CaptureDraft {
  const roots = (profile: string) =>
    c.harnesses.find((h) => h.profile === profile)?.roots.join("\n") ?? "";
  return {
    enabled: c.enabled,
    cli: c.harnesses.some((h) => h.profile === configurableProfiles[0]),
    desktop: c.harnesses.some((h) => h.profile === configurableProfiles[1]),
    cliRoots: roots(configurableProfiles[0]),
    desktopRoots: roots(configurableProfiles[1]),
    readers: String(c.max_readers),
    interval: String(c.rescan_seconds),
    sources: String(c.max_sources),
    bytes: String(c.max_pending_bytes),
  };
}
function roots(text: string) {
  const values = text
    .split("\n")
    .map((s) => s.trim())
    .filter(Boolean);
  if (
    values.length > 16 ||
    new Set(values).size !== values.length ||
    values.some(
      (s) =>
        !s.startsWith("/") ||
        s === "/" ||
        new TextEncoder().encode(s).length > 4096 ||
        /[\u0000-\u001f\u007f]/.test(s) ||
        s.split("/").some((c) => c === "." || c === ".."),
    )
  )
    throw new Error(
      "Use up to 16 distinct absolute folder paths per harness, one per line.",
    );
  return values;
}
export function captureConfigValue(
  d: CaptureDraft,
  defaults: Model<"CaptureHarnessSetting">[],
): Model<"CaptureConfiguration"> {
  const integer = (v: string, max: number, label: string) => {
    const n = Number(v);
    if (!/^\d+$/.test(v) || !Number.isSafeInteger(n) || n < 1 || n > max)
      throw new Error(
        `${label} must be a whole number between 1 and ${max.toLocaleString("en-GB")}.`,
      );
    return n;
  };
  if (d.enabled && !d.cli && !d.desktop)
    throw new Error("Select at least one harness before enabling capture.");
  return {
    enabled: d.enabled,
    max_readers: integer(d.readers, 4, "Readers"),
    rescan_seconds: integer(d.interval, 30, "Discovery interval"),
    max_sources: integer(d.sources, 4096, "Tracked sources"),
    max_pending_bytes: integer(d.bytes, 67108864, "Pending byte budget"),
    harnesses: configurableProfiles.flatMap((profile, i) => {
      if (!(i === 0 ? d.cli : d.desktop)) return [];
      const version = defaults.find(
        (h) => h.profile === profile,
      )?.product_version;
      if (!version)
        throw new Error("Harness defaults are unavailable. Refresh settings.");
      return [
        {
          profile,
          product_version: version,
          roots: roots(i === 0 ? d.cliRoots : d.desktopRoots),
        },
      ];
    }),
  };
}
export const sameCaptureConfiguration = (
  a: Model<"CaptureConfiguration">,
  b: Model<"CaptureConfiguration">,
) =>
  pretty(captureConfigDraft(a)) === pretty(captureConfigDraft(b)) &&
  a.harnesses.every((h) =>
    b.harnesses.some(
      (x) => x.profile === h.profile && x.product_version === h.product_version,
    ),
  );
export function validateCaptureSettings(v: Model<"LocalCaptureSettings">) {
  requireCapture(
    v?.version === 1 &&
      /^[a-f0-9]{64}$/.test(v.revision) &&
      typeof v.restart_required === "boolean" &&
      Array.isArray(v.defaults) &&
      v.defaults.length === 2,
  );
  for (const c of [v.desired, v.effective]) {
    requireCapture(
      c &&
        typeof c.enabled === "boolean" &&
        Array.isArray(c.harnesses) &&
        c.harnesses.length <= 2 &&
        new Set(c.harnesses.map((h) => h.profile)).size === c.harnesses.length,
    );
    for (const h of c.harnesses)
      requireCapture(
        configurableProfiles.includes(
          h.profile as (typeof configurableProfiles)[number],
        ) &&
          Array.isArray(h.roots) &&
          h.roots.every((s) => typeof s === "string") &&
          h.product_version ===
            v.defaults.find((x) => x.profile === h.profile)?.product_version,
      );
    const parsed = captureConfigValue(captureConfigDraft(c), v.defaults);
    requireCapture(sameCaptureConfiguration(parsed, c));
  }
  requireCapture(new Set(v.defaults.map((h) => h.profile)).size === 2);
  for (const h of v.defaults) {
    requireCapture(
      configurableProfiles.includes(
        h.profile as (typeof configurableProfiles)[number],
      ) &&
        typeof h.product_version === "string" &&
        h.product_version.length > 0 &&
        h.product_version.length <= 32 &&
        Array.isArray(h.roots) &&
        h.roots.every((s) => typeof s === "string"),
    );
    roots(h.roots.join("\n"));
  }
  requireCapture(
    v.restart_required === !sameCaptureConfiguration(v.desired, v.effective),
  );
}
