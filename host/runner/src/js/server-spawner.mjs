import { workerKind } from "./shared-spawner.mjs";
import { toOutput } from "./shared-server.mjs";
switch (workerKind) {
    case 0 /* WorkerKind.Dedicated */: {
        const worker = new Worker("./worker.mjs", { type: "module" });
        worker.addEventListener("message", event => {
            toOutput(event.data);
        });
        break;
    }
    case 1 /* WorkerKind.Shared */: {
        const worker = new SharedWorker("./worker.mjs", { name: crypto.randomUUID(), type: "module" });
        worker.port.addEventListener("message", event => {
            toOutput(event.data);
        });
        worker.port.start();
        break;
    }
    case 2 /* WorkerKind.Service */: {
        function install(worker) {
            const channel = new MessageChannel();
            worker.postMessage(undefined, [channel.port2]);
            channel.port1.addEventListener("message", event => {
                toOutput(event.data);
            });
            channel.port1.start();
        }
        const registration = await navigator.serviceWorker.register("./worker.mjs", {
            scope: crypto.randomUUID(),
            type: "module",
            updateViaCache: "none",
        });
        let worker = registration.installing || registration.waiting || registration.active;
        if (worker) {
            if (worker.state === "activated") {
                install(worker);
            }
            else {
                worker.addEventListener("statechange", () => {
                    if (worker.state === "activated")
                        install(worker);
                });
            }
        }
        else {
            registration.addEventListener("updatefound", () => {
                worker = registration.installing;
                worker.addEventListener("statechange", () => {
                    if (worker.state === "activated")
                        install(worker);
                });
            });
        }
    }
}
