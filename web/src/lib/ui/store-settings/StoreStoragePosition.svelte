<script lang="ts">
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type { StorageOperation } from "../../controller/storage-settings.js";
  let { position }: { position: StorageOperation["target"] } = $props();
</script>

<dl class="store-settings__rows">
  <div>
    <dt>Repository</dt>
    <dd><CopyableValue value={position.repository} label="Repository" /></dd>
  </div>
  <div>
    <dt>Epoch</dt>
    <dd><CopyableValue value={position.epoch} label="Storage epoch" /></dd>
  </div>
  <div>
    <dt>Published position</dt>
    <dd>{position.position.toLocaleString("en-GB")}</dd>
  </div>
  <div>
    <dt>Manifest revision</dt>
    <dd>{position.revision.toLocaleString("en-GB")}</dd>
  </div>
  <div>
    <dt>Manifest digest</dt>
    <dd><CopyableValue value={position.digest} label="Manifest digest" /></dd>
  </div>
</dl>
