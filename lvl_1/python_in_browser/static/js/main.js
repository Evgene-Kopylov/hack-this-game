// find the output element
const output = document.getElementById("output");
// initializing the codemirror and pass configuration to support python and dracula theme
const editor = CodeMirror.fromTextArea(document.getElementById("code"), {
              mode: {
                  name: "python",
                  version: 3,
                  singleLineStringErrors: false,
              },
              theme: "dracula",
              lineNumbers: true,
              indentUnit: 4,
              matchBrackets: true,
            });
// set the initial value of the editor
editor.setValue(
`
from typing import Optional

def function(target_pos: (float, float) = None) -> Optional[str]:
    """
    @param target_pos: 
    @return: 
    """
    return "Shoot"
`
);
output.value = "Initializing...\n";

// Add pyodide returned value to the output
function addToOutput(stdout) {
  output.value += ">>>\n" + stdout + "\n";
}

// Clean the output section
function clearHistory() {
  output.value = "";
}

// init Pyodide and show sys.version when it's loaded successfully
async function main() {
  let pyodide = await loadPyodide();
  output.value = pyodide.runPython(
`
import sys
sys.version
`
  );
  output.value += `
Python ${output.value.split(" ")[0]}

`;
  return pyodide;
}
// run the main funciton
let pyodideReadyPromise = main();

// pass the editor value to the pyodide.runPython function and show the result in the output section
async function evaluatePython() {
    let wasm_says = decodeURI(window.location.hash);
    // console.log(`js: raw_wasm_say: ${wasm_says}`);
    wasm_says = wasm_says.split(": ").slice(1);
    // console.log(`js: wasm_says: ${wasm_says}`);

    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    const target_pos = decodeURI(urlParams.get('target_pos'));
    console.log('target_pos = ' + target_pos);

    let pyodide = await pyodideReadyPromise;
    try {
        pyodide.runPython(
`
import io
sys.stdout = io.StringIO()

target_pos = ${target_pos}      
`
        );
        pyodide.runPython(
`
${editor.getValue()}    
print(function(target_pos))
`
        );
        let stdout = pyodide.runPython("sys.stdout.getvalue()");
        addToOutput(stdout);
        window.location.hash = `js_says: ${stdout.toString()}`;
        // console.log(`js_says: ${js_says}`);
  } catch (err) {
    addToOutput(err);
  }
}