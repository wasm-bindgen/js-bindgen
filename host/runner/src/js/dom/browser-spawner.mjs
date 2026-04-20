import { workerKind } from "./shared-spawner.mjs";
switch (workerKind) {
    case 0 /* WorkerKind.Dedicated */:
        new Worker("../worker/script.mjs", { type: "module" });
        break;
    case 1 /* WorkerKind.Shared */: {
        new SharedWorker("../worker/script.mjs", { name: crypto.randomUUID(), type: "module" });
        break;
    }
    case 2 /* WorkerKind.Service */:
        await navigator.serviceWorker
            .register("../worker/script.mjs", {
            scope: "./worker/" + crypto.randomUUID(),
            type: "module",
            updateViaCache: "none",
        })
            .catch(async (error) => {
            await fetch("../report", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    order: 0,
                    report: { stream: 1 /* Stream.Stderr */, line: error.message + "\n" },
                }),
            }).finally(() => fetch("../finished", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ status: 2 /* Status.Abnormal */, messages: 1 }),
            }));
        });
}
