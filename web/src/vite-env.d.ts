/// <reference types="svelte" />
/// <reference types="vite/client" />
declare module '*.svelte';

namespace svelteHTML {
	interface HTMLAttributes<T> {
		'onoutclick'?: () => boolean;
	}
}