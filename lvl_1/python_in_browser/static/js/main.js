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
editor.setValue(`
from typing import Optional


print(f"{target_pos=}")
print(f"{unit_pos=}")

def function(
    target_pos: (float, float) = None,
    unit_pos: (float, float) = None
    ) -> Optional[str]:
    """
    @param target_pos: 
    @return: 
    """

    return 0, "Shoot"

`);
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
  output.value = pyodide.runPython(`
import sys
sys.version
  `);
  output.value += `
Python ${output.value.split(" ")[0]}

`;
  return pyodide;
}
// run the main funciton
let pyodideReadyPromise = main();

function setParameter(name, value) {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    urlParams.set(name, value);
    const newurl = window.location.protocol
        + "//" + window.location.host
        + window.location.pathname
        + '?' + urlParams;
    window.history.pushState({path:newurl},'',newurl);
}

function getParameterValue(name) {
    const queryString = window.location.search;
    const urlParams = new URLSearchParams(queryString);
    return decodeURI(urlParams.get(name));
}

// pass the editor value to the pyodide.runPython function and show the result in the output section
async function evaluatePython() {
    console.log('window.location.search = ' + window.location.search);
    let target_pos = getParameterValue('target_pos');
    let unit_pos = getParameterValue('unit_pos');
    // console.log("unit_pos = " + unit_pos);

    let pyodide = await pyodideReadyPromise;
    try {
        pyodide.runPython(`    
import io
sys.stdout = io.StringIO()


target_pos = ${target_pos}
unit_pos = ${unit_pos}
        `);
        pyodide.runPython(`
${editor.getValue()}    
print(function(target_pos, unit_pos))
        `);
        let stdout = pyodide.runPython("sys.stdout.getvalue()");
        addToOutput(stdout);

        // // get rotation
        // pyodide.runPython("print(rotation)");
        // let rotation = pyodide.runPython("sys.stdout.getvalue()");
        // setParameter("rotation", rotation.toString().trim());
        //
        // // get command
        // pyodide.runPython("print(command)");
        // let command = pyodide.runPython("sys.stdout.getvalue()");
        // setParameter("command", command.toString().trim());
  } catch (err) {
    addToOutput(err);
  }
}