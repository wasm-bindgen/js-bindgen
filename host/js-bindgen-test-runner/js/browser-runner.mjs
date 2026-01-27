import { createTextFormatter, installConsoleProxy, withConsoleCapture } from "./shared.mjs";
import { runTests } from "./runner-core.mjs";

export async function runBrowser({ nocapture, filtered, worker }) {
	const consoleProxy = installConsoleProxy();
	const baseLog = consoleProxy.base.log;
	const baseError = consoleProxy.base.error;
	const baseWarn = consoleProxy.base.warn;
	const baseInfo = consoleProxy.base.info;
	const baseDebug = consoleProxy.base.debug;

	let result = worker
		? await runInWorker({
			nocapture,
			filtered,
			worker,
			baseLog,
			baseError,
			baseWarn,
			baseInfo,
			baseDebug,
		})
		: await runInWindow({ nocapture, filtered, consoleProxy });

	if (typeof window !== "undefined") {
		window.__jbtestDone = true;
		window.__jbtestFailed = result.failed;
	}

	return result;
}

async function runInWindow({ nocapture, filtered, consoleProxy }) {
	const tests = await (await fetch("/tests.json")).json();
	const wasmBytes = await (await fetch("/wasm")).arrayBuffer();
	const { importObject } = await import("/import.js");

	const lines = [];
	const formatter = createTextFormatter({
		nocapture,
		write(line) {
			lines.push(line);
			appendOutput(line);
		},
	});

	const testInputs = tests.map(test => ({
		...test,
		run(testFn) {
			return withConsoleCapture({
				name: test.name,
				run: () => testFn(),
				emit: event => formatter.onEvent(event),
				consoleProxy,
				forwardToConsole: true,
			});
		},
	}));

	const result = await runTests({
		wasmBytes,
		importObject,
		tests: testInputs,
		filtered,
		emit: event => formatter.onEvent(event),
	});

	return { lines, failed: result.failed };
}

async function runInWorker({
	nocapture,
	filtered,
	worker,
	baseLog,
	baseError,
	baseWarn,
	baseInfo,
	baseDebug,
}) {
	const lines = [];

	function handleMessage(event) {
		const data = event.data || {};
		if (data.type === "user-output") {
			switch (data.level) {
				case "error":
					baseError(data.line);
					break;
				case "warn":
					baseWarn(data.line);
					break;
				case "info":
					baseInfo(data.line);
					break;
				case "debug":
					baseDebug(data.line);
					break;
				default:
					baseLog(data.line);
			}
			return null;
		}
		if (data.type === "line") {
			lines.push(data.line);
			appendOutput(data.line);
			return null;
		}
		if (data.type === "report") {
			return { lines: data.lines || lines, failed: data.failed || 0 };
		}
		return null;
	}

	const workerRunners = {
		dedicated: runDedicatedWorker,
		shared: runSharedWorker,
		service: runServiceWorker,
	};
	const runWorker = workerRunners[worker];
	if (!runWorker) {
		throw new Error(`unsupported worker worker: ${worker}`);
	}
	const reportPromise = runWorker({ filtered, nocapture, handleMessage });

	const report = await reportPromise;
	return report;
}

function runDedicatedWorker({ filtered, nocapture, handleMessage }) {
	return new Promise((resolve, reject) => {
		let worker;
		try {
			worker = new Worker("/worker-runner.mjs", { type: "module" });
		} catch (error) {
			reject(error);
			return;
		}
		const timeout = setTimeout(() => {
			reject(new Error("dedicated worker timed out"));
			worker.terminate();
		}, 30000);
		worker.onmessage = event => {
			const report = handleMessage(event);
			if (report) {
				clearTimeout(timeout);
				resolve(report);
				worker.terminate();
			}
		};
		worker.onerror = err => reject(err);
		worker.postMessage({ filtered, nocapture });
	});
}

function runSharedWorker({ filtered, nocapture, handleMessage }) {
	return new Promise((resolve, reject) => {
		let shared;
		try {
			shared = new SharedWorker("/worker-runner.mjs", { type: "module" });
		} catch (error) {
			reject(error);
			return;
		}
		const port = shared.port;
		const timeout = setTimeout(() => {
			reject(new Error("shared worker timed out"));
			port.close();
		}, 30000);
		port.onmessage = event => {
			const report = handleMessage(event);
			if (report) {
				clearTimeout(timeout);
				resolve(report);
				port.close();
			}
		};
		port.onmessageerror = err => reject(err);
		port.start();
		port.postMessage({ filtered, nocapture });
	});
}

async function runServiceWorker({ filtered, nocapture, handleMessage }) {
	if (!navigator.serviceWorker) {
		throw new Error("service workers are not supported");
	}
	const registration = await navigator.serviceWorker.register("/service-worker.mjs", {
		type: "module",
	});
	await navigator.serviceWorker.ready;

	if (!navigator.serviceWorker.controller) {
		if (!sessionStorage.getItem("jbtest-sw-reload")) {
			sessionStorage.setItem("jbtest-sw-reload", "1");
			location.reload();
			return new Promise(() => { });
		}
		throw new Error("service worker not controlling the page");
	}

	return new Promise((resolve, reject) => {
		const channel = new MessageChannel();
		const timeout = setTimeout(() => {
			reject(new Error("service worker timed out"));
			channel.port1.close();
			channel.port2.close();
		}, 30000);
		channel.port1.onmessage = event => {
			const report = handleMessage(event);
			if (report) {
				clearTimeout(timeout);
				resolve(report);
				channel.port1.close();
				channel.port2.close();
			}
		};
		channel.port1.onmessageerror = err => reject(err);
		navigator.serviceWorker.controller.postMessage(
			{ filtered, nocapture },
			[channel.port2]
		);
	});
}

function appendOutput(line) {
	const output = ensureOutput();
	if (output.textContent.length > 0) {
		output.textContent += "\n";
	}
	output.textContent += stripAnsi(line);
}

function stripAnsi(line) {
	return line.replace(/\x1b\[[0-9;]*m/g, "");
}

function ensureOutput() {
	if (typeof document === "undefined") {
		return { textContent: "" };
	}
	let output = document.getElementById("output");
	if (!output) {
		output = document.createElement("pre");
		output.id = "output";
		document.body.append(output);
	}
	return output;
}
