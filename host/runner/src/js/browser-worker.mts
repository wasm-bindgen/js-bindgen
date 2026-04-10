import { runBrowser } from "./shared-browser.mts"
import { importJsBindgen } from "./shared-import.mts"

await runBrowser(await importJsBindgen())

self.close()
