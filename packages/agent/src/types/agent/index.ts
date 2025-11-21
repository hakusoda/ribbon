import type { RibbonAgentServerApi } from './apis/server';

export interface RibbonAgent {
	readonly servers: RibbonAgentServerApi
}