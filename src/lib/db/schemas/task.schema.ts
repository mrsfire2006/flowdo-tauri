export const taskPriority = {
  enumValues: ["HIGH", "MEDIUM", "LOW"] as const,
};

export type TaskPriority = (typeof taskPriority.enumValues)[number];

export const taskStatus = {
  enumValues: ["INBOX", "IN_PROGRESS", "DONE"] as const,
};

export type TaskStatus = (typeof taskStatus.enumValues)[number];

export interface TaskSelect {
  id: string;
  title: string | null;
  description: string | null;
  estimatedMinutes: number | null;
  priority: TaskPriority;
  status: TaskStatus;
  userId: string;
  createdAt: Date | string;
  updatedAt: Date | string;
  completedAt: Date | string | null;
  focusStartedAt: Date | string | null;
  focusElapsedSeconds: number;
}

export interface TaskInsert {
  id: string;
  userId: string;
  title?: string | null;
  description?: string | null;
  status?: TaskStatus;
  priority?: TaskPriority;
  estimatedMinutes?: number | null;
  focusStartedAt?: Date | string | null;
  focusElapsedSeconds?: number;
  completedAt?: Date | string | null;
  createdAt?: Date | string;
  updatedAt?: Date | string;
};
