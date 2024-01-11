import { writable, type Writable } from 'svelte/store';

export const notification: Writable<string | null> = writable(null)
