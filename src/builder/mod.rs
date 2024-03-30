//! # Builder Pattern
/// Builder pattern is a creational design pattern that allows constructing complex objects step by step. It is useful when you need to create an object with a lot of configuration options. This example use chainable methods to build a document.

// define a trait for general document builder, including necessary methods for document builder. Underline the parameter `&mut self` to make sure the method can modify the struct itself and the return type is `&mut Self` to make sure the method can be chained.
pub trait DocumentBuilder {
    fn make_title(&mut self, title: &str) -> &mut Self;
    fn make_author(&mut self, author: &str) -> &mut Self;
    fn make_content(&mut self, content: &str) -> &mut Self;
    fn build(&self) -> String;
}

pub struct TextBuilder {
    document: String,
}
impl TextBuilder {
    fn new() -> Self {
        TextBuilder {
            document: String::new(),
        }
    }
}

impl DocumentBuilder for TextBuilder {
    fn make_title(&mut self, title: &str) -> &mut Self {
        self.document += &format!("{}\n", title);
        self
    }

    fn make_author(&mut self, author: &str) -> &mut Self {
        self.document += &format!("Author: {}\n\n", author);
        self
    }

    fn make_content(&mut self, content: &str) -> &mut Self {
        self.document += &format!("{}\n", content);
        self
    }

    fn build(&self) -> String {
        self.document.clone()
    }
}

struct HTMLBuilder {
    document: String,
}

impl HTMLBuilder {
    fn new() -> Self {
        HTMLBuilder {
            document: String::from("<html>\n"),
        }
    }
}

impl DocumentBuilder for HTMLBuilder {
    fn make_title(&mut self, title: &str) -> &mut Self {
        self.document += &format!("<head><title>{}</title></head>\n", title);
        self.document += &format!("<body>\n<h1>{}</h1>\n", title);
        self
    }

    fn make_author(&mut self, author: &str) -> &mut Self {
        self.document += &format!("<h2>Author: {}</h2>\n", author);
        self
    }

    fn make_content(&mut self, content: &str) -> &mut Self {
        self.document += &format!("<p>{}</p>\n", content);
        self
    }

    fn build(&self) -> String {
        self.document.clone() + "</body>\n</html>\n"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_pattern_functionality() {
        let text_document = TextBuilder::new()
            .make_title("Design Patterns")
            .make_author("Gang of Four")
            .make_content("Design Patterns: Elements of Reusable Object-Oriented Software")
            .build();
        assert_eq!("Design Patterns\nAuthor: Gang of Four\n\nDesign Patterns: Elements of Reusable Object-Oriented Software\n", text_document);

        let html_document = HTMLBuilder::new()
            .make_title("Design Patterns")
            .make_author("Gang of Four")
            .make_content("Design Patterns: Elements of Reusable Object-Oriented Software")
            .build();
        assert_eq!("<html>\n<head><title>Design Patterns</title></head>\n<body>\n<h1>Design Patterns</h1>\n<h2>Author: Gang of Four</h2>\n<p>Design Patterns: Elements of Reusable Object-Oriented Software</p>\n</body>\n</html>\n", html_document);
    }
}
