use std::iter;


/// A structure for manipulating Table Of Content
pub struct Toc {
    elements: Vec<TocElement>
}

impl Toc {
    /// Create a new, empty, Toc
    pub fn new() -> Toc {
        Toc {
            elements: vec!(),
        }
    }

    /// Adds an element 
    pub fn add(&mut self, level: i32, url: String, title: String) {
        let element = TocElement::new(level, url, title);
        self.elements.push(element);
    }

    /// Render the Toc
    pub fn render(&self) -> String {
        let mut output = String::new();

        let mut x = 0;
        let mut level = 0;
        output.push_str(&self.render_vec(&mut x, &mut level));
        for i in (0..level).rev() {
            output.push_str(&format!("{}</ul>",
                                     iter::repeat(' ').take(i as usize).collect::<String>()));
        }
        output
    }

    fn render_vec(&self, x: &mut usize, level: &mut i32) -> String {
        let orig_level = *level;
        let mut content = String::new();
        while *x < self.elements.len() {
            let elem = &self.elements[*x];
            
            if elem.level <= orig_level {
                return content
            }
            
            *x += 1;

            if elem.level > *level {
                for i in *level..elem.level {
                    content.push_str(&format!("{}<ul>\n",
                                              iter::repeat(' ').take(i as usize).collect::<String>()));
                    *level = elem.level;
                }
            } else if elem.level < *level {
                for i in (elem.level..*level).rev() {
                    content.push_str(&format!("{}</ul>\n",
                                              iter::repeat(' ').take(i as usize).collect::<String>()));
                }
                *level = elem.level;
            }
            let spaces:String = iter::repeat(' ').take(elem.level as usize).collect();
            content.push_str(&format!("{}<li>{}\n", spaces, elem.title));
            content.push_str(&self.render_vec(x, level));

            for i in (elem.level..*level).rev() {
                content.push_str(&format!("{}</ul>\n",
                                          iter::repeat(' ').take(i as usize).collect::<String>()));
            }
            *level = elem.level;
            content.push_str(&format!("{}</li>\n", spaces));
            
        }
        content
    }
}


struct TocElement {
    level: i32,
    url: String,
    title: String,
}

impl TocElement {
    pub fn new(level: i32, url: String, title: String) -> TocElement {
        TocElement {
            level: level,
            url: url,
            title: title,
        }
    }
}

