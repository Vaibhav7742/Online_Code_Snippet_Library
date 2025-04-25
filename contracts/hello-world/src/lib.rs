#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Snippet {
    pub id: u64,
    pub title: String,
    pub code: String,
    pub language: String,
    pub creator: String,
}

#[contracttype]
pub enum SnippetBook {
    Snip(u64),
}

const COUNT_SNIP: Symbol = symbol_short!("C_SNIP");

#[contract]
pub struct CodeLibraryContract;

#[contractimpl]
impl CodeLibraryContract {
    // Function to add a new code snippet
    pub fn add_snippet(env: Env, title: String, code: String, language: String, creator: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&COUNT_SNIP).unwrap_or(0);
        count += 1;

        let snippet = Snippet {
            id: count,
            title,
            code,
            language,
            creator,
        };

        env.storage().instance().set(&SnippetBook::Snip(count), &snippet);
        env.storage().instance().set(&COUNT_SNIP, &count);
        count
    }

    // Function to view a snippet by ID
    pub fn view_snippet(env: Env, id: u64) -> Snippet {
        env.storage().instance().get(&SnippetBook::Snip(id)).unwrap_or(Snippet {
            id: 0,
            title: String::from_str(&env, "Not Found"),
            code: String::from_str(&env, "Not Found"),
            language: String::from_str(&env, "Unknown"),
            creator: String::from_str(&env, "Unknown"),
        })
    }

    // Function to count how many snippets are stored
    pub fn snippet_count(env: Env) -> u64 {
        env.storage().instance().get(&COUNT_SNIP).unwrap_or(0)
    }
}