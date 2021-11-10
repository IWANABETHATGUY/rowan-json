const fs = require('fs')
let json = fs.readFileSync("./assets/big.json").toString()


console.time('label')
JSON.parse(json)
console.timeEnd('label')


