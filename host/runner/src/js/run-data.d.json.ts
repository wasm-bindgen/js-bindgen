type RunData = TestRunData | BinaryRunData

type TestRunData = {
	kind: "test"
	ctx: string | null
	worker: WorkerKind
	noCapture: boolean
	filteredCount: number
	tests: TestEntry[]
}

type BinaryRunData = {
	kind: "binary"
	ctx: string | null
	worker: WorkerKind
	wasm64: boolean
}

declare const enum WorkerKind {
	Dedicated,
	Shared,
	Service,
}

type TestEntry = {
	name: string
	importName: string
	ignore: boolean | string
	shouldPanic: boolean | string
}

declare const config: RunData

export default config
