import { commands } from '$lib/commands';
import { toastStore } from '$lib/stores/toast';
import type { AppError } from '$lib/types';

type CommandResult<T> =
  | { status: 'ok'; data: T }
  | { status: 'error'; error: AppError };

export async function tryCommand<T>(
  fn: () => Promise<CommandResult<T>>,
): Promise<T> {
  const result = await fn();
  if (result.status === 'error') {
    const err = result.error;
    toastStore.error(err.message);
    console.error(`[${err.code}]`, err.message);
    throw err;
  }
  return result.data;
}

export { commands };
