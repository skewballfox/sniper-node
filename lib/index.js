"use strict";
"enable eslint";
const { promisify } = require('util');
const { add_target, drop_target, get_snippet: _get_snippet, get_triggers: _get_triggers } = require("../index.node");
//const { sniper } = require(newLocal);
//const get_triggers = promisify(get_triggers);
//const get_snippet = promisify(get_snippet);
// function sniper() {
//   this.get_triggers = promisify(_get_triggers);
//   this.get_snippet = promisify(_get_snippet);
//   this.add_target = add_target;
//   this.drop_target = drop_target;

// }
module.exports = //sniper;
{
  get_triggers: promisify(_get_triggers),
  get_snippet: promisify(_get_snippet),
  add_target,//: add_target,
  drop_target,//: drop_target
}


//const { Sniper } = addon;



/*
class Sniper{
  constructor(){
    this.boxed=SniperClient.init()
  }
  add_target(session_id,uri,language){
    SniperClient.add_target(this.boxed,session_id,uri,language);
  }
}
let x=new Sniper();
console.log(x);
x.add_target("23456","test.py","python");
*/

//SniperClient.add_target("23456","test.py","python");


// Example
// (async () => {
//     let trigs = await get_triggers("23456","test.py");

//     //trigs.forEach(function(entry) {
//     //  console.log(entry);
//     //});
//     console.log(trigs)
// })();
// (async () => {
//   let snippet = await get_snippet("python","if");


//     console.log(snippet);

// })();

//SniperClient.drop_target("12345","test.py","python");
// console.log(typeof (node));
// *


// Example
// (async () => {
//     const node = await connectAsync();

//     console.log(node);
// })();
//console.log(x.get('language'));
//console.log(process.version);

//export default addon;

