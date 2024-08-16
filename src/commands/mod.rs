pub struct Command {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub arguments: Vec<Argument>,
    pub run: fn(&Vec<Argument>) -> Result<(), CommandError>,
}

pub struct Argument {
    pub name: String,
    pub short: Option<String>,
    pub description: String,
    pub value: Option<String>,
    pub required: bool,
}