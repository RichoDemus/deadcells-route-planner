import { writable } from 'svelte/store';

export const backlistedBiomes = writable(new Set());
export const bossCells = writable(5);
