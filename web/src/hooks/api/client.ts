export async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const r = await fetch(url, {
    headers: { 'Content-Type': 'application/json' },
    ...options,
  });
  if (!r.ok) {
    throw new Error(`API error [${r.status}]: ${r.statusText}`);
  }
  return r.json();
}

export const get = <T>(url: string): Promise<T> => request<T>(url);
export const post = <T>(url: string, body: Record<string, unknown>): Promise<T> =>
  request<T>(url, { method: 'POST', body: JSON.stringify(body) });
