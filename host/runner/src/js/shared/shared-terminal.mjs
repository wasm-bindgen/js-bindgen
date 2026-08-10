export function keepAlive(promise) {
    // A pending `Atomics.waitAsync` does not keep a command-line event loop
    // alive: https://github.com/denoland/deno/issues/15358. Keep one timer active
    // until the complete test or binary run settles.
    const timer = globalThis.setInterval(() => undefined, 0x7fffffff);
    return promise.finally(() => globalThis.clearInterval(timer));
}
export function colorText(text) {
    const green = "\u001b[32m";
    const yellow = "\u001b[33m";
    const red = "\u001b[31m";
    const reset = "\u001b[0m";
    let output = "";
    for (const part of text) {
        switch (part.color) {
            case 0 /* Color.Default */:
                output += part.text;
                break;
            case 1 /* Color.Green */:
                output += `${green}${part.text}${reset}`;
                break;
            case 2 /* Color.Yellow */:
                output += `${yellow}${part.text}${reset}`;
                break;
            case 3 /* Color.Red */:
                output += `${red}${part.text}${reset}`;
                break;
        }
    }
    return output;
}
