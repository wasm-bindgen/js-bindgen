import { chromium, firefox, webkit } from "playwright";

const url = process.env.JBTEST_URL;
if (!url) {
	throw new Error("missing JBTEST_URL");
}

const browserName = (process.env.JBTEST_BROWSER || "chromium").toLowerCase();
const browserType = {
	chromium,
	firefox,
	webkit,
}[browserName];

if (!browserType) {
	throw new Error(`unsupported browser: ${browserName}`);
}

const browser = await browserType.launch({
	headless: true,
	chromiumSandbox: false,
});
const page = await browser.newPage();
await page.goto(url);
await page.waitForFunction("window.__jbtestDone === true");
await browser.close();
