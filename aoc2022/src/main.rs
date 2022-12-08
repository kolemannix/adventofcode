#![feature(iter_array_chunks)]

use anyhow::{Error, Result};
use std::{cell::RefCell, rc::Rc, slice::Iter, str::FromStr};
mod helper;
mod past;
struct Engine {
    pwd: Rc<RefCell<FSEntry>>,
    root: Rc<RefCell<FSEntry>>,
}

impl Engine {
    fn exec(&mut self, entry: &InputEntry) {
        println!("EXEC from {}: {:?}", self.pwd.borrow().name(), &entry);
        match entry {
            InputEntry::Cmd(FSCmd::CdDir { name }) => {
                if name == "/" {
                    self.pwd = self.root.clone();
                } else {
                    let pwd = self.pwd.clone();
                    let child = pwd
                        .borrow()
                        .children_iter()
                        .find(|c| c.borrow().name() == name)
                        .unwrap_or_else(|| panic!("Expected child: {name}"))
                        .clone();
                    self.pwd = child;
                }
            }
            InputEntry::Cmd(FSCmd::CdUp) => {
                let cur_pwd = self.pwd.clone();
                println!("GO UP from {}", cur_pwd.borrow().name());
                if let Some(parent) = cur_pwd.borrow().parent() {
                    self.pwd = parent;
                };
            }
            InputEntry::Cmd(FSCmd::LS) => {
                // No need to do anything
            }
            InputEntry::Output(FSEntry::Dir(dir)) => {
                let mut pwd = self.pwd.borrow_mut();
                if let FSEntry::Dir(FSDir { children, .. }) = &mut *pwd {
                    let mut dir_to_add = dir.clone();
                    dir_to_add.parent = Some(self.pwd.clone());
                    children.push(Rc::new(RefCell::new(FSEntry::Dir(dir_to_add))));
                }
            }
            InputEntry::Output(file @ FSEntry::File { .. }) => {
                if let Some(children) = self.pwd.borrow_mut().children_mut() {
                    children.push(Rc::new(RefCell::new(file.clone())));
                }
            }
        };
    }
}

#[derive(Debug, Clone)]
enum FSEntry {
    File { name: String, size: usize },
    Dir(FSDir),
}

#[derive(Debug, Clone)]
struct FSDir {
    name: String,
    children: Vec<Rc<RefCell<FSEntry>>>,
    parent: Option<Rc<RefCell<FSEntry>>>,
}

impl FSEntry {
    fn parent(&self) -> Option<Rc<RefCell<FSEntry>>> {
        match self {
            FSEntry::File { .. } => None,
            FSEntry::Dir(dir) => dir.parent.clone(),
        }
    }
    fn children_mut(&mut self) -> Option<&mut Vec<Rc<RefCell<FSEntry>>>> {
        match self {
            FSEntry::File { .. } => None,
            FSEntry::Dir(dir) => Some(&mut dir.children),
        }
    }
    fn children_iter(&self) -> Iter<Rc<RefCell<FSEntry>>> {
        match self {
            FSEntry::File { .. } => [].iter(),
            FSEntry::Dir(dir) => dir.children.iter(),
        }
    }
    fn name(&self) -> &String {
        match self {
            FSEntry::File { name, .. } => name,
            FSEntry::Dir(dir) => &dir.name,
        }
    }
}

#[derive(Debug, Clone)]
enum InputEntry {
    Cmd(FSCmd),
    Output(FSEntry),
}

#[derive(Debug, Clone)]
enum FSCmd {
    CdDir { name: String },
    CdUp,
    LS,
}

impl FromStr for InputEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let first = chars.next().unwrap();
        if first == '$' {
            chars.next();
            let cmd_char = chars.next().unwrap();
            match cmd_char {
                'c' => {
                    chars.next(); // Eat d
                    chars.next(); // Eat space
                    let dest = chars.peek().unwrap();
                    if *dest == '.' {
                        Ok(InputEntry::Cmd(FSCmd::CdUp))
                    } else {
                        let dest_name: String = chars.collect();
                        Ok(InputEntry::Cmd(FSCmd::CdDir { name: dest_name }))
                    }
                }
                'l' => {
                    chars.next(); // Eat s
                    Ok(InputEntry::Cmd(FSCmd::LS))
                }
                _ => Err(anyhow::anyhow!("Could not parse command")),
            }
        } else if first == 'd' {
            chars.next(); // eat i
            chars.next(); // eat r
            chars.next(); // eat space
            let dir_name: String = chars.collect();
            Ok(InputEntry::Output(FSEntry::Dir(FSDir {
                name: dir_name,
                children: vec![],
                parent: None,
            })))
            // Dir
        } else if first.is_numeric() {
            // File entry
            let mut parts = s.split_whitespace();
            let size: usize = parts.next().unwrap().parse()?;
            let name: String = parts.next().unwrap().to_string();
            Ok(InputEntry::Output(FSEntry::File { name, size }))
        } else {
            Err(anyhow::anyhow!("Did not parse command or output"))
        }
    }
}

fn day7() -> Result<String> {
    let root = Rc::new(RefCell::new(FSEntry::Dir(FSDir {
        name: "/".into(),
        children: vec![],
        parent: None,
    })));
    let mut engine = Engine {
        pwd: root.clone(),
        root,
    };
    let input = helper::load_puzzle_to_string(7, 0)?;
    let commands = input.lines().flat_map(|line| line.parse::<InputEntry>());
    for cmd in commands {
        engine.exec(&cmd);
        println!("{cmd:?}");
    }
    Ok("unimpl".to_string())
}

fn main() -> Result<()> {
    let res = day7()?;
    println!("{res}"); // 3059
    Ok(())
}
