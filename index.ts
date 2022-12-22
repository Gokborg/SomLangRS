import init, { compile, Kind, lex, wasm_to_wat } from "./pkg/somelang_rs.js";
export {}

await init();

const bytes = compile("a := 10;");
console.log(bytes);
console.log(wasm_to_wat(bytes));

const module = await WebAssembly.instantiate(bytes, {});
const exports = module.instance.exports
console.log(exports);
console.log(exports.f instanceof Function ? exports.f() : "no f");


// for (let i = 0; i < tokens.len(); i++) {
//     const token = tokens.get(i);
//     console.log(Kind[token.kind], token.lineno, token.start);
// }

