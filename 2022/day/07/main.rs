use std::fmt::Debug;

use color_eyre::{Report, Result};
use parse::Command;

#[allow(dead_code)]
fn empty_option_err() -> Report {
    use std::io::{Error, ErrorKind::Other};
    Error::new(Other, "Option was empty").into()
}

static INPUT: &str = include_str!("input");
static TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

fn main() -> Result<()> {
    const MAX_DIR_SIZE: u32 = 100_000;
    const DISK_SIZE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;
    color_eyre::install()?;

    let (_, test_cmds) = parse::input(TEST_INPUT)?;
    let test_fs = Dir::from(test_cmds.as_slice());
    assert_eq!(filesystem_size(&test_fs, MAX_DIR_SIZE), 95437);
    assert_eq!(test_fs.calculate_size(), 48381165);
    assert_eq!(free_space(&test_fs, REQUIRED_SPACE, DISK_SIZE), 24933642);

    let (_, cmds) = parse::input(INPUT)?;
    let filesystem = Dir::from(cmds.as_slice());

    let p1_solution = filesystem_size(&filesystem, MAX_DIR_SIZE);
    let p2_solution = free_space(&filesystem, REQUIRED_SPACE, DISK_SIZE);

    println!("Problem 1: {p1_solution}\nProblem 2: {p2_solution}");

    Ok(())
}

fn free_space<'a, 'b: 'a>(fs: &'a Dir<'b>, required: u32, total: u32) -> u32 {
    let unused = total - fs.calculate_size();
    let to_free = required - unused;

    let mut queue: Vec<_> = fs
        .files
        .iter()
        .filter_map(|entry| match &entry.kind {
            EntryKind::Dir(dir) => Some(dir),
            EntryKind::File(_) => None,
        })
        .collect();

    let mut min_size = fs.calculate_size();

    while let Some(wd) = queue.pop() {
        let size = wd.calculate_size();
        if size < min_size && size >= to_free {
            min_size = size;
        }

        for file in &wd.files {
            match &file.kind {
                EntryKind::Dir(dir) => queue.push(dir),
                _ => continue,
            }
        }
    }

    min_size
}

fn filesystem_size(filesystem: &Dir, max_dir_size: u32) -> u32 {
    let mut size = 0;

    let mut queue = vec![filesystem];
    while let Some(wd) = queue.pop() {
        if wd.calculate_size() <= max_dir_size {
            size += wd.calculate_size();
        }

        for file in &wd.files {
            match &file.kind {
                EntryKind::Dir(dir) => queue.push(dir),
                _ => continue,
            }
        }
    }

    size
}

struct DirEntry<'a> {
    name: &'a str,
    kind: EntryKind<'a>,
}

#[derive(Debug)]
struct Dir<'a> {
    files: Vec<DirEntry<'a>>,
}

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
enum EntryKind<'a> {
    File(File),
    Dir(Dir<'a>),
}

impl Debug for DirEntry<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.name)?;
        match &self.kind {
            EntryKind::Dir(Dir { files }) => f.debug_list().entries(files).finish(),
            EntryKind::File(File { size }) => write!(f, "{size}"),
        }
    }
}

impl Dir<'_> {
    fn calculate_size(&self) -> u32 {
        self.files.iter().map(DirEntry::calculate_size).sum()
    }
}

impl DirEntry<'_> {
    fn calculate_size(&self) -> u32 {
        match &self.kind {
            EntryKind::Dir(dir) => dir.calculate_size(),
            EntryKind::File(File { size }) => *size,
        }
    }
}

impl<'a> Dir<'a> {
    fn lookup_dir_mut(&mut self, path: &[&str]) -> &mut Dir<'a> {
        match path {
            [] => self,
            [name] => {
                let entry = self
                    .files
                    .iter_mut()
                    .find(|child| &child.name == name)
                    .expect("dir not found");
                match &mut entry.kind {
                    EntryKind::File(_) => panic!("Expected Dir"),
                    EntryKind::Dir(dir) => dir,
                }
            }
            _ => self.lookup_dir_mut(&[path[0]]).lookup_dir_mut(&path[1..]),
        }
    }
}

impl<'a> From<&[Command<'a>]> for Dir<'a> {
    fn from(cmds: &[Command<'a>]) -> Self {
        use Command::*;

        let mut root = Self { files: vec![] };
        let mut pwd = vec![];

        for cmd in cmds {
            match cmd {
                Cd(parse::Dir::Root) => pwd.clear(),
                Cd(parse::Dir::Up) => {
                    pwd.pop();
                }
                Cd(parse::Dir::Name(name)) => pwd.push(*name),
                Ls(files) => {
                    let wd = root.lookup_dir_mut(&pwd);

                    assert!(wd.files.is_empty());
                    for file in files.iter().copied() {
                        wd.files.push(file.into())
                    }
                }
            };
        }

        root
    }
}

impl<'a> From<parse::DirEntry<'a>> for DirEntry<'a> {
    fn from(de: parse::DirEntry<'a>) -> Self {
        match de {
            parse::DirEntry::File { size, name } => Self {
                name,
                kind: EntryKind::File(File { size }),
            },
            parse::DirEntry::Dir { name } => Self {
                name,
                kind: EntryKind::Dir(Dir { files: vec![] }),
            },
        }
    }
}

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::{
            complete::{digit1, line_ending, space1},
            streaming::not_line_ending,
        },
        combinator::{map, map_res},
        multi::{many0, many1},
        sequence::{delimited, pair, preceded, separated_pair, terminated},
        IResult,
    };

    #[derive(Debug)]
    pub enum Command<'a> {
        Cd(Dir<'a>),
        Ls(Vec<DirEntry<'a>>),
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Dir<'a> {
        Root,
        Up,
        Name(&'a str),
    }

    #[derive(Debug, Clone, Copy)]
    pub enum DirEntry<'a> {
        File { size: u32, name: &'a str },
        Dir { name: &'a str },
    }

    pub fn input(input: &str) -> IResult<&str, Vec<Command>> {
        many1(command)(input)
    }

    fn command(input: &str) -> IResult<&str, Command> {
        alt((cd_cmd, ls_cmd))(input)
    }

    fn cd_cmd(input: &str) -> IResult<&str, Command> {
        use Command::Cd;
        use Dir::{Name, Root, Up};
        map(
            delimited(tag("$ cd "), not_line_ending, line_ending),
            |s| match s {
                ".." => Cd(Up),
                "/" => Cd(Root),
                _ => Cd(Name(s)),
            },
        )(input)
    }

    fn ls_cmd(input: &str) -> IResult<&str, Command> {
        use Command::Ls;
        use DirEntry::{Dir, File};

        preceded(
            pair(tag("$ ls"), line_ending),
            map(
                many0(terminated(
                    alt((
                        map(
                            separated_pair(
                                map_res(digit1, str::parse::<u32>),
                                space1,
                                not_line_ending,
                            ),
                            |(size, name)| File { size, name },
                        ),
                        map(preceded(tag("dir "), not_line_ending), |name| Dir { name }),
                    )),
                    line_ending,
                )),
                Ls,
            ),
        )(input)
    }
}
