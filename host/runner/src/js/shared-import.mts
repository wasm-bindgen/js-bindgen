import type { JsBindgen } from "./imports.mts"

export async function importJsBindgen(): Promise<typeof JsBindgen | Error> {
	try {
		const { JsBindgen } = await import("./imports.mts")
		return JsBindgen
	} catch (error) {
		return error as Error
	}
}
