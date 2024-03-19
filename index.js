import { createClient } from "@libsql/client";

async function main() {
  const db_client = createClient({
    url: "libsql://color-blocks-kpovel.turso.io",
    authToken: "...",
  });

  for (let y = 0; y < 50; ++y) {
    for (let x = 0; x < 70; ++x) {
      await db_client.execute({
        sql: `insert into blocks (y, x, color_id)
values (:y, :x, (select id from available_colors where color = '#64748b'));`,
        args: {
          y,
          x,
        },
      });
    }
  }
  db_client.execute;
}

main();

//     let mut blocks = vec![];
//
//     for y in 0..50 {
//         let mut y_block = vec![];
//         for x in 0..70 {
//             y_block.push(Block {
//                 position: format!("{}:{}", y, x),
//                 color: String::from("grey"),
//             });
//         }
//
//         blocks.push(y_block);
//     }
//
