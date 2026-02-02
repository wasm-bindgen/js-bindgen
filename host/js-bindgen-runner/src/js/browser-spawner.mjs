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
        await navigator.serviceWorker.register("./worker.mjs", {
            scope: crypto.randomUUID(),
            type: "module",
            updateViaCache: "none",
        });
}
