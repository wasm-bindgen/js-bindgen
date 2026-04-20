// eslint-disable-next-line no-var
declare var self: ServiceWorkerGlobalScope

import { runBrowser } from "../shared/shared-browser.mjs"
import { JsBindgen } from "../imports.mts"

void runBrowser(JsBindgen).then(() => self.registration.unregister())
