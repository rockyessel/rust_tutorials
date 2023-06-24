// //make struct

// struct Data {
//     num: i32,
//     num2: i32,
//     str1: String,
//     optional_num: Option<i32>,
// }

// struct TwoNums(i32, i32);

// #[derive(Debug)]
// struct Calculator;

// impl Data {
//     fn new() -> Self {
//         Data {
//             num: 0,
//             num2: 0,
//             str1: "default string".to_string(),
//             optional_num: Some(0),
//         }
//     }

//     fn sum(&self) -> i32 {
//         print!("{}", self.num);
//         self.num + 2
//     }

//     pub fn sqr_num(n1: i32) -> i32 {
//         n1 * n1
//     }
// }

// fn main() {
//     let mut a = Data {
//         num: 32,
//         num2: 34,
//         str1: "String".to_string(),
//         optional_num: Some(20),
//     };

//     let f = 3;
//     let f  = Data::sqr_num(54);
//     println!("Square: {}", f);

//     let d = TwoNums(32, 54);
//     println!("({}, {})", d.0, d.1)
// }

use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

const STYLE_FILE: &str = include_str!("main.css");

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let name = "d";
    let my_object = MyObject {
        username: "Rocky Essel".to_string(),
        favorite_language: "en".to_string(),
    };

    let style_sheet = style!(
        r#"
        
        h1 {
            color: red;
        }

        p {
            border-radius: 5px;
            width: fit-content;
            background-color: gray;
            padding: 5px 10px;
            transition: all 0.5s ease-in-out;
        }

        p:hover{
            transform: translate(-10px, -20px);
            box-shadow: 10px 10px 0 black;
        }

        ul {
            color: red;
        }

        "#
    )
    .unwrap();

    log!(name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class = "my_tile";
    let message: Option<&str> = None;

    let tasks_one = vec![
        html! {<li class={css!("color:black;")}>{"records"}</li>},
        html! {<li>{"noble"}</li>},
        html! {<li>{"news"}</li>},
        html! {<li>{"cards"}</li>},
    ];

    let tasks = vec![
        "class", "close", "cut", "cat", "conclude", "concern", "computer", "common", "course",
        "con",
    ];

    let style_e = Style::new(&STYLE_FILE).unwrap();

    html! {
        <main class={style_sheet}>
        <label class={class} for="">{"Name::"}</label>
        if class == "my_title" {
            <input type="text"/>
        }else{
            <p>{"it is not"}</p>
        }

        if let Some(message) = message {
            <p>{message}</p>
        }else{
            <p>{"No messages for today!"}</p>
        }


        <ul>
            <li style="font-weight: 900;">{"Render List in Array"}</li>
            {tasks_one}
        </ul>
        <hr/>
        <ul>
            <li style="font-weight: 900;">{"Render List using iter() map() collect()"}</li>
            {tasks.iter().map(| task | html! {<li>{task}</li>}).collect::<Html>()}
        </ul>
        <hr/>
        <ul>
            <li style="font-weight: 900;">{"Render List using iter() map() collect() but in separate function"}</li>
            {render_html_list(tasks)}
        </ul>

        <p>{"The Bokk"}</p>

    </main>
    }
}

fn render_html_list(lists: Vec<&str>) -> Html {
    return lists.iter().map(|item| html! {<li>{item}</li>}).collect();
}
