// place files you want to import through the `$lib` alias in this folder.

import { env } from "$env/dynamic/public";

export type SortMode = "ASC" | "DESC";
export type Property = {
	enabled: boolean;
	order: number;
	name: string;
	category: string;
	filter: string[];
};

export type PlayerInfoExtra = {
	id: string;
	image?: string;
	username?: string;
	display_username?: string;
	about_me?: string;
	region?: string;
	run_count: number;
	win_count: number;
	favourite_survivor?: string;
	favourite_difficulty?: string;
};

export function validUrl(url: string | undefined): boolean {
	if (!url) return false;
	try {
		const parsed = new URL(url);
		return parsed.protocol === 'http:' || parsed.protocol === 'https:';
	} catch {
		return false;
	}
}

export function api(route: string): string {
	return `${env.PUBLIC_BACKEND_URL}/api/${route}`;
}


export function auth(route: string): string {
	return `${env.PUBLIC_BACKEND_URL}/auth/${route}`;
}
