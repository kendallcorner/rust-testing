#![allow(warnings)]
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn heap_vs_stack() {
        let str_val = "It's a string".to_string();
        let str_assign = str_val;
        println!("{}", str_assign);
        // TODO: make test that str_val does not exist anymore?
        let str_val = "new_string".to_string();
        assert_ne!(str_val, str_assign);

        let str_val = "It's a string";
        let str_assign = str_val;
        // something like:
        // match str_val {
        //     Some => assert(true),
        //     _ => assert(false)

        // }
        let str_val = "new_string";
        assert_ne!(str_val, str_assign);
        

        let num_val = 1;
        let num1 = num_val;
        assert_eq!(num_val, num1);

    }

    #[test]
    fn ref_test() {
        let val = "reciprocal";
 
        let ref r1 = val;
        let r2 = &val;
        
        assert_eq!(r1, r2);
    }

    #[test]
    fn slice() {
        let s = String::from("hello world");
 
        let hello = &s[0..5];
        let world = &s[6..11];
        assert_eq!(hello, "hello");
        assert_eq!(world, "world");
    }

    #[test]
    fn ref_test_2() {
        let starship: Option<String> = Some("Omaha".to_string());
        fn find_starship_name(name: &String) {
            assert_eq!("Omaha", *name);
        }
        match starship {
            Some(ref name) => find_starship_name(name),
            None => {}
        }

        let planet = "Earth";
        let earth = &&&&planet;

        assert_eq!("EARTH", earth.to_uppercase());
    }

    #[test]
    fn mut_test() {
        let number = 20;

        // don't work
        // number += 1;
        // let number += 1;
        let number = number + 1;
        assert_eq!(number, 21);

        let mut another = 10;
        assert_eq!(another, 10);

        another = 12;
        assert_eq!(another, 12);

        another += 1;
        assert_eq!(another, 13);

    }

    #[test]
    fn struct_mut() {
        struct Coordinate {
            x: i32,
            y: i32,
        }
        
        let mut coord = Coordinate { x: 20, y: 20 };
        
        // We can mutate each field because coord is mutable.
        coord.x = 12;
        coord.y = 91;
        assert_eq!(coord.x, 12);
        assert_eq!(coord.y, 91);

        let _no_mut_coord = Coordinate { x: 10, y:15};
        // won't compile
        // _no_mut_coord.x = 12;
    }

    #[test]
    fn ref_with_mut() {
        fn question(s: &mut String) {
            s.pop();
            s.push('?');
        }

        let mut sentence = String::from("I am.");
        assert_eq!(sentence, "I am.");

        
        question(&mut sentence);
        assert_eq!(sentence, "I am?");
        
        let immutable_reference = &mut sentence;
        println!("{}", immutable_reference);
        println!("{}", sentence);
        // Doesn't work. Not sure why.
        // println!("{}", immutable_reference);
        
    }

    #[test]
    fn format_within_print() {
        let jungle_bird = "Macaw";
        let sound = "caws";
        format!("The {jungle_bird}"); // Does not print a newline
        format!(" {sound}."); // Prints a newline
        print!("The {jungle_bird}"); // Does not print a newline
        println!(" {sound}."); // Prints a newline
    }

    #[test]
    fn test_if_let() {
        let whale = ("Ahab", "Humpback");
 
        if let ("Ahab", species) = whale {
            println!("Funny name for a {species} whale");
        }
        
        // We can use `_` to ignore a specific field while matching.
        if let ("Tony", _) = whale {
            println!("Say hello to Tony the whale!");
        }
    }

    #[test]
    fn test_enums() {
        enum Fish {
            Carp,
            Flounder,
            Coy(String)
        }
        let pet_1 = Fish::Carp;

        let pet_2 = Fish::Flounder;
        
        let my_pet = Fish::Coy("Wanda".to_string());

        fn test_the_fish(my_pet: Fish) -> &'static str{
            if let Fish::Flounder = my_pet {
                // println!("Meet my pet flounder.")
                return "flounder"
            }
            
            if let Fish::Coy(name) = my_pet {
                // println!("My pet's name is {name}");
                return "name"
            } else {
                return "nope"
            }
        }

        let output_first = test_the_fish(pet_1);
        assert_eq!(output_first, "nope");
        let output_sec = test_the_fish(pet_2);
        assert_eq!(output_sec, "flounder");
        let output_third = test_the_fish(my_pet);
        assert_eq!(output_third, "name");

    }
}

