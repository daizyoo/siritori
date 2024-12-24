mod default;

fn main() {
    let mut content: Vec<String> = vec![];

    let mut player = true;

    let first_section = "しりとり".to_string();
    let mut previous_last = last(&first_section);

    println!("{}", first_section);
    content.push(first_section);

    loop {
        println!("プレイヤー: {}", if player { 1 } else { 2 });
        let section = input();

        if section.is_empty() || !check(&section, previous_last) {
            println!("{}で始まる単語を入力してください", previous_last);
            continue;
        }

        if game_over_check(&section) {
            println!("`ん`がついたので負けです");
            break;
        }

        previous_last = last(&section);

        content.push(section);
        player = !player;
    }
    println!("プレイヤー{}の負けです", if player { 1 } else { 2 });
}

fn game_over_check(section: &str) -> bool {
    section.ends_with('ん')
}

fn check(section: &str, previous_last: char) -> bool {
    section.starts_with(previous_last)
}

fn last(section: &str) -> char {
    section.chars().last().unwrap()
}

fn input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}
