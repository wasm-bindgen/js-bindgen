/// <reference lib="webworker" />
declare var self: ServiceWorkerGlobalScope

import { runBrowser } from "./shared-browser.mts"

runBrowser().then(() => self.registration.unregister())
