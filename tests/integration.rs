use sully_peg::peg;

#[test]
fn test_char_lit() {
    peg! {
        x = 'x'
    }
    let input = Input::new("x");
    assert_eq!(x(input), Some((input.advance(1), ())));
}

#[test]
fn test_str_lit() {
    peg! {
        x = "Hello"
    }
    let input = Input::new("Hello, there!");
    assert_eq!(x(input), Some((input.advance(5), ())));
}

#[test]
fn test_char_range() {
    peg! {
        x = '0'..='9'
    }
    let input = Input::new("1234");
    assert_eq!(x(input), Some((input.advance(1), ())));
}

#[test]
fn test_call() {
    peg! {
        x = '0'..='9'
        y = x
    }
    let input = Input::new("1234");
    assert_eq!(y(input), Some((input.advance(1), ())));
}

#[test]
fn test_cat() {
    peg! {
        x = '0' '1'
    }
    let input = Input::new("0101");
    assert_eq!(x(input), Some((input.advance(2), ())));
}

#[test]
fn test_many0() {
    peg! {
        x = '0'..='9'
        y = x*
    }
    let input = Input::new("1234");
    assert_eq!(y(input), Some((input.advance(4), ())));

    let input = Input::new("");
    assert_eq!(y(input), Some((input, ())));
}

#[test]
fn test_many1() {
    peg! {
        x = '0'..='9'
        y = x+
    }
    let input = Input::new("1234");
    assert_eq!(y(input), Some((input.advance(4), ())));

    let input = Input::new("");
    assert_eq!(y(input), None);
}

#[test]
fn test_optional() {
    peg! {
        x = '0'..='9'
        y = x x?
    }
    let input = Input::new("1234");
    assert_eq!(y(input), Some((input.advance(2), ())));

    let input = Input::new("1");
    assert_eq!(y(input), Some((input.advance(1), ())));

    let input = Input::new("");
    assert_eq!(y(input), None);
}

#[test]
fn test_pos() {
    peg! {
        x = 'a'..='z'
        y = &x "a1234"
    }
    let input = Input::new("a1234");
    assert_eq!(y(input), Some((input.advance(5), ())));

    let input = Input::new("asdf");
    assert_eq!(y(input), None);
}

#[test]
fn test_neg() {
    peg! {
        x = 'a'..='z'
        y = !x "1234"
    }
    let input = Input::new("1234");
    assert_eq!(y(input), Some((input.advance(4), ())));

    let input = Input::new("asdf");
    assert_eq!(y(input), None);
}

#[test]
fn test_implicit_space() {
    peg! {
        x = '0'..='9'
        y = x x x x
    }
    let input = Input::new("12 34");
    assert_eq!(y(input), Some((input.advance(5), ())));

    let input = Input::new("1 2 3 4 5");
    assert_eq!(y(input), Some((input.advance(7), ())));
}

#[test]
fn test_implicit_space_many0() {
    peg! {
        x = '0'..='9'
        y = x*
    }
    let input = Input::new("12 3 4 5");
    assert_eq!(y(input), Some((input.advance(8), ())));

    let input = Input::new("");
    assert_eq!(y(input), Some((input, ())));
}

#[test]
fn test_implicit_space_many1() {
    peg! {
        x = '0'..='9'
        y = x+
    }
    let input = Input::new("12 3 4 5");
    assert_eq!(y(input), Some((input.advance(8), ())));

    let input = Input::new("");
    assert_eq!(y(input), None);
}

#[test]
fn test_atomic() {
    peg! {
        x = '0'..='9'
        y = x+
        z = @y
    }
    let input = Input::new("12 3 4 5");
    assert_eq!(y(input), Some((input.advance(8), ())));
    assert_eq!(z(input), Some((input.advance(2), ())));
}

#[test]
fn test_action() {
    peg! {
        x = '0'..='9'
        y = x+
        z -> ((), ()) = r: y { (r, r) }
    }
    let input = Input::new("12 3 4 5");
    assert_eq!(y(input), Some((input.advance(8), ())));
    assert_eq!(z(input), Some((input.advance(8), ((), ()))));
}

#[test]
fn test_span() {
    peg! {
        x = '0'..='9'
        y = x+
        z -> Span<'a> = r: $y { r.0 }
    }
    
    let s = "12 3 4 5";
    let input = Input::new(s);
    let span = Span::new(s, 0, s.len());
    assert_eq!(y(input), Some((input.advance(8), ())));
    assert_eq!(z(input), Some((input.advance(8), span)));
}

#[test]
fn test_left_rec() {
    peg! {
        x = '0'..='9'
        y -> (Span<'a>, ()) = r: $(y x | x) { r }
    }

    let s = "1 2 3";
    let input = Input::new(s);
    let span = Span::new(s, 0, 5);
    assert_eq!(y(input), Some((input.advance(5), (span, ()))));
}

#[test]
fn test_left_rec_extra() {
    peg! {
        x = '0'..='9'
        y = y x | x
        z -> (Span<'a>, ()) = r: $y { r }
    }

    let s = "1 2 3";
    let input = Input::new(s);
    let span = Span::new(s, 0, 5);
    assert_eq!(y(input), Some((input.advance(5), ())));
    assert_eq!(z(input), Some((input.advance(5), (span, ()))));
}

#[test]
fn test_many0_vector() {
    peg! {
        x = '0'..='9'
        y -> Vec<(Span<'a>, ())> = r: ($x)* { r }
    }

    let s = "1 2 3";
    let input = Input::new(s);
    assert_eq!(y(input), Some((input.advance(5), vec![(Span::new(s, 0, 1), ()), (Span::new(s, 2, 3), ()), (Span::new(s, 4, 5), ())])));
}

#[test]
fn test_many1_vector() {
    peg! {
        x = '0'..='9'
        y -> Vec<(Span<'a>, ())> = r: ($x)+ { r }
    }

    let s = "1 2 3";
    let input = Input::new(s);
    assert_eq!(y(input), Some((input.advance(5), vec![(Span::new(s, 0, 1), ()), (Span::new(s, 2, 3), ()), (Span::new(s, 4, 5), ())])));
}

#[test]
fn test_optional_vector() {
    peg! {
        x = '0'..='9'
        y -> Option<(Span<'a>, ())> = r: ($x)? { r }
    }

    let s = "1";
    let input = Input::new(s);
    assert_eq!(y(input), Some((input.advance(1), Some((Span::new(s, 0, 1), ())))));

    let s = "";
    let input = Input::new(s);
    assert_eq!(y(input), Some((input, None)));
}

#[test]
fn test_delims() {
    peg! {
        x = '0'..='9'
        y -> Span<'a> = r: $@(x+) { r.0 }
        comma -> Span<'a> = r: $',' { r.0 }
        z -> (Vec<Span<'a>>, Vec<Span<'a>>)
            = r: y ^ comma { r }
    }

    let s = "10, 20, 30";
    let input = Input::new(s);
    assert_eq!(z(input), Some((input.advance(s.len()), (
        vec![
            Span::new(s, 0, 2),
            Span::new(s, 4, 6),
            Span::new(s, 8, 10),
        ],
        vec![
            Span::new(s, 2, 3),
            Span::new(s, 6, 7),
        ],
    ))));
}
