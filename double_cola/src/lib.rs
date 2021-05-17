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
fn who_is_next(names: &Names, mut n: usize) -> Name {
    let mut double = 1;
    while 5 * double < n {
        n -= 5 * double;
        double *= 2;
    }
    let index = (n - 1) / double;
    *names.iter().nth(index).unwrap()
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
        assert_eq!(who_is_next(names, 6), Name::Sheldon);
        assert_eq!(who_is_next(names, 7), Name::Sheldon);
        assert_eq!(who_is_next(names, 8), Name::Leonard);
        assert_eq!(who_is_next(names, 30), Name::Howard);
        assert_eq!(who_is_next(names, 31), Name::Sheldon);
        assert_eq!(who_is_next(names, 52), Name::Penny);
        assert_eq!(who_is_next(names, 7_230_702_951), Name::Leonard);
    }
}
