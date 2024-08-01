use core::fmt;
use std::collections::HashMap;
use std::{default, env};

#[derive(Debug, Clone)]
enum Value<'a> {
    Boole(bool),
    Int(i64),
    Str(&'a str),

}

#[derive(Debug,Clone)]
enum Type {
    Boole,
    Int,
    Str,
}
enum Error {
    InvalidType(String),
    InvalidArg(String),
    ParseError(String),//解析字符串为其他类型，那么可能会遇到解析错误。
    OtherError(String),

}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            Error::InvalidType(desc) => desc,
            Error::InvalidArg(desc) => desc,
            Error::ParseError(desc) => desc,
            Error::OtherError(desc) => desc,
        };
        write!(f, "Error: {}", description)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            Error::InvalidType(desc) => desc,
            Error::InvalidArg(desc) => desc,
            Error::ParseError(desc) => desc,
            Error::OtherError(desc) => desc,
        };
        write!(f, "Error: {}", description)
    }
}
#[derive(Debug,Clone)]
struct PrefixArg<'a>{
    short_prefix:Option<&'a str>,
    long_prefix:Option<&'a str>,
    both_prefix:Option<(&'a str,&'a str)>,
}
impl<'a> PrefixArg<'a> {
    fn from_str(s: &'a str) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split(',').collect();
        match parts.len() {
            1 => {
                if parts[0].starts_with("--") {
                    Ok(PrefixArg { long_prefix: Some(parts[0]), short_prefix: None, both_prefix: None })
                } else if parts[0].starts_with("-") {
                    Ok(PrefixArg { short_prefix: Some(parts[0]), long_prefix: None, both_prefix: None })
                } else {
                    Err("Invalid prefix")
                }
            },
            2 => {
                if parts[0].starts_with("--") && parts[1].starts_with("-") {
                    Ok(PrefixArg { both_prefix: Some((parts[1], parts[0])), short_prefix: None, long_prefix: None })
                } else if parts[0].starts_with("-") && parts[1].starts_with("--") {
                    Ok(PrefixArg { both_prefix: Some((parts[0], parts[1])), short_prefix: None, long_prefix: None })
                } else {
                    Err("Invalid prefix")
                }
            },
            _ => Err("Invalid prefix")
        }
    }
}
#[derive(Debug,Clone)]
struct ArgInfo<'a> {
    prefix: PrefixArg<'a>,
    arg_type: Type,
    default: Option<Value<'a>>,
    instruction: &'a str,
}

impl<'a> ArgInfo<'a>{
    
    fn new(prefix: PrefixArg<'a>, arg_type: Type, instruction: &'a str, default: Option<Value<'a>>) -> Self {
        ArgInfo {
            prefix,
            arg_type,
            default,
            instruction,
        }
    }
    
}





#[derive(Debug,Clone)]
pub struct Commander<'a> {
    name:&'a str,
    version:&'a str,
    description:&'a str,
    args_list: Vec<ArgInfo<'a>>,// 参数信息列表
    // all_args:Vec<&'a str>, // 所有参数列表
    default_values: HashMap<&'a str, Value<'a>>, // 参数值映射表
    function: &'a str, // 函数指针function: fn()
    // helps: Vec<&'a str>, // 帮助选项列表
}

impl<'a> Commander<'a>{
    fn new() -> Self {
        Commander {
            name: "",
            version: "",
            description: "",
            args_list: Vec::new(),
            default_values: HashMap::new(),
            function: "",
        }
    }
    //添加name
    fn name(mut self, name: &'a str) -> Commander {
        self.name = name;
        self
    }

    //添加version
    fn version(mut self, version: &'a str) ->Commander {
        self.version = version;
        self
    }
    //添加命令行工具描述
    fn instruction(mut self, description: &'a str) -> Commander {
        self.description = description;
        self
    }
    pub fn option(mut self, prefix: &'a str, instruction: &'a str, default: Option<bool>) -> Self {
        //判断prefix输入格式： "-p,--print" or "-p" or "--print"
        match PrefixArg::from_str(prefix) {
            Ok(prefix_arg) => {
                //如果default是Some，那么default_value就会是Some(Value::Boole(val))；
                //如果default是None，那么default_value就会是None
                let default_value = default.map(|val|Value::Boole(val));
                if let Some(value)=default_value.clone(){
                    self.default_values.insert(prefix,value);
                }
                let arg = ArgInfo::new(prefix_arg, Type::Boole, instruction, default_value);
                self.args_list.push(arg);

            },
            Err(e) => {
                eprint!("prefix format err: {}", e);
            }
        }
        self
    }
    pub fn option_str(mut self, prefix: &'a str, instruction: &'a str, default: Option<&'a str>) -> Self {
        //判断prefix输入格式： "-p,--print" or "-p" or "--print"
        match PrefixArg::from_str(prefix) {
            Ok(prefix_arg) => {
                //如果default是Some，那么default_value就会是Some(Value::Boole(val))；
                //如果default是None，那么default_value就会是None
                let default_value = default.map(|val|Value::Str(val));
                if let Some(value)=default_value.clone(){
                    self.default_values.insert(prefix,value);
                }
                let arg = ArgInfo::new(prefix_arg, Type::Str, instruction, default_value);
                self.args_list.push(arg);

            },
            Err(e) => {
                eprint!("prefix format err: {}", e);
            }
        }
        self
    }
    pub fn option_int(mut self, prefix: &'a str, instruction: &'a str, default: Option<i64>) -> Self {
        //判断prefix输入格式： "-p,--print" or "-p" or "--print"
        match PrefixArg::from_str(prefix) {
            Ok(prefix_arg) => {
                //如果default是Some，那么default_value就会是Some(Value::Boole(val))；
                //如果default是None，那么default_value就会是None
                let default_value = default.map(|val|Value::Int(val));
                if let Some(value)=default_value.clone(){
                    self.default_values.insert(prefix,value);
                }
                let arg = ArgInfo::new(prefix_arg, Type::Str, instruction, default_value);
                self.args_list.push(arg);

            },
            Err(e) => {
                eprint!("prefix format err: {}", e);
            }
        }
        self
    }

    pub fn execute_command(&mut self,input_args:Vec<String>){
        //&mut self,prefix:&'a str,command:&'a str
        let prefix_clone = input_args[1].clone();
        let prefix = prefix_clone.as_str();
        let command_clone = input_args[2].clone();
        let command = command_clone.as_str();
        for arg in &self.args_list {
            if let Some(short_prefix) = arg.prefix.short_prefix {
                if short_prefix == prefix {
                    match arg.arg_type {
                        Type::Boole => {
                            match command {
                                "true" => {
                                    println!("true");
                                },
                                "false" => {
                                    println!("false");
                                },
                                _ => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Int => {
                            match command.parse::<i64>() {
                                Ok(val) => {
                                    println!("{}",val);
                                },
                                Err(e) => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Str => {
                            println!("{}",command);
                        }
                    }
                }
            }
            if let Some(long_prefix) = arg.prefix.long_prefix {
                if long_prefix == prefix {
                    match arg.arg_type {
                        Type::Boole => {
                            match command {
                                "true" => {
                                    println!("true");
                                },
                                "false" => {
                                    println!("false");
                                },
                                _ => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Int => {
                            match command.parse::<i64>() {
                                Ok(val) => {
                                    println!("{}",val);
                                },
                                Err(e) => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Str => {
                            println!("{}",command);
                        }
                    }
                }
            }
            if let Some((short_prefix,long_prefix)) = arg.prefix.both_prefix {
                if short_prefix == prefix || long_prefix == prefix {
                    match arg.arg_type {
                        Type::Boole => {
                            match command {
                                "true" => {
                                    println!("true");
                                },
                                "false" => {
                                    println!("false");
                                },
                                _ => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Int => {
                            match command.parse::<i64>() {
                                Ok(val) => {
                                    println!("{}",val);
                                },
                                Err(e) => {
                                    eprint!("Invalid command");
                                }
                            }
                        },
                        Type::Str => {
                            println!("{}",command);
                        }
                    }
                }
            }
        }


    }

    pub fn analyse_command(&mut self){
        let input_args: Vec<String> = env::args().collect();
        if input_args.len() < 3 {
            eprint!("No command input");
            return;
        }
        let prefix = input_args[1].clone();
        match prefix.as_str() {
            "-h" | "--help" => {
                self.print_help();
            },
            "-v" | "--version" => {
                self.print_version();
            },
            _ => {
                self.execute_command(input_args);
            }
        }
    }


    
    pub fn print_version(&self){
        let mut version = "".to_string();
        version += "Version:";
        version += self.version;
        println!("{}",version);

    }
    pub fn print_help(&self){
        let mut help = "".to_string();
        help += "Description:";
        help += self.description;
        println!("{}",help);
        for arg in &self.args_list {
            let mut arg_help = "".to_string();
            if let Some(short_prefix) = arg.prefix.short_prefix {
                arg_help += short_prefix;
            }
            if let Some(long_prefix) = arg.prefix.long_prefix {
                arg_help += long_prefix;
            }
            arg_help += "\t";
            arg_help += arg.instruction;
            println!("{}",arg_help);
        }
    }


}
fn main() {
    let mut command = Commander::new()
        .name("test")
        .version("1.0")
        .instruction("test command")
        .option("-p,--print","print",Some(true))
        .option_str("-n,--name", "print name", Some("test"))
        .option_int("-v", "print version", Some(2))
        .analyse_command();
}
