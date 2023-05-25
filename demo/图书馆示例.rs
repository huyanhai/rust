struct Library {
    books: Vec<Book>,
}

#[derive(Debug)]
struct Book {
    title: String,
    year: u16,
}

trait Page {
    fn find_book(self, str: &str) -> Result<Book, &str>;
}

impl Book {
    fn new(title: String, year: u16) -> Book {
        Book { title, year }
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    fn len(&self) -> usize {
        self.books.len()
    }
}

impl Page for Library {
    fn find_book(self, str: &str) -> Result<Book, &str> {
        for bk in self.books {
            if bk.title == str {
                return Ok(bk);
            }
        }
        return Err("not found");
    }
}

fn main() {
    let mut library = Library::new();

    library.add(Book::new(String::from("语文"), 1991));
    library.add(Book::new(String::from("数学"), 1992));

    let len = library.len();

    match library.find_book("语文") {
        Ok(book) => println!("the book is {},year is {}", book.title, book.year),
        Err(str) => println!("{str:}"),
    }

    println!("library has {len:} books");
}
