import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import type { ClientResult } from "$lib/shared-types/result";
import type {
  CreateTaskRequest,
  TaskCard,
  UpdateTaskRequest,
} from "$lib/shared-types/task";
import { authClient } from "$lib/auth-client";
import { playCompletionSound } from "$lib/completion-sound";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  createMutation,
  createQuery,
  useQueryClient,
} from "@tanstack/svelte-query";
const byLastUpdated = (a: TaskCard, b: TaskCard) =>
  new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime();

async function redirectIfUnauthorized(result: ClientResult<unknown>) {
  if (result.statusCode === 401) {
    await goto(resolve("/(auth)/login"));
  }
}

let lastCompletionNotification: { taskId: string; at: number } | undefined;

async function restoreWindowAfterTimer() {
  try {
    const appWindow = getCurrentWindow();
    await appWindow.unminimize().catch((error: unknown) => {
      console.warn("Could not unminimize the app window:", error);
    });
    await appWindow.show().catch((error: unknown) => {
      console.warn("Could not show the app window:", error);
    });
    await appWindow.maximize().catch((error: unknown) => {
      console.warn("Could not maximize the app window:", error);
    });
    await appWindow.setFocus().catch((error: unknown) => {
      console.warn("Could not focus the app window:", error);
    });
  } catch (error) {
    console.warn(
      "Could not restore the app window after timer completion:",
      error,
    );
  }
}

/** Called only when a focus timer reaches zero, never for manual task changes. */
export function notifyTaskCompleted(taskId: string): void {
  const now = Date.now();
  if (
    lastCompletionNotification?.taskId === taskId &&
    now - lastCompletionNotification.at < 15_000
  ) {
    return;
  }
  lastCompletionNotification = { taskId, at: now };

  playCompletionSound();

  // Native window operations are deliberately detached from the task/timer
  // mutation so a slow window manager cannot freeze task completion.
  void restoreWindowAfterTimer();
}

export class TaskStore {
  private queryTasks = createQuery(() => ({
    queryKey: ["tasks"],
    enabled: false,
    staleTime: 1000 * 60 * 5,
    queryFn: async (): Promise<ClientResult<TaskCard[]>> => {
      const result = await authClient.listTasks();
      if (result.statusCode === 401) {
        await goto(resolve("/(auth)/login"));
      }
      return result;
    },
  }));
  tasks = $derived(
    [...(this.queryTasks.data?.value ?? [])].sort(byLastUpdated),
  );
  inboxTasks = $derived(this.tasks.filter((x) => x.status === "INBOX"));
  inProgressTasks = $derived(
    this.tasks.filter((x) => x.status === "IN_PROGRESS"),
  );
  doneTasks = $derived(this.tasks.filter((x) => x.status === "DONE"));
  private queryClient = useQueryClient();
  errorGetTasks = $state<string>("");
  isLoadingTasks = $state<boolean>(false);
  isLoadingDelete = $state<boolean>(false);
  isLoadingCreate = $state<boolean>(false);

  deleteTask = createMutation(() => ({
    mutationFn: async (req: { id: string }) => {
      const result = await authClient.deleteTask(req.id);
      await redirectIfUnauthorized(result);

      if (result.isSuccess) {
        this.queryClient.setQueryData<ClientResult<TaskCard[]>>(
          ["tasks"],
          (oldData) => {
            if (!oldData?.isSuccess || !oldData.value) {
              return oldData;
            }
            return {
              ...oldData,
              value: oldData.value.filter((task) => task.id !== req.id),
            };
          },
        );
      }
      return result;
    },
  }));

  updateTask = createMutation(() => ({
    mutationFn: async (req: { id: string; task: UpdateTaskRequest }) => {
      const result = await authClient.updateTask(req.id, req.task);
      await redirectIfUnauthorized(result);
      if (result.isSuccess) {
        this.queryClient.setQueryData<ClientResult<TaskCard[]>>(
          ["tasks"],
          (oldData) => {
            if (!oldData?.isSuccess || !oldData.value) {
              return oldData;
            }
            return {
              ...oldData,
              value: oldData.value.map((task) =>
                task.id === result.value?.id ? result.value! : task,
              ),
            };
          },
        );
      }
      return result;
    },
  }));

  async loadTasks() {
    this.isLoadingTasks = true;
    try {
      const result = await this.queryTasks.refetch();
      const data = result.data;

      if (!data) {
        this.errorGetTasks = "Failed to load tasks";
        return;
      }

      if (data.isSuccess) {
        this.errorGetTasks = "";
        return;
      }

      this.errorGetTasks = data.errorMsg ?? "Failed to load tasks";
    } finally {
      this.isLoadingTasks = false;
    }
  }
  removeTasks() {
    this.queryClient.setQueryData<ClientResult<TaskCard[]>>(
      ["tasks"],
      (oldData) => {
        if (!oldData) {
          return oldData;
        }

        return { ...oldData, statusCode: 400, isSuccess: false, value: [] };
      },
    );
  }

  createTask = createMutation(() => ({
    mutationFn: async (req: { task: CreateTaskRequest }) => {
      const result = await authClient.createTask(req.task);
      await redirectIfUnauthorized(result);

      if (result.isSuccess) {
        this.queryClient.setQueryData<ClientResult<TaskCard[]>>(
          ["tasks"],
          (oldData) => {
            if (!oldData?.isSuccess || !oldData.value) {
              return oldData;
            }
            return {
              ...oldData,
              value: [result.value!, ...oldData.value],
            };
          },
        );
      }

      return result;
    },
  }));
}
