// /"use strict";
import { promisify } from 'util';
import { get_triggers as _get_triggers, get_snippet as _get_snippet, drop_target as _drop_target } from '../index.node';
const get_triggers = promisify(_get_triggers);
const get_snippet = promisify(_get_snippet);

export default {
  get_triggers,
  get_snippet,
  add_target: _drop_target,
  drop_target: _drop_target
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

