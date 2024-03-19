use crate::AppState;
use askama::Template;
use axum::{extract::State, response::Html};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    blocks: Vec<Vec<Block>>,
}

struct Block {
    position: String,
    color: String,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut blocks = vec![];

    for y in 0..50 {
        let mut y_block = vec![];
        for x in 0..70 {
            y_block.push(Block {
                position: format!("{}:{}", y, x),
                color: String::from("grey"),
            });
        }

        blocks.push(y_block);
    }

    let template = IndexTemplate { blocks };

    Html(template.render().unwrap())
}
