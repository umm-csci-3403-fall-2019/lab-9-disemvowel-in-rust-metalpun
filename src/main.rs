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
    let strlength = inputString.len();
    let mut newString = String::new();
    let text = "aAeEiIoOuU";
    let char_vec:Vec<char> = text.chars().collect();
    let leta = char_vec[0];
    let letA = char_vec[1];
    let lete = char_vec[2];
    let letE = char_vec[3];
    let leti = char_vec[4];
    let letI = char_vec[5];
    let leto = char_vec[6];
    let letO = char_vec[7];
    let letu = char_vec[8];
    let letU = char_vec[9];
    let mut inputString2 = inputString;

    for i in 0..strlength {
        if inputString.chars().next().unwrap() == leta ||
        inputString2.chars().next().unwrap() == letA ||
        inputString2.chars().next().unwrap() == lete ||
        inputString2.chars().next().unwrap() == letE ||
        inputString2.chars().next().unwrap() == leti ||
        inputString2.chars().next().unwrap() == letI ||
        inputString2.chars().next().unwrap() == leto ||
        inputString2.chars().next().unwrap() == letO ||
        inputString2.chars().next().unwrap() == letu ||
        inputString2.chars().next().unwrap() == letU  {
        
        }else {
        newString.push(inputString2.chars().next().unwrap());
        }
        remove_first(inputString2);
    }
    
return newString;
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