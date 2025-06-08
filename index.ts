// ---
// bun init
// bun add figlet
// bun add -d @types/figlet
// bun index.ts
// ---
import figlet from "figlet";

const message = figlet.textSync("Star Atlas: Holosim Hackr");
console.log(message);
