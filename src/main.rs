fn main() {
    println!("Hello, world!");
    let s = String::from("Hello, world");
    let s_disemvowel = disemvowel(&s);

    println!("s was '{}', and without vowels is '{}'.", s, s_disemvowel);
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

fn disemvowel(inputString: &str) -> String {
    let mut newString = String::new();
   
    for c in inputString.chars() {
        if isVowel(c) {
        }else {
        newString.push(c);
        }
    }
    return newString;
}

fn isVowel(c: char) -> bool {
    let vowels = "aAeEiIoOuU";
    let char_vec:Vec<char> = vowels.chars().collect();
    if c == char_vec[0]  ||
        c == char_vec[1] ||
        c == char_vec[2] ||
        c == char_vec[3] ||
        c == char_vec[4] ||
        c == char_vec[5] ||
        c == char_vec[6] ||
        c == char_vec[7] ||
        c == char_vec[8] ||
        c == char_vec[9] {
        return true;
        } else {return false;}
}

// Everything from here down is Rust test code. You shouldn't need to 
// change any of this. 
//
// Use `cargo test` to run all these tests. All the tests will initially 
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let input = "Hello, world!";
        let expected = "Hll, wrld!";

        assert_eq!(
            expected,
            disemvowel(input)
        );
    }

    #[test]
    fn empty() {
        assert_eq!("", disemvowel(""));
    }

    #[test]
    fn no_vowels() {
        assert_eq!("pqrst", disemvowel("pqrst"));
    }

    #[test]
    fn all_vowels() {
        assert_eq!("", disemvowel("aeiouAEIOUOIEAuoiea"));
    }

    #[test]
    fn morris_minnesota() {
        assert_eq!("Mrrs, Mnnst", disemvowel("Morris, Minnesota"));
    }

    #[test]
    fn handle_punctuation() {
        assert_eq!("n (nxplnd) lphnt!", 
            disemvowel("An (Unexplained) Elephant!"));
    }

    #[test]
    fn handle_unicode() {
        assert_eq!("Sm hrglyphs: 𒐁	𒐌	𒐥	𒑳",
            disemvowel("Some hieroglyphs: 𒐁	𒐌	𒐥	𒑳"));
        assert_eq!("Sm Lnr B: 	𐂀	𐂚	𐃃	𐃺",
            disemvowel("Some Linear B: 	𐂀	𐂚	𐃃	𐃺"));
        assert_eq!(" lttl Phncn: 𐤀	𐤈	𐤔	𐤕",
            disemvowel("A little Phoenician: 𐤀	𐤈	𐤔	𐤕"));
        assert_eq!("W cn hndl mj s wll! 🤣😃👍",
            disemvowel("We can handle emoji as well! 🤣😃👍"))
    }
}