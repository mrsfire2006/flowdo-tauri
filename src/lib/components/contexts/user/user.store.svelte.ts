import { authClient } from "$lib/auth-client";
import type { ClientResult } from "$lib/shared-types/result";
import type { UserBio } from "$lib/shared-types/user";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";

export class UserStore {
  private queryClient = useQueryClient();
  private query = createQuery(() => ({
    queryKey: ["user"],
    queryFn: async () => {
      const result = await authClient.getSession();
      if (!result.isSuccess) {
        return result as ClientResult<UserBio>;
      }

      const session = result.value as { user?: UserBio } | null | undefined;
      if (!session?.user) {
        return {
          isSuccess: false,
          errorMsg: "You are not signed in.",
          statusCode: 401,
        } satisfies ClientResult<UserBio>;
      }

      return {
        isSuccess: true,
        value: session.user,
        statusCode: 200,
      } satisfies ClientResult<UserBio>;
    },

    staleTime: 1000 * 60 * 5,
  }));

  userBio = $derived(this.query.data?.value);
  isLoadingUser = $state(false);
  errorUser = $state("");

  removeUser() {
    this.queryClient.setQueryData<ClientResult<UserBio>>(
      ["user"],
      (oldData) => {
        if (!oldData) {
          return oldData;
        }

        return {
          ...oldData,
          statusCode: 400,
          isSuccess: false,
          value: undefined,
        };
      },
    );
  }

  async loadUser(): Promise<boolean> {
    this.isLoadingUser = true;
    try {
      const result = await this.query.refetch();

      const data = result.data;

      if (!data) {
        this.errorUser = "Failed to load user";
        return false;
      }

      if (data.isSuccess) {
        this.errorUser = "";
        return true;
      }

      this.errorUser = data.errorMsg ?? "Failed to load user";
      return false;
    } finally {
      this.isLoadingUser = false;
    }
  }
}
