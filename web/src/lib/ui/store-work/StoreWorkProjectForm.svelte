<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import type { Model } from "../../api/client.js";
  import { workId, type WorkController } from "../../controller/work-types.js";
  let {
    work,
    project = null,
    disabled = false,
  }: {
    work: WorkController;
    project?: Model<"ProjectV1"> | null;
    disabled?: boolean;
  } = $props();
  let title = $state(""),
    body = $state("");
  $effect(() => {
    title = project?.title ?? "";
    body = project?.body ?? "";
  });
  async function save(event: SubmitEvent) {
    event.preventDefault();
    await work.submit(
      project
        ? {
            command: "update_project",
            id: project.id,
            expected_version: project.version,
            title,
            body,
          }
        : { command: "create_project", id: workId(), title, body },
    );
  }
</script>

<form
  onsubmit={save}
  class="store-work__form"
  aria-label={project ? "Edit project" : "Create project"}
>
  <label
    >Project title<Input
      bind:value={title}
      required
      maxlength={512}
      {disabled}
    /></label
  >
  <label
    >Project description<Textarea
      bind:value={body}
      maxlength={65536}
      {disabled}
    /></label
  >
  <Button type="submit" disabled={disabled || !title.trim()}
    >{project ? "Save project" : "Create project"}</Button
  >
</form>
