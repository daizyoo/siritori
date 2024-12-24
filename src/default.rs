use crate::input;

pub fn main() {
    let mut content: Vec<String> = vec![];

    let mut player = true;

    let first_section = "しりとり".to_string();
    let mut previous_last = last(&first_section);

    println!("{}", first_section);
    content.push(first_section);

    loop {
        player = !player;

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
    }
    println!("プレイヤー{}の負けです", if player { 1 } else { 2 });
}

fn game_over_check(section: &str) -> bool {
    let last = last(section);
    if last == 'ん' {
        return true;
    } else {
        return false;
    }
}

fn check(section: &str, previous_last: char) -> bool {
    let chars: Vec<char> = section.chars().collect();

    if chars[0] == previous_last {
        return true;
    } else {
        return false;
    }
}

fn last(section: &str) -> char {
    let chars: Vec<char> = section.chars().collect();
    chars[chars.len() - 1]
}
