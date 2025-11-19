import * as Sentry from '@sentry/sveltekit';

import { PUBLIC_SENTRY_URL } from '$env/static/public';
Sentry.init({
	dsn: PUBLIC_SENTRY_URL,
	sendDefaultPii: true
});

export const handleError = Sentry.handleErrorWithSentry();