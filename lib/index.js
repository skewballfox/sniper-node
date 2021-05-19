"use strict";
"enable eslint";
const { promisify } = require('util');
const { add_target, drop_target, get_snippet: _get_snippet, get_triggers: _get_triggers } = require("../index.node");



module.exports =
{
  get_triggers: promisify(_get_triggers),
  get_snippet: promisify(_get_snippet),
  add_target,
  drop_target,
}



/*
class Sniper{
  constructor(){
    this.boxed=init()
  }
  add_target(session_id,uri,language){
    add_target(this.boxed,session_id,uri,language);
  }
}
let x=new Sniper();
console.log(x);
x.add_target("23456","test.py","python");
*/




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


