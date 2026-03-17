import { runBrowser } from "./shared-browser.mjs";
import { JsBindgen } from "./imports.mjs";
runBrowser(JsBindgen).then(() => self.registration.unregister());
