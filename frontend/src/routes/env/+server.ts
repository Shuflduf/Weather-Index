import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = () => {
  return new Response(JSON.stringify(env));
};
