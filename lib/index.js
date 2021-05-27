"use strict";
"enable eslint";
const { promisify } = require('util');
const { add_target, drop_target, get_snippet: _get_snippet, get_completions: _get_completions } = require("../index.node");



module.exports =
{
  get_completions: promisify(_get_completions),
  get_snippet: promisify(_get_snippet),
  add_target,
  drop_target,
}


// Example
// (async () => {
//     let completions = await get_completions("23456","test.py","i");

//     //completions.forEach(function(entry) {
//     //  console.log(entry);
//     //});
//     console.log(trigs)
// })();
// (async () => {
//   let snippet = await get_snippet("python","if");


//     console.log(snippet);

// })();


