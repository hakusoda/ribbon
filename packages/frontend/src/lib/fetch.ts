export async function fetch_json<T>(url: string, init?: RequestInit): Promise<T> {
	const response = await fetch(url, init);
	if (response.status < 200 || response.status >= 300)
		throw new FetchError(await response.text(), response.status);
	
	return response.json();
}

export class FetchError extends Error {
	constructor(message: string, public status: number) {
		super(message);
		this.name = "FetchError";
	}
}