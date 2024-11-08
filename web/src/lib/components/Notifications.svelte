<script lang="ts">
  import type { Notification, NotificationLevel } from "$lib/types";

  import PhCheckCircle from "virtual:icons/ph/check-circle";
  import PhInfo from "virtual:icons/ph/info";
  import PhQuestion from "virtual:icons/ph/question";
  import PhWarningCircle from "virtual:icons/ph/warning-circle";

  import { onMount } from "svelte";

  import { notifications } from "$lib/stores";

  let toasts: Array<[number, Notification]> = $state([]);

  onMount(() => {
    return notifications.subscribe((n) => {
      if (n != undefined) {
        toasts.push([new Date().getTime(), n]);
        toasts = toasts;
      }
    });
  });

  onMount(() => {
    const intvl = setInterval(() => {
      const now = new Date().getTime();
      let changed = false;
      let i = toasts.length;

      while (i--) {
        const t = toasts[i];
        if (now - t[0] > t[1].timeout) {
          toasts.splice(i, 1);
          changed = true;
        }
      }

      if (changed) {
        toasts = toasts;
      }
    }, 1000);

    return () => {
      clearInterval(intvl);
    };
  });

  const getStyle = (level: NotificationLevel) => {
    // I know this could be `alert-${level}`
    // But svelte / tailwind don't recognise that
    // and the styles don't get included in the bundle :(
    switch (level) {
      case "success":
        return "alert-success";
      case "info":
        return "alert-info";
      case "warning":
        return "alert-warning";
      case "error":
        return "alert-error";
    }
  };

  const getIcon = (level: NotificationLevel) => {
    switch (level) {
      case "success":
        return PhCheckCircle;
      case "info":
        return PhInfo;
      case "warning":
        return PhQuestion;
      case "error":
        return PhWarningCircle;
    }
  };
</script>

<div class="toast toast-center toast-bottom">
  {#each toasts as [_d, toast]}
    {@const Icon = getIcon(toast.level)}
    <div role="alert" class="alert shadow-lg {getStyle(toast.level)} ">
      <Icon />
      <span>{toast.message}</span>
    </div>
  {/each}
</div>

<style>
  .toast {
    z-index: 9999;
  }
</style>
