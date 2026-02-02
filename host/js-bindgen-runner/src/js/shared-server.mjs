const output = document.getElementById("output");
export function toOutput(text) {
    function createColor(color, text) {
        const span = document.createElement("span");
        span.style.color = color;
        span.textContent = text;
        output.appendChild(span);
    }
    const isAtBottom = Math.abs(output.scrollHeight - output.clientHeight - output.scrollTop) <= 1;
    for (const part of text) {
        switch (part.color) {
            case 0 /* Color.Default */:
                const node = document.createTextNode(part.text);
                output.appendChild(node);
                break;
            case 1 /* Color.Green */:
                createColor("green", part.text);
                break;
            case 2 /* Color.Yellow */:
                createColor("yellow", part.text);
                break;
            case 3 /* Color.Red */:
                createColor("red", part.text);
                break;
        }
    }
    if (isAtBottom) {
        output.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });
    }
}
