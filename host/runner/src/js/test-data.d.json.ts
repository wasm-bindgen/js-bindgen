type TestData = {
	worker: WorkerKind
	noCapture: boolean
	filteredCount: number
	tests: TestEntry[]
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

declare const config: TestData

export default config
