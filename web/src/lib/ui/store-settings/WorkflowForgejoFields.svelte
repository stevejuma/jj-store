<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import Workflow from "@lucide/svelte/icons/workflow";
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import type { RunnerEnvironment } from "../../controller/workflow-runner-environment.js";
  let {
    configuration,
    disabled,
    onChange,
  }: {
    configuration: RunnerEnvironment;
    disabled: boolean;
    onChange(key: keyof RunnerEnvironment, value: unknown): void;
  } = $props();
  function update(key: string, value: string) {
    onChange("forgejo", { ...configuration.forgejo, [key]: value });
  }
</script>

<Field.Field orientation="responsive" class="store-settings__option-row">
  <Field.Content
    ><Field.Label for="workflow-forgejo-enabled"
      ><StoreSettingsIcon Icon={Workflow} />GitHub workflow executor</Field.Label
    ><Field.Description
      >Forgejo Runner 13.2.0 executes revision-pinned YAML in Docker.</Field.Description
    ></Field.Content
  >
  <Switch
    id="workflow-forgejo-enabled"
    checked={Boolean(configuration.forgejo)}
    {disabled}
    onCheckedChange={(enabled) =>
      onChange(
        "forgejo",
        enabled
          ? {
              program: "",
              docker_program: "",
              image: "node:20-bullseye",
              architecture: "",
              environment: {},
              secrets: {},
            }
          : undefined,
      )}
  />
</Field.Field>
{#if configuration.forgejo}
  {#each [{ key: "program", label: "Forgejo runner executable", placeholder: "/absolute/path/forgejo-runner" }, { key: "docker_program", label: "Docker executable", placeholder: "/absolute/path/docker" }, { key: "image", label: "Job image", placeholder: "node:20-bullseye" }, { key: "architecture", label: "Container architecture", placeholder: "Automatic, linux/arm64 or linux/amd64" }] as field}
    <Field.Field orientation="responsive" class="store-settings__option-row"
      ><Field.Content
        ><Field.Label for={`workflow-forgejo-${field.key}`}
          ><StoreSettingsIcon Icon={Workflow} />{field.label}</Field.Label
        ></Field.Content
      ><Input
        class="workflow-forgejo-input"
        id={`workflow-forgejo-${field.key}`}
        value={String(
          configuration.forgejo[
            field.key as keyof typeof configuration.forgejo
          ] ?? "",
        )}
        placeholder={field.placeholder}
        {disabled}
        oninput={(e) => update(field.key, e.currentTarget.value)}
      /></Field.Field
    >
  {/each}
  <Field.Field orientation="responsive" class="store-settings__option-row"
    ><Field.Content
      ><Field.Label for="workflow-forgejo-docker-host"
        ><StoreSettingsIcon Icon={Workflow} />Docker endpoint</Field.Label
      ><Field.Description
        >Use the endpoint from your Docker context. Leave empty for the default
        socket.</Field.Description
      ></Field.Content
    ><Input
      class="workflow-forgejo-input"
      id="workflow-forgejo-docker-host"
      value={configuration.forgejo.environment.DOCKER_HOST ?? ""}
      placeholder="unix:///path/to/docker.sock"
      {disabled}
      oninput={(e) =>
        onChange("forgejo", {
          ...configuration.forgejo,
          environment: {
            ...configuration.forgejo!.environment,
            DOCKER_HOST: e.currentTarget.value,
          },
        })}
    /></Field.Field
  >
{/if}

<style>
  :global(.workflow-forgejo-input) {
    width: min(32rem, 100%);
    min-width: 0;
    flex: 1;
  }
</style>
