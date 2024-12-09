import { createInterface } from "readline";
// Create an interface for reading from stdin
const rl = createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
});
// Listen for the 'line' event to read each line
rl.on("line", (line) => {
    // Print the line read from stdin
    console.log(line);
});
