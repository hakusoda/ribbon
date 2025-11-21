import MockRibbonAgentServerApi from './apis/server';
import type { RibbonAgent } from '../types/agent';

export default class MockRibbonAgent implements RibbonAgent {
	public servers: MockRibbonAgentServerApi
	
	public constructor() {
		this.servers = new MockRibbonAgentServerApi(this);
	}
}