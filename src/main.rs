use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name : String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}



fn main()
{
let article : Article = Article{
    article: String::from("Writing json in Rust"),
    author: String::from("Arka"),
    paragraph: vec![
        Paragraph{
            name: String::from("First Sentence")
        },
        Paragraph{
            name: String::from("Second Sentence")
        },
        Paragraph{
            name: String::from("Third Sentence")
        },
        Paragraph{
            name: String::from("Fourth Sentence")
        },
        Paragraph{
            name: String::from("Fifth Sentence")
        },
    ]
 };

let json = serde_json::to_string(&article).unwrap();
println!("the json is: {}", json);

}