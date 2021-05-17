/// _BigBang_ gang
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Name {
    Sheldon,
    Leonard,
    Penny,
    Rajesh,
    Howard,
}

/// Just to make code look a bit nicer
type Names = Vec<Name>;

/// Will return the `Name` of the person who will drink the `n`-th cola.
fn who_is_next(names: &Names, n: usize) -> Name {
    let len = names.len();
    let mut r = n - 1;
    while r >= len {
        r = (r - len) / 2;
    }
    *names.iter().nth(r).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let names = &vec![
            Name::Sheldon,
            Name::Leonard,
            Name::Penny,
            Name::Rajesh,
            Name::Howard,
        ];

        assert_eq!(who_is_next(names, 1), Name::Sheldon);
        assert_eq!(who_is_next(names, 5), Name::Howard);
        assert_eq!(who_is_next(names, 6), Name::Sheldon);
        assert_eq!(who_is_next(names, 15), Name::Howard);
        assert_eq!(who_is_next(names, 16), Name::Sheldon);
        assert_eq!(who_is_next(names, 35), Name::Howard);
        assert_eq!(who_is_next(names, 36), Name::Sheldon);
        assert_eq!(who_is_next(names, 52), Name::Penny);
        assert_eq!(who_is_next(names, 7_230_702_951), Name::Leonard);
    }
}
