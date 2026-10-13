<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { ChipAutocomplete } from "@lapismd/design-core/forms";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import WorkflowHostPrincipalPicker from "./WorkflowHostPrincipalPicker.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import Workflow from "@lucide/svelte/icons/workflow";
  import User from "@lucide/svelte/icons/user";
  import Folder from "@lucide/svelte/icons/folder";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Tags from "@lucide/svelte/icons/tags";
  import Shield from "@lucide/svelte/icons/shield";
  import type { Model } from "../../api/client.js";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import type { WorkflowHostChoices } from "../../controller/workflow-host-choices.svelte.js";
  let {
    configuration,
    choices,
    repositories = [],
    authors = [],
    disabled,
    onChange,
  }: {
    configuration: Model<"WorkflowHostConfiguration">;
    choices: WorkflowHostChoices;
    repositories?: readonly StoreRepositorySummary[];
    authors?: readonly { id: string; name: string }[];
    disabled: boolean;
    onChange(key: string, value: unknown): void;
  } = $props();
  let query = $state("");
  const homes = $derived([
    ...new Map(
      [
        ...choices.state.bindings
          .filter((b) => b.availability === "ready")
          .map((b) => ({ id: b.path, name: b.name, description: b.path })),
        ...configuration.sources
          .filter((s) => s.workspace)
          .map((s) => ({
            id: s.workspace,
            name:
              repositories.find((r) => r.id === s.repository)?.name ||
              s.repository,
            description: s.workspace,
          })),
        ...(configuration.home
          ? [
              {
                id: configuration.home,
                name:
                  repositories.find((r) =>
                    configuration.sources.some(
                      (s) =>
                        s.workspace === configuration.home &&
                        s.repository === r.id,
                    ),
                  )?.name || "Current metadata home",
                description: configuration.home,
              },
            ]
          : []),
        ...(query.trim().startsWith("/")
          ? [
              {
                id: query.trim(),
                name: "Use native attachment path",
                description: query.trim(),
              },
            ]
          : []),
      ].map((r) => [r.id, r]),
    ).values(),
  ]);
</script>

{#each [{ key: "host", label: "Host name", help: "A stable name for this host's journal.", Icon: Workflow }, { key: "state_dir", label: "Recovery state directory", help: "Private directory outside repository checkouts. Keep its journal across reloads.", Icon: Folder }, { key: "jj_executable", label: "JJ executable", help: "Absolute path to the paired custom JJ CLI.", Icon: Terminal }] as field}
  <Field.Field orientation="responsive" class="store-settings__option-row"
    ><Field.Content
      ><Field.Label for={`workflow-${field.key}`}
        ><StoreSettingsIcon Icon={field.Icon} />{field.label}</Field.Label
      ><Field.Description>{field.help}</Field.Description></Field.Content
    >
    <Input
      class="store-settings__input"
      id={`workflow-${field.key}`}
      value={String(configuration[field.key as "host"])}
      {disabled}
      oninput={(e) => onChange(field.key, e.currentTarget.value)}
    />
  </Field.Field>
{/each}
<Field.Field orientation="responsive" class="store-settings__option-row"
  ><Field.Content
    ><Field.Label for="workflow-principal"
      ><StoreSettingsIcon Icon={User} />Execution principal</Field.Label
    ><Field.Description
      >The principal authenticated by the native CLI attachment.</Field.Description
    ></Field.Content
  >
  <div class="workflow-host-picker">
    <WorkflowHostPrincipalPicker
      id="workflow-principal"
      label="Execution principal"
      multiple={false}
      values={configuration.principal ? [configuration.principal] : []}
      {authors}
      {disabled}
      onChange={(v) => onChange("principal", v[0] || "")}
    />
  </div>
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row"
  ><Field.Content
    ><Field.Label for="workflow-home"
      ><StoreSettingsIcon Icon={Folder} />Metadata home</Field.Label
    ><Field.Description
      >Select a native metadata attachment or enter its absolute path.</Field.Description
    ></Field.Content
  >
  <div class="workflow-host-picker">
    <StoreRepositoryPicker
      id="workflow-home"
      label="Metadata home"
      repositories={homes}
      value={configuration.home}
      {disabled}
      onOpen={() => void choices.refresh()}
      loading={choices.state.loading}
      onSearch={(v) => (query = v)}
      onChange={(v) => onChange("home", v)}
      searchLabel="Find a metadata repository or native attachment path"
      groupLabel="Native attachments"
    />
  </div>
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row"
  ><Field.Content
    ><Field.Label for="workflow-labels"
      ><StoreSettingsIcon Icon={Tags} />Runner labels</Field.Label
    ><Field.Description
      >Every label required by a workflow must match.</Field.Description
    ></Field.Content
  >
  <fieldset {disabled} class="workflow-host-picker">
    <ChipAutocomplete
      id="workflow-labels"
      label="Runner labels"
      showLabel={false}
      uppercase={false}
      placeholder="Add runner label…"
      value={configuration.labels ?? []}
      onChange={(items) => {
        if (!disabled) onChange("labels", items);
      }}
    />
  </fieldset>
</Field.Field>
<Field.Field orientation="responsive" class="store-settings__option-row"
  ><Field.Content
    ><Field.Label for="workflow-allowed_owners"
      ><StoreSettingsIcon Icon={Shield} />Allowed workflow owners</Field.Label
    ><Field.Description
      >Only these principals' definitions may execute.</Field.Description
    ></Field.Content
  >
  <div class="workflow-host-picker">
    <WorkflowHostPrincipalPicker
      id="workflow-allowed_owners"
      label="Allowed workflow owners"
      values={configuration.allowed_owners}
      {authors}
      {disabled}
      onChange={(v) => onChange("allowed_owners", v)}
    />
  </div>
</Field.Field>

<style>
  .workflow-host-picker {
    width: min(22rem, 100%);
    border: 0;
    padding: 0;
    margin: 0;
    min-width: 0;
  }
</style>
