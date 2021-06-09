
export type SnippetInfo = {
    name: String,
    description: String
};



export function add_target(session_id: string, uri: string, language: string): void;

export function drop_target(session_id: string, uri: string, language: string): void;
//export function get_snippet(session_id: string, uri: string, snippet_key: string): Promise<[string]>;
export function get_snippet(session_id: string, uri: string, snippet_key: string): [string];
export function get_completions(session_id: string, uri: string): [SnippetInfo];

