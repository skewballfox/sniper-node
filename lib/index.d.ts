

/*
declare class Sniper {
    ['constructor']: typeof Sniper;
    constructor(config?: string | undefined);

    get(attribute: string): string;
}
*/
//let snipe = new Sniper("../snippets");
//export = Sniper;


//import addon = require('../index.node');
//import { promisify } from 'util';

export as namespace sniper;
//export =add_target, drop_target, get_snippet, get_triggers;
export function add_target(session_id: string, uri: string, language: string): void;

export function drop_target(session_id: string, uri: string, language: string): void;
export function get_snippet(language: string, snippet_key: string): Promise<[string]>;
export function get_triggers(session_id: string, uri: string): Promise<[string]>;

// export = {
//     add_target,
//     drop_target,
//     get_snippet,
//     get_triggers,
// }
// declare function add_target(session_id: string, uri: string, language: string): void;
// declare function drop_target(session_id: string, uri: string, language: string): void;
// declare function get_snippet(language: string, snippet_key: string): Promise<[string]>;
// declare function get_triggers(session_id: string, uri: string): Promise<[string]>;

//export =sniper;

