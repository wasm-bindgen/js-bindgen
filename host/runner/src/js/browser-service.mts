declare var self: ServiceWorkerGlobalScope

import { runBrowser } from "./shared-browser.mts"
import { JsBindgen } from "./imports.mts"

runBrowser(JsBindgen).then(() => self.registration.unregister())
