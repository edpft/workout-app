import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    const url = "http://localhost:8000/workout/";
    const result = await fetch(url);
    return result.json();
}) satisfies PageLoad;