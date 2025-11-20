import * as Sentry from '@sentry/sveltekit';

import { dev } from '$app/environment';
import { PUBLIC_SENTRY_URL } from '$env/static/public';
Sentry.init({
	dsn: PUBLIC_SENTRY_URL,
	enabled: !dev,
	sendDefaultPii: true
});

export const handleError = Sentry.handleErrorWithSentry();