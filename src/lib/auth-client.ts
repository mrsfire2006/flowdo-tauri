import { invoke } from "@tauri-apps/api/core";
import type { ClientResult } from "$lib/shared-types/result";
import type {
  CreateTaskRequest,
  TaskCard,
  UpdateTaskRequest,
} from "$lib/shared-types/task";

function invokeResult<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<ClientResult<T>> {
  const promise = invoke<ClientResult<T>>(command, args);
  return promise.then(
    (result) => result as ClientResult<T>,
    (error: unknown) => ({
      isSuccess: false,
      errorMsg: error instanceof Error ? error.message : String(error),
      statusCode: 500,
    }),
  );
}

export const authClient = {
  signIn: {
    email: (options: {
      email: string;
      password: string;
      rememberMe?: boolean;
    }) =>
      invokeResult("auth_sign_in", {
        email: options.email,
        password: options.password,
        rememberMe: options.rememberMe ?? true,
      }),
  },
  signUp: {
    email: (options: { name: string; email: string; password: string }) =>
      invokeResult("auth_sign_up", {
        name: options.name,
        email: options.email,
        password: options.password,
      }),
  },
  getSession: () =>
    invokeResult<Record<string, unknown> | null>("auth_get_session"),
  signOut: () => invokeResult<{ success: boolean }>("auth_sign_out"),
  listTasks: () => invokeResult<TaskCard[]>("task_list"),
  createTask: (task: CreateTaskRequest) =>
    invokeResult<TaskCard>("task_create", { task }),
  updateTask: (id: string, task: UpdateTaskRequest) =>
    invokeResult<TaskCard>("task_update", { id, task }),
  deleteTask: (id: string) =>
    invokeResult<{ success: boolean }>("task_delete", { id }),
};
