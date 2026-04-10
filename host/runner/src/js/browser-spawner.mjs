import { workerKind } from "./shared-spawner.mjs";
switch (workerKind) {
    case 0 /* WorkerKind.Dedicated */:
        new Worker("./worker.mjs", { type: "module" });
        break;
    case 1 /* WorkerKind.Shared */: {
        new SharedWorker("./worker.mjs", { name: crypto.randomUUID(), type: "module" });
        break;
    }
    case 2 /* WorkerKind.Service */:
        await navigator.serviceWorker
            .register("./worker.mjs", {
            scope: crypto.randomUUID(),
            type: "module",
            updateViaCache: "none",
        })
            .catch(async (error) => {
            await fetch("./report", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    order: 0,
                    stream: 1 /* Stream.Stderr */,
                    line: error.message + "\n",
                }),
            }).finally(() => fetch("/finished", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(2 /* Status.Abnormal */),
            }));
        });
}
