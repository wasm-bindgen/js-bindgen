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
		await navigator.serviceWorker.register("./worker.mjs", {
			scope: crypto.randomUUID(),
			type: "module",
			updateViaCache: "none",
		})
}
