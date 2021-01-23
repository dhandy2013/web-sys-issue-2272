self.importScripts("pkg/web_sys_issue_2272.js");

async function run() {
    console.log("worker.js:run(): Initializing the WASM module");
    await wasm_bindgen("pkg/web_sys_issue_2272_bg.wasm");
}

run();
