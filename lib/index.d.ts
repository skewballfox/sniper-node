

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

export as namespace sniper;
//export =add_target, drop_target, get_snippet, get_triggers;
export declare function add_target(session_id: string, uri: string, language: string): void;
export declare function drop_target(session_id: string, uri: string, language: string): void;
export declare function get_snippet(language: string, snippet_key: string): Promise<Array<string>>;
export declare function get_triggers(session_id: string, uri: string): Promise<Array<string>>;


//export =sniper;

