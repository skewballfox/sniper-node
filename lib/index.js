// /"use strict";
const addon = require('../native/index.node');

const { Sniper } = addon;
let x = new Sniper("snippets", "python");

console.log(x)
console.log(typeof (x));
console.log(x.get('language'));
console.log(process.version);

module.exports = addon;