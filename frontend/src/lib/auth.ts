export let token: string | null = null;

export function setToken(t: string | null) {
  token = t
  if (t) localStorage.setItem("session-token", t);
  else localStorage.removeItem("session-token");
}

export function authedFetch(input: RequestInfo, init?: RequestInit): Promise<Response> {
  const headers = new Headers(init?.headers);
  if (token) {
    headers.set("Authorization", `Bearer ${token}`);
  }
  return fetch(input, { ...init, headers, credentials: "include" })
}

if (typeof window != "undefined") {
  token = localStorage.getItem("session-token");
}
