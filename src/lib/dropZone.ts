import type { TaskStatus } from './db/schemas/task.schema';
import type { TaskCard } from './shared-types/task';

type Options = {
	onDragEnter: (status: TaskStatus) => void;
	onDragEnd?: () => void;
	onDragLeave?: () => void;
	onDrop: (id: TaskCard, column: TaskStatus) => void;
	canDrop?: (source: { type: string; [key: string]: unknown }) => boolean;
	getColumnStatus: () => TaskStatus;
};

export function dropZone(node: HTMLElement, options: Options) {
	let cleanup: (() => void) | undefined;
	let destroyed = false;

	import('@atlaskit/pragmatic-drag-and-drop/element/adapter').then(({ dropTargetForElements }) => {
		if (destroyed) return;
		cleanup = dropTargetForElements({
			element: node,

			canDrop: ({ source }) => {
				if (source.data.type !== 'task') return false;
				return options.canDrop?.(source.data as never) ?? true;
			},

			getData: () => ({
				type: 'column',
				status: options.getColumnStatus()
			}),

			onDragEnter({ source }) {
				node.dataset.dragOver = 'true';
				const task = source.data.task as TaskCard;
				options.onDragEnter?.(task.status!);
			},

			onDragLeave() {
				delete node.dataset.dragOver;
				options.onDragLeave?.();
			},

			onDrop({ source, self }) {
				delete node.dataset.dragOver;
				if (source.data.type === 'task') {
					options.onDrop(source.data.task as TaskCard, self.data.status as TaskStatus);
				}
			}
		});
	});

	return {
		destroy() {
			destroyed = true;
			cleanup?.();
		}
	};
}
