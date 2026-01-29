import { chromium, firefox, webkit } from "playwright";

const url = process.env.JBG_TEST_URL;
if (!url) {
	throw new Error("missing JBG_TEST_URL");
}

const browserName = (process.env.JBG_TEST_BROWSER || "chromium").toLowerCase();
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
});
const page = await browser.newPage();
await page.goto(url);
await page.waitForFunction("window.__jbtestDone === true");
await browser.close();
