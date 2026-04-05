export function formatLocal(ts: number): string {
  const date = new Date(ts * 1000);
  return date.toLocaleString(undefined, {
    dateStyle: 'medium',
    timeStyle: 'short',
  });
}

export function nowUnix(): number {
  return Math.floor(Date.now() / 1000);
}
