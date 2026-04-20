import { runBrowser } from "../shared/shared-browser.mjs"
import { importJsBindgen } from "../shared/shared-import.mjs"

await runBrowser(await importJsBindgen())

self.close()
