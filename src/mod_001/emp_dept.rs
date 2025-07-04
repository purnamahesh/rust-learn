
// Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company. For example, “Add Sally to Engineering” or “Add
// Amir to Sales.” Then let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.

use std::{collections::HashMap, io::stdin, sync::Mutex};

fn read_input(inp: &mut String) {
    inp.clear();
    stdin().
    read_line( inp)
    .expect("failed in readline");
}

fn main () {

    let mut inp = String::new();
    let mut iter_count:u16 = 1;
    let mut records: HashMap<&str, Vec<String>> = HashMap::new();

    loop {

        println!("[1] Add Employee\n[2] Display Employees\n[3] exit\n\nEnter your choice:");

        read_input(&mut inp);

        let op: u8 = inp.trim().parse::<u8>().expect("Invalid choice");

        match op {
            1 => {
                if iter_count == 1 {
                    println!("\n\nEnter your input:\n\nExamples:\n\"Add John to R&D\"\n\"Remove John from R&D\"\n");
                }

                read_input(&mut inp);
                let mut temp_inp = inp.clone();
                let words:Vec<&str> = inp.trim().split_whitespace().collect();
                
                if words.len() != 4 {panic!("Enter proper input")}
                
                let oper = &words[0];
                let emp = &words[1];
                let dept = &words[3];
                
                // records.push(value);
                match *oper {
                    "add" => {
                        records
                        .entry(*dept)
                        .and_modify(|e| e.push(emp.to_string()))
                        .or_insert(vec![emp.to_string()]);
                        // records.push(Record { emp_name: emp.to_string(), department: dept.to_string() });
                    },
                    "remove" => {
                        println!("Functionality not available yet!");
                    }
                    _ => {
                        println!("Invalid instruction!");
                        return;
                    }
                }
                
            },
            2 => {

                for record in &records {
                    println!("{:?}", record);
                }

            },
            3 => return,
            _ => {
                println!("Invalid Choice!");
            }
        }
        iter_count += 1;
    }

}
