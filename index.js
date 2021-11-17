const fs = require("fs");

let json = fs.readFileSync("./assets/big.json").toString();

console.time("parse");
let obj = JSON.parse(json);
console.timeEnd("parse");

console.time("stringify");
JSON.stringify(obj);
console.timeEnd("stringify");
