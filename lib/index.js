// /"use strict";
var addon = require('../native/index.node');

const { Sniper } = addon;
let x = new Sniper("../../sniper-core/snippets", "python");
console.log(x);

module.exports = addon;