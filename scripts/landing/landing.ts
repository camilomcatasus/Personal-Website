import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";



var term = new Terminal();
const fitAddon = new FitAddon();
term.options.fontSize = 16;
term.loadAddon(fitAddon);
term.open(document.getElementById('terminal'));
fitAddon.fit();
term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ')
