import type { RequestHandler } from './$types';
import { env } from '$env/dynamic/private';

export const GET: RequestHandler = () => {
  const allowedKeys = ["VERCEL", "VERCEL_GIT_COMMIT_SHA"]
  return new Response(
    JSON.stringify(
      Object.fromEntries(
        Object.entries(env).filter(
          ([key, _]) => allowedKeys.includes(key)
        )
      )
    )
  );
};
