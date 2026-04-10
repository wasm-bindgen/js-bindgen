import { runBrowser } from "./shared-browser.mjs";
import { importJsBindgen } from "./shared-import.mjs";
await runBrowser(await importJsBindgen());
self.close();
