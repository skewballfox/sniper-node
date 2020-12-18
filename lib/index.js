// /"use strict";
const addon = require('../native/index.node');

const { Sniper } = addon;
let x = new Sniper("../../sniper-core/snippets");
console.log(x);

module.exports = addon;