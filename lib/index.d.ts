
/*
declare class Sniper {
    ['constructor']: typeof Sniper;
    constructor(config?: string | undefined);

    get(attribute: string): string;
}
*/

export function add_target(session_id: string, uri: string, language: string): void;

export function drop_target(session_id: string, uri: string, language: string): void;
export function get_snippet(language: string, snippet_key: string): Promise<[string]>;
export function get_triggers(session_id: string, uri: string): Promise<[string]>;

