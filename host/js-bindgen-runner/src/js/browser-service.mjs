/// <reference lib="webworker" />
import { runBrowser } from "./shared-browser.mjs";
runBrowser().then(() => self.registration.unregister());
