import { redirect } from '@sveltejs/kit';

import { PUBLIC_DISCORD_APP_URL } from '$env/static/public';
export async function load() {
	redirect(307, PUBLIC_DISCORD_APP_URL);
}