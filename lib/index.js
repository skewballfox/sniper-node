// /"use strict";
const addon = require('../index.node');

//const { Sniper } = addon;
class SniperNode {
    constructor(config) {
        this.boxed = addon.startSniper(config);
    }
}
let x = new SniperNode("snippets");

console.log(x)
console.log(typeof (x));
//console.log(x.get('language'));
//console.log(process.version);

module.exports = addon;

