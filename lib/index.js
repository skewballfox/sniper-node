"use strict";
"enable eslint";
const { promisify } = require('util');
const { add_target, drop_target, get_snippet, get_completions } = require("../index.node");



module.exports =
{
  get_completions,
  //get_snippet: promisify(_get_snippet),
  get_snippet,
  add_target,
  drop_target,
}


