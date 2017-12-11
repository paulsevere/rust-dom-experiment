const wasm = require('./main.rs');
window.cool = console.log;
wasm.initialize({ noExitRuntime: true }).then(module => {
  let exec = module.cwrap('exec', null, []);
  window.mod = module;
  console.log(module.wasmMemory);
  exec();
  // console.log(hello_world(4, 6));
});
