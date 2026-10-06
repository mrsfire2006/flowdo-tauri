import type { TaskInsert, TaskSelect } from '$lib/db/schemas/task.schema';

export type CreateTaskRequest = Omit<
	TaskInsert,
	| 'id'
	| 'createdAt'
	| 'updatedAt'
	| 'userId'
	| 'status'
	| 'completedAt'
	| 'focusStartedAt'
	| 'focusElapsedSeconds'
>;
export type TaskCard = Omit<
	TaskSelect,
	'completedAt' | 'estimatedMinutes' | 'userId' | 'updatedAt' | 'completedAt' | 'createdAt'
> & {
	durationMinutes: TaskSelect['estimatedMinutes'];
	updatedAt: string;
	createdAt:string;
	completedAt : string | undefined
};

export type UpdateTaskRequest = Partial<
	Omit<TaskInsert, 'id' | 'userId' | 'createdAt' | 'updatedAt' | 'completedAt'>
>;
