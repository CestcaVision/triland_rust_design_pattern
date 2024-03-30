//! # Composite Pattern
//! Composite pattern is a structural design pattern that lets you compose objects into tree structures to represent part-whole hierarchies. Composite lets clients treat individual objects and compositions of objects uniformly.

pub trait FileSystemComponent {
    fn print(&self, indent: usize);
}

pub struct File {
    name: String,
}

impl FileSystemComponent for File {
    fn print(&self, indent: usize) {
        println!("{}- File: {}", " ".repeat(indent), self.name);
    }
}

pub struct Folder {
    name: String,
    children: Vec<Box<dyn FileSystemComponent>>,
}

impl FileSystemComponent for Folder {
    fn print(&self, indent: usize) {
        println!("{}+ Folder: {}", " ".repeat(indent), self.name);
        for child in self.children.iter() {
            child.print(indent + 2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_pattern_functionality() {
        let file1 = File {
            name: String::from("File1.txt"),
        };
        let file2 = File {
            name: String::from("File2.txt"),
        };
        let file3 = File {
            name: String::from("File3.txt"),
        };
        let mut folder1 = Folder {
            name: String::from("Folder1"),
            children: vec![Box::new(file1)],
        };
        let folder2 = Folder {
            name: String::from("Folder2"),
            children: vec![Box::new(file2), Box::new(file3)],
        };

        folder1.children.push(Box::new(folder2));

        folder1.print(0);
    }
}
