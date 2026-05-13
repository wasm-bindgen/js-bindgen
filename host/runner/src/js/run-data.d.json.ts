type RunData = TestRunData | BinaryRunData

type TestRunData = {
	kind: "test"
	worker: WorkerKind
	noCapture: boolean
	filteredCount: number
	tests: TestEntry[]
}

type BinaryRunData = {
	kind: "binary"
	worker: WorkerKind
	wasm64: boolean
	args: string[]
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
