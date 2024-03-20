use crate::AppState;
use askama::Template;
use axum::{extract::State, response::Html};
use libsql::Error;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    blocks: Vec<Vec<Block>>,
    colors: Vec<String>,
}

struct Block {
    y: u32,
    x: u32,
    color: String,
}

pub async fn index(State(state): State<Arc<AppState>>) -> Html<String> {
    let blocks = color_blocks(Arc::clone(&state)).await;
    let blocks = match blocks {
        Ok(b) => b,
        Err(e) => {
            println!("{}", e);
            return Html(String::from("Error during receiving game state"));
        }
    };

    let blocks: Vec<Vec<Block>> = blocks
        .into_iter()
        .fold(vec![], |mut acc, curr_block| match acc.last_mut() {
            Some(last_y) => {
                if let Some(last_x) = last_y.last() {
                    if curr_block.x < last_x.x {
                        acc.push(vec![curr_block]);
                        return acc;
                    } else {
                        last_y.push(curr_block);
                        return acc;
                    }
                }
                unreachable!();
            }
            None => {
                acc.push(vec![curr_block]);
                return acc;
            }
        });

    let colors = match available_colors(Arc::clone(&state)).await {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return Html(String::from("Error during receiving available colors"));
        }
    };

    let template = IndexTemplate { blocks, colors };

    Html(template.render().unwrap())
}

async fn color_blocks(state: Arc<AppState>) -> Result<Vec<Block>, Error> {
    let query = "\
select y, x, color \
from blocks \
         inner join available_colors ac on blocks.color_id = ac.id;";

    let mut rows = state.db_conn.query(query, ()).await?;
    let mut blocks = vec![];

    while let Ok(Some(row)) = rows.next().await {
        let block = Block {
            y: row.get(0)?,
            x: row.get(1)?,
            color: row.get(2)?,
        };

        blocks.push(block);
    }

    Ok(blocks)
}

async fn available_colors(state: Arc<AppState>) -> Result<Vec<String>, Error> {
    let query = "\
select color \
from available_colors;";

    let mut rows = state.db_conn.query(query, ()).await?;
    let mut colors = vec![];

    while let Ok(Some(row)) = rows.next().await {
        colors.push(row.get::<String>(0)?);
    }

    Ok(colors)
}
