<script lang="ts">
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  const notifications = ownNotifications();
  import { indexPreparationLabels } from "../../controller/index-preparation-labels.js";
  import type { StoreController } from "../../controller/contracts.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { controller }: { controller: StoreController } = $props();
  const settings = $derived(controller.state.settings);
  const label = $derived(
    indexPreparationLabels[settings.indexPreparationKind ?? "graph"],
  );
  const preparing = $derived(settings.indexPreparation === "loading");
  const failed = $derived(!!settings.indexPreparationError);
  const message = $derived(
    failed
      ? settings.indexPreparationError
      : preparing
        ? settings.indexTaskStatus === "canceling"
          ? `Canceling ${label.toLowerCase()} preparation…`
          : `Preparing the ${label.toLowerCase()}…`
        : settings.indexTaskStatus === "canceled"
          ? `${label} preparation canceled.`
          : settings.indexPreparation === "ready"
            ? `${label} preparation completed.`
            : "",
  );
</script>

<StoreNotice
  noticeKey="index-preparation"
  scope={JSON.stringify([
    "repository-index",
    controller.state.route?.repository,
    settings.indexOperation,
  ])}
  title={`${label} preparation`}
  {message}
  severity={failed ? "error" : "info"}
  lifetime={preparing || failed ? "condition" : "dismiss"}
  action={preparing && settings.indexTaskStatus === "running"
    ? {
        id: "cancel-index",
        label: settings.indexCancellationUnconfirmed
          ? "Retry cancellation"
          : "Cancel preparation",
        run: () => void controller.actions.cancelIndex(),
      }
    : failed
      ? {
          id: "prepare-index",
          label: "Retry preparation",
          run: () =>
            settings.indexPreparationKind &&
            settings.indexPreparationKind !== "graph"
              ? void controller.actions.prepareKindIndex?.(
                  settings.indexPreparationKind,
                )
              : void controller.actions.prepareIndex(),
        }
      : undefined}
/>

<StoreNotice
  noticeKey="index-preparation-summary"
  scope={JSON.stringify([
    "repository-index",
    controller.state.route?.repository,
    settings.indexOperation,
  ])}
  title="Prepared index summary"
  severity="info"
  lifetime="dismiss"
  message={settings.indexPreparation === "ready" && settings.indexProgress
    ? `${settings.indexProgress.matching_ids.toLocaleString()} matching identities in ${settings.indexProgress.stats.windows.toLocaleString()} windows; ${settings.indexProgress.stats.scanned.toLocaleString()} descriptors inspected.`
    : ""}
/>

<StoreNotificationHost owner={notifications} />
