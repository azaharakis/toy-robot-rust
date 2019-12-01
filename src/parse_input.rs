use crate::commands::KnownCommands;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_input() {
        assert_eq!(parse_input(String::from("LEFT")), vec![KnownCommands::Left]);
        assert_eq!(
            parse_input(String::from("RIGHT")),
            vec![KnownCommands::Right]
        )
    }
}

pub fn parse_input(input: String) -> Vec<KnownCommands> {
    input
        .split(' ')
        .filter_map(|s| match s {
            "LEFT" => Some(KnownCommands::Left),
            "RIGHT" => Some(KnownCommands::Right),
            "MOVE" => Some(KnownCommands::Move),
            _ => None,
        })
        .collect()
}
