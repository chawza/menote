import type { AppError } from "../../bindings";
import { commands } from "../../bindings";
import { toastStore } from "../stores/toast";

type CommandResult<T> = { status: "ok"; data: T } | { status: "error"; error: AppError };

export async function tryCommand<T>(fn: () => Promise<CommandResult<T>>): Promise<T> {
	const result = await fn();
	if (result.status === "error") {
		const err = result.error;
		toastStore.error(err.message);
		console.error(`[${err.code}]`, err.message);
		throw err;
	}
	return result.data;
}

export { commands };
