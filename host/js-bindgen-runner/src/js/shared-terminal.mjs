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
