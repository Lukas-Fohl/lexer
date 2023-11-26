enum Par{
    ParNorOp,
    ParNorCl,
    ParCurOp,
    ParCurCl,
    ParSquOp,
    ParSquCl,
    Invalid
}

trait ToPar{
    fn to_par(&self) -> Par;
}

impl ToPar for char{
    fn to_par(&self) -> Par{
        match self {
            '(' => Par::ParNorOp,
            ')' => Par::ParNorCl,
            '{' => Par::ParCurOp,
            '}' => Par::ParCurCl,
            '[' => Par::ParSquOp,
            ']' => Par::ParSquCl,
            _ =>   Par::Invalid
        }
    }
}

impl ToString for Par {
    fn to_string(&self) -> String {
        match self {
            Par::ParNorOp => String::from("\"(\""),
            Par::ParNorCl => String::from("\")\""),
            Par::ParCurOp => String::from("\"{\""),
            Par::ParCurCl => String::from("\"}\""),
            Par::ParSquOp => String::from("\"[\""),
            Par::ParSquCl => String::from("\"]\""),
            _ => String::from("\"Invalid\"")
        }
    }
}

fn main() {
    let mut line = String::new();
    let _b = std::io::stdin().read_line(&mut line).unwrap();
    for x in line.chars(){
        print!("{}\t",x.to_par().to_string());
    }
    /* 
    TODO
        - get cmd input
        - split --> look for par
        - put into vec
        - print vec
        - put into struct?
    */
}
