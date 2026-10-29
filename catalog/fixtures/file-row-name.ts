/** Match the exact file identity while retaining the shared listing's visible name. */
export function fileRowName(path: string) {
  return (name: string, element: Element | null) =>
    name === path ||
    (element?.getAttribute("data-file-path") === path &&
      name.includes(path.split("/").at(-1)!));
}

/** Retain a directory's exact path when the listing adds visible change counts. */
export function directoryRowName(path: string) {
  return (name: string, element: Element | null) =>
    element?.getAttribute("data-directory-path") === path &&
    name.includes(path.split("/").at(-1)!);
}
