import { MockRibbonAgent, UserRibbonAgent, type RibbonAgent } from '@ribbonette/agent';

import { dev } from '$app/environment';
import { PUBLIC_BACKEND_URL } from '$env/static/public';

export const agent: RibbonAgent = dev ? new MockRibbonAgent() : new UserRibbonAgent(PUBLIC_BACKEND_URL);