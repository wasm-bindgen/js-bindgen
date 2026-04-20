import { runBrowser } from "../shared/shared-browser.mjs";
import { JsBindgen } from "../imports.mjs";
void runBrowser(JsBindgen).then(() => self.registration.unregister());
