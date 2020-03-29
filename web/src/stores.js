import { writable } from 'svelte/store';

export const backlistedBiomes = writable(new Set());
