pub struct J{
    content:i32
}
impl G for J{
    type Content = i32;
    fn is_event()->bool{
        true
    }
    fn is_socket()->bool{
        false
    }
    fn get_content(&self)->i32{
        self.content
    }
}
pub struct R<'a>{
    content:&'a str
}
impl<'a> G for R<'a>{
    type Content = &'a str;
    fn is_event()->bool{
        false
    }
    fn is_socket()->bool{
        true
    }
    fn get_content(&self)-> &'a str{
        self.content
    }
}
pub trait G<Content>{
    type Content;
    fn is_event()->bool;
    fn is_socket()->bool;
    fn get_content(&self)->Self::Content;
}

pub fn main(){
    let j1:J = J{content:34};
    let r1:R = R{content:"asdas"};
    let mut v: Vec<Box<G>> = Vec::new();
    v.push(Box::new(j1));
    v.push(Box::new(r1));
    for animal in v.iter(){
        println!("{}",animal.get_content());
    }
    
}
