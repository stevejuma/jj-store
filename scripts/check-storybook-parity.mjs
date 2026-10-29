import { readFileSync } from "node:fs";
import { resolve, posix } from "node:path";
import {
  walk,
  value,
  attr,
  storyModel,
  resolveDefaultComponent,
} from "./storybook-parity/model.mjs";
export { storyModel } from "./storybook-parity/model.mjs";
export const name = "storybook-parity";
const host = "catalog/fixtures/ApplicationStory.svelte";
const production =
  "web/src/lib/ui/store-application/StoreApplicationRoot.svelte";
export function parityFindings({
  root,
  files,
  read = (file) => readFileSync(resolve(root, file), "utf8"),
}) {
  const findings = [],
    known = new Set(files),
    cache = new Map();
  const add = (file, message, node = { start: 0 }) =>
    findings.push({
      code: "STORYBOOK-PARITY",
      rule: "JST-UI-001",
      file,
      line: read(file).slice(0, node.start).split("\n").length,
      message,
    });
  const load = (file) => {
    if (!cache.has(file)) cache.set(file, storyModel(read(file), file));
    return cache.get(file);
  };
  function target(file, reference) {
    if (!reference?.startsWith(".")) return null;
    const path = posix.normalize(posix.join(posix.dirname(file), reference));
    return (
      [path, path.replace(/\.js$/, ".ts"), path + "/index.ts"].find(
        (p) => known.has(p) || p === host || p === production,
      ) ?? null
    );
  }
  const resolveExport = (file) => resolveDefaultComponent(file, target, read);
  function composition(file, props = {}, chain = []) {
    if (file === host) return { root: true, errors: [] };
    if (!file || !file.startsWith("catalog/"))
      return {
        root: false,
        errors: ["Application composition does not reach ApplicationStory."],
      };
    if (chain.includes(file))
      return {
        root: false,
        errors: ["Cyclic application fixture composition."],
      };
    const model = load(file),
      env = {};
    for (const statement of model.ast.instance?.content.body ?? [])
      if (statement.type === "VariableDeclaration")
        for (const d of statement.declarations) {
          if (d.id.type === "Identifier") env[d.id.name] = value(d.init, env);
          if (
            d.id.type === "ObjectPattern" &&
            d.init?.callee?.name === "$props"
          )
            for (const p of d.id.properties) {
              const name = p.key?.name;
              env[name] =
                p.value?.type === "AssignmentPattern"
                  ? value(p.value.right)
                  : undefined;
            }
        }
    Object.assign(env, props);
    const errors = [];
    if (
      model.ast.css &&
      /(?:width|height|padding|margin|inset|transform)\s*:[^;}]*\d(?:px|rem|em)\b/.test(
        model.ast.css.content.styles,
      )
    )
      errors.push(`${file}: fixture CSS overrides application geometry.`);
    function nodes(fragment) {
      let reaches = false;
      for (const node of fragment?.nodes ?? []) {
        if (node.type === "IfBlock") {
          const condition = value(node.test, env);
          if (condition === undefined) {
            const left = nodes(node.consequent),
              right = nodes(node.alternate);
            const hasContent = (f) =>
              (f?.nodes ?? []).some((n) => n.type !== "Text" || n.data.trim());
            if (
              left !== right &&
              hasContent(node.consequent) &&
              hasContent(node.alternate)
            )
              errors.push(
                `${file}: unresolved conditional application/standalone branch.`,
              );
            reaches = reaches || left || right;
          } else {
            const branch = nodes(condition ? node.consequent : node.alternate);
            reaches = reaches || branch;
          }
        } else if (node.type === "Component") {
          const selected = resolveExport(
            target(file, model.imports.get(node.name)),
          );
          if (selected === "catalog/fixtures/FixtureControls.svelte") continue;
          if (selected) {
            const args = {};
            for (const attribute of node.attributes ?? [])
              if (attribute.type === "Attribute")
                args[attribute.name] = attr(node, attribute.name, env);
            const outcome = composition(selected, args, [...chain, file]);
            if (outcome.root) {
              if (
                node.fragment?.nodes?.some(
                  (n) => n.type !== "Text" || n.data.trim(),
                )
              )
                errors.push(
                  `${file}: application root cannot accept replacement children.`,
                );
              if (args.displayMode && args.displayMode !== "auto")
                errors.push(
                  `${file}: application responsiveness must use viewport size.`,
                );
              reaches = true;
            }
            errors.push(...outcome.errors);
          } else
            errors.push(
              `${file}: unresolved rendered application component ${node.name}.`,
            );
        } else if (node.type === "SvelteComponent" || node.type === "RenderTag")
          errors.push(
            `${file}: dynamic application composition requires a statically resolved adapter.`,
          );
        else if (node.type === "Text" && node.data.trim())
          errors.push(`${file}: fixture text must be inside FixtureControls.`);
        else if (node.fragment) {
          const style = attr(node, "style", env);
          if (
            typeof style === "string" &&
            /(?:width|height|padding|margin|inset|transform)\s*:[^;]*\d(?:px|rem|em)\b/.test(
              style,
            )
          )
            errors.push(`${file}: inline application geometry override.`);
          const children = nodes(node.fragment);
          reaches = reaches || children;
        }
      }
      return reaches;
    }
    const reaches = nodes(model.ast.fragment);
    return { root: reaches, errors };
  }
  // The trusted adapter is itself checked, so replacing it cannot waive application parity.
  for (const entry of [host, "web/src/App.svelte"].filter((file) =>
    known.has(file),
  )) {
    const model = load(entry);
    const roots = [];
    walk(model.ast.fragment, (node) => {
      if (
        node.type === "Component" &&
        target(entry, model.imports.get(node.name)) === production
      )
        roots.push(node);
    });
    if (
      roots.length !== 1 ||
      model.ast.fragment.nodes.filter((n) => n.type !== "Text" || n.data.trim())
        .length !== 1 ||
      !model.ast.fragment.nodes.includes(roots[0]) ||
      roots[0].fragment?.nodes?.some((n) => n.type !== "Text" || n.data.trim())
    )
      add(
        entry,
        "The host must render one unmodified shared production application root.",
      );
    if (model.ast.css)
      add(
        entry,
        "Application presentation styles belong in the shared production root.",
      );
  }
  for (const file of files.filter((f) =>
    /\.stories\.(?:svelte|[jt]sx?)$/.test(f),
  )) {
    if (!file.endsWith(".svelte")) {
      add(file, "Story source format requires explicit parity parser support.");
      continue;
    }
    let model;
    try {
      model = load(file);
    } catch (error) {
      add(file, `Cannot parse story: ${error.message}`);
      continue;
    }
    for (const story of model.stories) {
      const params = {
        ...model.metadata.parameters,
        ...attr(story, "parameters"),
      };
      const contract = params.jjStoreStory;
      if (
        !contract ||
        !["application", "component", "prototype"].includes(contract.kind)
      ) {
        add(
          file,
          "Every story must explicitly classify jjStoreStory.kind.",
          story,
        );
        continue;
      }
      if (
        contract.kind === "component" &&
        (!contract.subject?.startsWith("web/src/lib/") ||
          !contract.subject.endsWith(".svelte") ||
          !known.has(contract.subject))
      ) {
        add(
          file,
          "Component stories must identify an existing production subject.",
          story,
        );
        continue;
      }
      if (
        contract.kind === "prototype" &&
        (!/^JST-[A-Z]+-\d+$/.test(contract.requirement ?? "") ||
          typeof contract.reason !== "string" ||
          contract.reason.length < 20)
      ) {
        add(
          file,
          "Prototype stories require a governing requirement and purpose.",
          story,
        );
        continue;
      }
      if (contract.kind !== "application") continue;
      if (model.ast.css)
        add(
          file,
          "Application story styles belong in production, not the story.",
          story,
        );
      const props = { ...model.metadata.args, ...attr(story, "args") };
      let outcome;
      const contents =
        story.fragment?.nodes?.filter(
          (n) => n.type !== "Text" || n.data.trim(),
        ) ?? [];
      if (contents.length) {
        // Evaluate the selected template, rather than the metadata's unused component import.
        const template = contents.find(
          (n) =>
            n.type === "SnippetBlock" &&
            ["template", "children"].includes(n.expression?.name),
        );
        const fragment = template?.body ?? story.fragment;
        const temporary = `${file}#template`;
        cache.set(temporary, {
          ...model,
          ast: { ...model.ast, instance: undefined, fragment, css: undefined },
        });
        outcome = composition(temporary, props);
      } else
        outcome = composition(
          resolveExport(target(file, model.imports.get(model.component))),
          props,
        );
      if (!outcome.root)
        add(
          file,
          "Application story must render the shared production application root.",
          story,
        );
      for (const error of [...new Set(outcome.errors)]) add(file, error, story);
    }
  }
  return findings;
}
export function validate(context) {
  return parityFindings({
    root: context.model.repoRoot,
    files: context.trackedFiles,
  });
}
export default { name, validate };
