import { Stream, Status } from "../shared/shared.mjs"
import { workerKind, WorkerKind } from "./shared-spawner.mts"

switch (workerKind) {
	case WorkerKind.Dedicated:
		new Worker("../worker/script.mjs", { type: "module" })
		break
	case WorkerKind.Shared: {
		new SharedWorker("../worker/script.mjs", { name: crypto.randomUUID(), type: "module" })
		break
	}
	case WorkerKind.Service:
		await navigator.serviceWorker
			.register("../worker/script.mjs", {
				scope: "./worker/" + crypto.randomUUID(),
				type: "module",
				updateViaCache: "none",
			})
			.catch(async (error: unknown) => {
				await fetch("../report", {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						order: 0,
						report: { stream: Stream.Stderr, line: (error as Error).message + "\n" },
					}),
				}).finally(() =>
					fetch("../finished", {
						method: "POST",
						headers: { "Content-Type": "application/json" },
						body: JSON.stringify({ status: Status.Abnormal, messages: 1 }),
					})
				)
			})
}
