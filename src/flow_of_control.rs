
fn loop_loop() {
     //Infinite loop
     //break & continue with labels are available
     //might return a value with 'break 56'

     let mut x = 1;
     let mut count = 0;

     let p = 'bla: loop {
          count += 1;

          if count == 3 {
               x += 10;
               continue 'bla;
          }

          if count == 5 {
               break 'bla 42;
          }
     };

     assert_eq!(x, 11);
     assert_eq!(p, 42);
}

fn while_loop() {
     let mut k = 1;

     //continue/break with labels are available
     //() is always return value
     let ret = 'p: while k < 10 {
          k += 1;
          if k < 3 {
               continue 'p
          }
          break 'p;
     };

     assert_eq!(k, 3);
     assert_eq!(ret, ());
}

fn if_else() {
     let n = 4;

     //Else branch is required if expression needed (if w/o less evaluates to ())
     //All sub-blocks must be of the same type
     //might return a value

     let big_n = if n < 10 {
          n * 2
     } else if n < 20 {
          n * 3
     } else {
          n * 4
     };

     assert_eq!(big_n, 8);
}

fn for_range_loop() {
     //for range/iterator loop
     //continue/break with labels available
     //() is always return value

     //range with exclusive right bound
     let mut x = 0;
     for n in 1..10 {
          x += n;
     };
     assert_eq!(x, 45);

     //range with inclusive right bound
     let mut y = 0;
     for n in 1..=9 {
          y += n
     }
     assert_eq!(y, 45);

     //can use continue/break
     let mut z = 0;
     'l: for _ in 1..100 {
          z += 1;
          if z < 10 {
               continue 'l
          } else {
               break 'l
          }
     }
     assert_eq!(z, 10);

     //() is the return value
     let ret = for _ in 1..10 {
          4;
     };
     assert_eq!(ret, ());
}

fn for_iterator_loop() {
     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names {
               names2.push(name)
          }
          //Can't do: values were moved
          // assert_eq!(names, names);
     }

     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.into_iter() {
               names2.push(name)
          }
          //Can't do: values were moved
          // assert_eq!(names, names);
     }
     
     {
          let names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.iter() {
               names2.push(name)
          }
          //Can do: values were immutably borrowed
          assert_eq!(names, names);
          assert_eq!(names2, vec![&"Bob", &"Frank", &"Ferris"]);
     }

     {
          let mut names = vec!["Bob", "Frank", "Ferris"];
          let mut names2 = vec![];
          for name in names.iter_mut() {
               names2.push(name)
          }
          //Can do: values were mutably borrowed
          // assert_eq!(names, names);
          assert_eq!(names2, vec![&"Bob", &"Frank", &"Ferris"]);
     }
}

fn match_control() {
     let number = 42;

     //pattern1 | ... | pattern if condition => ...
     //pattern is matched value, _, range, destructuring pattern
     //destructuring pattern = structure with: matched value | destrucrturing pattern
     //also, all matched constants can be named with "name @"
     let x = match number {
          1 => "one".to_owned(),
          2 | 3 | 4 | 5 => "two".to_owned(),
          6..=10 => ">= 4".to_owned(),
          n @ 11..=42 if n % 2 == 0 => format!("even {} >= 10", n),
          n @ 11..=42 if n % 2 == 1 => format!("odd {} >= 10", n),
          _ => "dunno".to_owned(),
          // p => "dunno".to_owned()
     };

     assert_eq!(x, "even 42 >= 10".to_owned());
}

fn match_and_destructuring() {
     //destructure tuples
     let pair = (2, false);
     let result = match pair {
          (2, true) => "a",
          (2, false) | (3, false) => "b",
          (_, true) => "c",
          (_d, g @ false) if g => "bla",
          _ => "oops"
     };
     assert_eq!(result, "b");

     //todo enums

     //todo structs
}

fn match_and_refrerences() {
     //todo
}

pub fn main() {
     if_else();
     loop_loop();
     while_loop();
     for_range_loop();
     for_iterator_loop();
     match_control();
     match_and_destructuring();
     match_and_refrerences();
}