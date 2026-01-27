export function createTextFormatter({ nocapture, write }) {
	const buffered = new Map();
	const failed = [];
	const failureReports = [];
	const green = "\u001b[32m";
	const red = "\u001b[31m";
	const yellow = "\u001b[33m";
	const reset = "\u001b[0m";

	function buffer(name, line, stream) {
		if (!buffered.has(name)) {
			buffered.set(name, []);
		}
		buffered.get(name).push({ line, stream });
	}

	function takeBuffer(name) {
		const entries = buffered.get(name);
		if (!entries || entries.length === 0) {
			return [];
		}
		buffered.delete(name);
		return entries;
	}

	return {
		onEvent(event) {
			switch (event.type) {
				case "run-start":
					write("", "stdout");
					write(`running ${event.total} tests`, "stdout");
					break;
				case "test-output":
					if (nocapture) {
						write(event.line, event.stream);
					}
					if (!nocapture) {
						buffer(event.name, event.line, event.stream);
					}
					break;
				case "test-ok":
					takeBuffer(event.name);
					if (event.should_panic) {
						write(
							`test ${event.name} - should panic ... ${green}ok${reset}`,
							"stdout"
						);
					} else {
						write(`test ${event.name} ... ${green}ok${reset}`, "stdout");
					}
					break;
				case "test-ignored":
					takeBuffer(event.name);
					if (event.reason) {
						write(
							`test ${event.name} ... ${yellow}ignored, ${event.reason}${reset}`,
							"stdout"
						);
					} else {
						write(`test ${event.name} ... ${yellow}ignored${reset}`, "stdout");
					}
					break;
				case "test-failed":
					failed.push(event.name);
					if (event.should_panic) {
						write(
							`test ${event.name} - should panic ... ${red}FAILED${reset}`,
							"stdout"
						);
					} else {
						write(`test ${event.name} ... ${red}FAILED${reset}`, "stdout");
					}
					failureReports.push({
						name: event.name,
						entries: takeBuffer(event.name),
						error: event.error,
					});
					break;
				case "run-end":
					write("", "stdout");
					if (failed.length > 0) {
						write("failures:", "stdout");
						write("", "stdout");
						for (const report of failureReports) {
							write(`---- ${report.name} stdout ----`, "stdout");
							for (const entry of report.entries) {
								write(entry.line, entry.stream);
							}
							if (report.error) {
								write("", "stdout");
								write(report.error, "stdout");
							}
							write("", "stdout");
						}
					}
					const status =
						event.status === "ok"
							? `${green}${event.status}${reset}`
							: `${red}${event.status}${reset}`;
					const durationMs = typeof event.duration_ms === "number" ? event.duration_ms : 0;
					const durationSeconds = (durationMs / 1000).toFixed(2);
					if (failed.length > 0) {
						write("failures:", "stdout");
						for (const name of failed) {
							write(`    ${name}`, "stdout");
						}
						write("", "stdout");
					}
					write(
						`test result: ${status}. ${event.passed} passed; ${event.failed} failed; ${event.ignored} ignored; 0 measured; ${event.filtered} filtered out; finished in ${durationSeconds}s`,
						"stdout"
					);
					write("", "stdout");
					break;
				default:
					break;
			}
		},
	};
}

export function installConsoleProxy() {
	const existing = globalThis.__jbtestConsoleProxy;
	if (existing) {
		return existing;
	}
	const base = {
		log: console.log.bind(console),
		error: console.error.bind(console),
		warn: console.warn.bind(console),
		info: console.info.bind(console),
		debug: console.debug.bind(console),
	};

	let hook = null;
	let forwardToConsole = false;
	function make(level) {
		return (...args) => {
			if (hook) {
				hook(level, args);
				if (forwardToConsole) {
					base[level](...args);
				}
			} else {
				base[level](...args);
			}
		};
	}
	console.log = make("log");
	console.error = make("error");
	console.warn = make("warn");
	console.info = make("info");
	console.debug = make("debug");

	const proxy = {
		base,
		setHook(fn, forward) {
			hook = fn;
			forwardToConsole = forward;
		},
		clearHook() {
			hook = null;
			forwardToConsole = false;
		},
	};
	globalThis.__jbtestConsoleProxy = proxy;
	return proxy;
}

export function withConsoleCapture({ name, run, emit, consoleProxy, forwardToConsole }) {
	consoleProxy.setHook(
		(level, args) => {
			const line = args.join(" ");
			const stream = level === "error" || level === "warn" ? "stderr" : "stdout";
			emit({ type: "test-output", name, line, stream, level });
		},
		forwardToConsole
	);

	try {
		run();
		return { ok: true };
	} catch (error) {
		return {
			ok: false,
			stack: error.stack
		};
	} finally {
		consoleProxy.clearHook();
	}
}
