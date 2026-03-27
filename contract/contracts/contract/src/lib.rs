#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, Vec, Map};

#[contracttype]
#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub content: String,
    pub author: String,
}

#[contract]
pub struct KnowledgeBase;

#[contractimpl]
impl KnowledgeBase {

    // Add an article
    pub fn add_article(env: Env, id: Symbol, title: String, content: String, author: String) {
        let article = Article {
            title,
            content,
            author,
        };

        let mut storage: Map<Symbol, Article> =
            env.storage().instance().get(&Symbol::short("ART")).unwrap_or(Map::new(&env));

        storage.set(id, article);
        env.storage().instance().set(&Symbol::short("ART"), &storage);
    }

    // Get an article
    pub fn get_article(env: Env, id: Symbol) -> Option<Article> {
        let storage: Map<Symbol, Article> =
            env.storage().instance().get(&Symbol::short("ART")).unwrap_or(Map::new(&env));

        storage.get(id)
    }

    // List all article IDs
    pub fn list_articles(env: Env) -> Vec<Symbol> {
        let storage: Map<Symbol, Article> =
            env.storage().instance().get(&Symbol::short("ART")).unwrap_or(Map::new(&env));

        storage.keys()
    }
}