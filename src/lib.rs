pub struct Terminal {
    pub flag: String,
    pub text: String,
    pub target: String,
}

impl Terminal {
    pub fn new(ctx: &[String]) -> Result<Terminal, &'static str> {
        if ctx.len() == 2 {
            let flag = ctx[1].clone();
            let text = String::from("");
            let target = String::from("");

            Ok(Terminal { flag, text, target })
        } else if ctx.len() == 3 {
            let flag = ctx[1].clone();
            let text = String::from("");
            let target = ctx[2].clone();

            Ok(Terminal { flag, text, target })
        } else if ctx.len() == 4 {
            let flag = ctx[1].clone();
            let text = ctx[2].clone();
            let target = ctx[3].clone();

            Ok(Terminal { flag, text, target })
        } else if ctx.len() > 4 {
            Err("Too many arguments! Run 'zender help' to see the list of commands")
        } else {
            Err("Missing arguments! Run 'zender help' to see the command pattern")
        }
    }
}