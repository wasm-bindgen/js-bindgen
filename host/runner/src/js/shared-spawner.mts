export const enum WorkerKind {
	Dedicated,
	Shared,
	Service,
}

export const workerKind: WorkerKind = Number(
	new URLSearchParams(window.location.search).get("worker")
)
