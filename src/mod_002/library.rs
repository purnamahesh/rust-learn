// A good real-world example is a Library Management System. Here’s how you can structure it to practice structs, enums, modules, and the pub keyword:

// Scenario:
// You are building a library system to manage books and members.

// Requirements:


// There should be a library module.
// Inside, define:
// A Book struct (with title, author, and status).
// An enum BookStatus (Available, CheckedOut).
// A Member struct (with name and a list of borrowed books).
// Only the library should be able to change a book’s status (internal/private).
// Anyone can view book info and member info (public).

// The library exposes a function to borrow a book (public), but the actual logic to update status is private.
// Behaviors:

// Public: View book details, view member details, borrow a book.
// Private: Change book status, add/remove books from the library.
// Access Control:


// Use pub for structs and methods that should be accessible outside the module.
// Omit pub for internal logic.



pub mod Library {

    #[derive(Debug)]
    pub enum BookStatus {
        Available,
        CheckedOut
    }

    pub struct Book {
        pub title: String,
        pub author: String,
        status: BookStatus
    }

    pub struct Member {
        pub name: String,
        pub BorrowedBooks: Vec<&String>
    }


    impl Book {

        fn update_book_status(&mut self, status: BookStatus) {
            self.status = status; 
        }

        pub fn info(&self) {
            println!("Title: {} Author: {} Status: {:?}", self.title, self.author, self.status);
        }

        pub fn add_new_book(title: String, author: String) -> Self {
            Book { title: title, author: author, status: BookStatus::Available }
        } 

        pub fn return_book(&mut self) {
            self.status = BookStatus::Available;
        }

    }

    pub fn borrow_book(member: &mut Member, book: &mut Book) {
        match book.status {
            BookStatus::Available => {
                book.update_book_status(BookStatus::CheckedOut);
                member.BorrowedBooks.push(book.title.clone());
                println!("Book {} borrowed by {}", book.title, member.name);
            },
            BookStatus::CheckedOut => {
                println!("Book not available");
            }
        };

        // if let BookStatus::Available = book.status { } else { };
    }

}

pub use crate::Library::{borrow_book, Book, Member};

fn main () {
    let mut book1:Book = Book::add_new_book(
        String::from("Dune"), 
        String::from("Frank Herbert")
    );

    let mut book2:Book = Book::add_new_book(
        String::from("Dune Messiah"), 
        String::from("Frank Herbert")
    );

    let mut book2:Book = Book::add_new_book(
        String::from("Children of Dune"), 
        String::from("Frank Herbert")
    );

    let mut mahesh = Member {
        name: String::from("Mahesh"),
        BorrowedBooks: [].to_vec()
    };

    book1.info();

    borrow_book(&mut mahesh, &mut book1);

    book1.info();

    let mut john = Member {
        name: String::from("John Doe"),
        BorrowedBooks: [].to_vec()
    };

    borrow_book(&mut john, &mut book1);

}