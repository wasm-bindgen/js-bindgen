function installConsoleHook() {
	const existing = globalThis.__jbtestConsoleHook;
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

	const h = {
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
	globalThis.__jbtestConsoleHook = h;
	return h;
}

export function withConsoleCapture({ name, run, emit, consoleHook, forwardToConsole }) {
	consoleHook.setHook(
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
		consoleHook.clearHook();
	}
}

export default installConsoleHook();
