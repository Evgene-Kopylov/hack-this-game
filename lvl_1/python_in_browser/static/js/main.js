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

const function_name = "unit_rotation";

// set the initial value of the editor
editor.setValue(`import numpy as np

def ${function_name}(
    target_pos: (int, int),
    unit_pos: (int, int)
) -> float:
    """
    Расчёт угла поворота по двум точкам.
    
    @param target_pos: координаты мишени 
    @param unit_pos: координаты юнита 
    @return: угол поворота в градусах.
    """
    return np.random.randint(-180, +180)
`);
output.value = "Initializing...\n";

// Add pyodide returned value to the output
function addToOutput(stdout) {
  output.value += stdout;
  output.scrollTop = output.scrollHeight;
}

// Clean the output section
function clearHistory() {
  output.value = "";
}

// init Pyodide and show sys.version when it's loaded successfully
async function main() {
  let pyodide = await loadPyodide();
  await pyodide.loadPackage('numpy');

    output.value = pyodide.runPython(`
import sys
sys.version
  `);
  output.value += `
Python ${output.value.split(" ")[0]}

`;

  // Версия Python в заголовке IDE
  let element = window.document.getElementById("python_version");
  element.innerText = output.value.split(" ")[0];

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
    let target_pos = getParameterValue('target_pos');
    let unit_pos = getParameterValue('unit_pos');

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
rotation = ${function_name}(target_pos, unit_pos)
print(rotation)
# print(command)
        `);
        let stdout = pyodide.runPython("sys.stdout.getvalue()");
        let result = stdout.toString().trim().split("\n")
        addToOutput(`>>> ${function_name}(${target_pos}, ${unit_pos})\n` + stdout);

        let rotation = result[0];
        setParameter("rotation", rotation.toString().trim());

        setParameter("command", "Shoot");
  } catch (err) {
    addToOutput(err);
  }
}