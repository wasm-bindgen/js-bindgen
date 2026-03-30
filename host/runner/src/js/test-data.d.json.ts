type TestData = {
	worker: WorkerKind
	benchBaseline?: BenchBaseline
	noCapture: boolean
	filteredCount: number
	tests: TestEntry[]
}

type BenchBaseline = {
    path: string,
    data?: string
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
