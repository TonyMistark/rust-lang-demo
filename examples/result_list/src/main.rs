use std::fmt::Error;

#[derive(Debug)]
struct Row {
    id: u8
}


fn fetch_chunk(from: u8, to: u8) -> Result<Row, Error> {
    let r: Row = Row {id: from + to};

    if from > 100 {
        panic!("out scope")
    } else {
        Ok(r)
    }

}

fn main() {
    let indexes = [1,2,3,4, 100];
    let chunks: Vec<Result<Row, Error>> = indexes.iter().map(|i| fetch_chunk(*i, i + 1)).collect();
    println!("chunks: {:?}", chunks);
    // chunks: [Ok(Row { id: 3 }), Ok(Row { id: 5 }), Ok(Row { id: 7 }), Ok(Row { id: 9 }), Ok(Row { id: 201 })]
    for chunk in chunks {
        match chunk {
            Ok(row) => {println!("row.id: {}", row.id)},
            _ => println!("skip"),
        }
    }

    let chunks: Result<Vec<Row>, Error> = indexes.iter().map(|i| fetch_chunk(*i, i + 1)).collect();
    println!("chunks: {:?}", chunks);
    // chunks: Ok([Row { id: 3 }, Row { id: 5 }, Row { id: 7 }, Row { id: 9 }, Row { id: 201 }])
    match chunks {
        Ok(rs) => {
            for r in rs {
                println!("row.id: {}", r.id);
            }
        },
        _ => {println!("skip")},
    };
}
