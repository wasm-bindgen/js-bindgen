import { Stream, Status } from "./shared.mts"
import { workerKind, WorkerKind } from "./shared-spawner.mts"

switch (workerKind) {
	case WorkerKind.Dedicated:
		new Worker("./worker.mjs", { type: "module" })
		break
	case WorkerKind.Shared: {
		new SharedWorker("./worker.mjs", { name: crypto.randomUUID(), type: "module" })
		break
	}
	case WorkerKind.Service:
		await navigator.serviceWorker
			.register("./worker.mjs", {
				scope: crypto.randomUUID(),
				type: "module",
				updateViaCache: "none",
			})
			.catch(async error => {
				await fetch("./report", {
					method: "POST",
					headers: { "Content-Type": "application/json" },
					body: JSON.stringify({
						order: 0,
						stream: Stream.Stderr,
						line: (error as Error).message + "\n",
					}),
				}).finally(() =>
					fetch("/finished", {
						method: "POST",
						headers: { "Content-Type": "application/json" },
						body: JSON.stringify(Status.Abnormal),
					})
				)
			})
}
