import type { TaskCard } from './shared-types/task';

type Options = {
	getTask: () => TaskCard;
	onDragStart?: () => void;
	onDrop?: () => void;
};

export function draggable(node: HTMLElement, options: Options) {
	let cleanup: (() => void) | undefined;
	let destroyed = false;

	import('@atlaskit/pragmatic-drag-and-drop/element/adapter').then(
		({ draggable: atlaskitDraggable }) => {
			if (destroyed) return;
			const handle = node.querySelector<HTMLElement>('[data-drag-handle]');
			cleanup = atlaskitDraggable({
				element: node,

				dragHandle: handle ?? undefined,

				getInitialData: () => ({
					type: 'task',
					task: options.getTask()
				}),

				onDragStart() {
					node.style.visibility = 'hidden';

					options.onDragStart?.();
				},

				onDrop({ location }) {
					const target = location.current.dropTargets[0];
					const movedToOtherColumn = target && target.data.status !== options.getTask().status;

					if (!movedToOtherColumn) {
						node.style.visibility = '';
					}
					options.onDrop?.();
				}
			});
		}
	);

	return {
		destroy() {
			destroyed = true;
			cleanup?.();
		}
	};
}
