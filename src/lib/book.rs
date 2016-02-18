use error::{Error,Result};

// Numbering for a given chapter
#[derive(Debug)]
pub enum Number {
    Unnumbered, // chapter is not numbered
    Default, // chapter follows books numbering, number is given automatically
    Specified(i32), //chapter number set to specified number
}
    

// Configuration of the book
#[derive(Debug)]
pub struct Book {
    // Generic options
    numbering: bool, // turns on/off chapter numbering (individual chapters may still avoid it)
    autoclean: bool, 
    chapters: Vec<(Number, String)>,  // list of the markdown files to process
    lang: String,
    author: String,
    title: String,
    cover: Option<String>,
}


impl Book {
    // Creates a new Book with default options
    pub fn new() -> Book {
        Book {
            numbering: true,
            autoclean: true,
            chapters: vec!(),
            lang: String::from("en"),
            author: String::from("Anonymous"),
            title: String::from("Untitled"),
            cover: None,
        }
    }

    /// Sets options according to configuration file
    ///
    /// A line with "option: value" sets the option to value
    /// + chapter_name.md adds the (default numbered) chapter
    /// - chapter_name.md adds the (unnumbered) chapter
    /// 3. chapter_name.md adds the (custom numbered) chapter
    pub fn set_from_config(&mut self, s: &str) -> Result<()> {
        fn get_filename(s: &str) -> Result<&str> {
            let words:Vec<&str> = (&s[1..]).split_whitespace().collect();
            if words.len() > 1 {
                return Err(Error::ConfigParser("chapter filenames must not contain whitespace"));
            } else if words.len() < 1 {
                return Err(Error::ConfigParser("no chapter name specified"));
            }
            Ok(words[0])
        }
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.starts_with('-') {
                //unnumbered chapter
                let file = try!(get_filename(line));
                self.add_chapter(Number::Unnumbered, String::from(file));
            } else if line.starts_with('+') {
                //nunmbered chapter
                let file = try!(get_filename(line));
                self.add_chapter(Number::Default, String::from(file));
            } else if line.starts_with(|c: char| c.is_digit(10)) {
                // chapter with specific number
                let parts:Vec<_> = line.splitn(2, |c: char| c == '.' || c == ':' || c == '+').collect();
                if parts.len() != 2 {
                    return Err(Error::ConfigParser("ill-formatted line specifying chapter number"));
                } else {
                    let file = try!(get_filename(parts[1]));
                    let number = try!(parts[0].parse::<i32>());
                    self.add_chapter(Number::Specified(number), String::from(file));
                }
            } else {
                // standard case: "option: value"
                let parts:Vec<_> = line.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(Error::ConfigParser("option setting must be of the form option: value"));
                }
                let option = parts[0].trim();
                let value = parts[1].trim();
                match option {
                    "numbering" => self.set_numbering(try!(value.parse::<bool>())),
                    "autoclean" => self.set_autoclean(try!(value.parse::<bool>())),
                    "author" => self.set_author(String::from(value)),
                    "title" => self.set_title(String::from(value)),
                    "cover" => self.set_cover(Some(String::from(value))),
                    "lang" => self.set_lang(String::from(value)),
                    _ => return Err(Error::ConfigParser("unrecognized option")),
                }
            }
        }

        Ok(())
    }

    /// Sets numbering of chapters
    ///
    /// false: no chapter is numbered
    /// true: chapters are numbered, expect the ones that opt out of it
    ///
    /// default: true
    pub fn set_numbering(&mut self, numbering: bool) {
        self.numbering = numbering;
    }

    /// Sets lang of a book
    ///
    /// Should be a standard code: En, Fr, ...
    ///
    /// Default: en
    pub fn set_lang(&mut self, lang: String) {
        self.lang = lang;
    }

    /// Sets author of a book
    ///
    /// A single string for full name
    ///
    /// Default: Anonymous
    pub fn set_author(&mut self, author: String) {
        self.author = author;
    }

    /// Sets title of a book
    ///
    /// Default: Untitled
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Sets the cover for the book
    ///
    /// Specifies the name (and path!) of a file, e.g. "cover.png"
    ///
    /// Default: None
    pub fn set_cover(&mut self, cover: Option<String>) {
        self.cover = cover;
    }

    /// Sets whether cleaning of input markdown is activated
    ///
    /// Default: true
    ///
    /// The cleaning is dependend on the language. By default, it
    /// only removes multiple following spaces, so it should have no effect
    /// on generated result (expect for the source files). But in french,
    /// tries to 'intelligently' replaces spaces with non-breaking ones when
    /// in front of appopriacte characters ('?', '!', ':' and so on).
    pub fn set_autoclean(&mut self, autoclean: bool) {
        self.autoclean = autoclean;
    }

    /// Adds a chapter to the book and its number scheme
    ///
    /// Number: either Default, Unnumbered or Specified(number)
    /// File: location of the file for this chapter
    pub fn add_chapter(&mut self, number: Number, file: String) {
        self.chapters.push((number, file));
    }
}