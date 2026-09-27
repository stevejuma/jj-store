import type { StorePage } from "./community-navigation/model.js";
import type { AppShellSidebarLayout } from "@lapismd/design-core/shell";

const layoutNamespace = "jj-store/layout/v1";

export const repositoryShellStorageKey = `${layoutNamespace}/shell`;
export const repositoryFileTreeStorageKey = `${layoutNamespace}/file-tree`;
export const repositoryFileTreeDefaultWidth = 256;
export const repositoryFileTreeMinimumWidth = 208;
export const repositoryFileTreeMaximumWidth = 420;

export interface RepositoryFileTreeLayout {
  open: boolean;
  width: number;
}

export interface RepositoryShellLayout {
  left?: AppShellSidebarLayout;
  right?: AppShellSidebarLayout;
}

export function repositoryColumnCanvasStorageKey(page: StorePage): string {
  return `${layoutNamespace}/column-canvas/${page}`;
}

export function resolveLayoutStorage(storage?: Storage): Storage | undefined {
  if (storage) return storage;
  try {
    return typeof window === "undefined" ? undefined : window.localStorage;
  } catch {
    return undefined;
  }
}

export function loadRepositoryShellLayout(
  storage: Storage | undefined,
): RepositoryShellLayout {
  if (!storage) return {};
  try {
    const serialized = storage.getItem(repositoryShellStorageKey);
    if (!serialized) return {};
    const value = JSON.parse(serialized) as unknown;
    if (!isRecord(value) || value.version !== 1 || !isRecord(value.panels)) {
      return {};
    }
    const left = readSidebarLayout(value.panels.left, "left");
    const right = readSidebarLayout(value.panels.right, "right");
    return {
      ...(left ? { left } : {}),
      ...(right ? { right } : {}),
    };
  } catch {
    return {};
  }
}

export function loadRepositoryFileTreeLayout(
  storage: Storage | undefined,
): RepositoryFileTreeLayout {
  if (!storage) return defaultFileTreeLayout();
  try {
    const serialized = storage.getItem(repositoryFileTreeStorageKey);
    if (!serialized) return defaultFileTreeLayout();
    const value = JSON.parse(serialized) as unknown;
    if (
      !isRecord(value) ||
      value.version !== 1 ||
      typeof value.open !== "boolean" ||
      typeof value.width !== "number" ||
      !Number.isFinite(value.width)
    ) {
      return defaultFileTreeLayout();
    }
    return {
      open: value.open,
      width: clampFileTreeWidth(value.width),
    };
  } catch {
    return defaultFileTreeLayout();
  }
}

export function saveRepositoryFileTreeLayout(
  storage: Storage | undefined,
  layout: RepositoryFileTreeLayout,
): void {
  if (!storage) return;
  try {
    storage.setItem(
      repositoryFileTreeStorageKey,
      JSON.stringify({
        version: 1,
        open: layout.open,
        width: clampFileTreeWidth(layout.width),
      }),
    );
  } catch {
    // Browsing remains available when storage is disabled or quota is full.
  }
}

export function clampFileTreeWidth(width: number): number {
  const finite = Number.isFinite(width)
    ? width
    : repositoryFileTreeDefaultWidth;
  return Math.min(
    repositoryFileTreeMaximumWidth,
    Math.max(repositoryFileTreeMinimumWidth, Math.round(finite)),
  );
}

function defaultFileTreeLayout(): RepositoryFileTreeLayout {
  return { open: true, width: repositoryFileTreeDefaultWidth };
}

function readSidebarLayout(
  value: unknown,
  side: "left" | "right",
): AppShellSidebarLayout | undefined {
  if (
    !isRecord(value) ||
    value.side !== side ||
    typeof value.collapsed !== "boolean" ||
    typeof value.closed !== "boolean"
  ) {
    return undefined;
  }
  return {
    side,
    collapsed: value.collapsed,
    closed: value.closed,
    ...(typeof value.width === "number" && Number.isFinite(value.width)
      ? { width: value.width }
      : {}),
  };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
