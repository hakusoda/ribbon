import type { RibbonAgent } from '../types/store/session';

export const agent = $state<RibbonAgent | null>(null);