use procinfo::pid::{stat, stat_self, Stat};
use std::io::Error;
use uname::uname;

fn main() -> Result<(), Error> {
    let exec_order = exec_list()?;

    println!(
        "Commands: {:?}",
        exec_order
            .iter()
            .map(|s| s.command.clone())
            .rev()
            .collect::<Vec<_>>()
    );
    println!(
        "Pids: {:?}",
        exec_order.iter().map(|s| s.pid).rev().collect::<Vec<_>>()
    );

    let uname_info = uname()?;
    println!("{:?}", uname_info);

    Ok(())
}

fn exec_list() -> Result<Vec<Stat>, Error> {
    let mut vec = vec![];
    vec.push(stat_self()?);
    while vec.last().unwrap().pid > 1 {
        vec.push(stat(vec.last().unwrap().ppid)?);
    }
    Ok(vec)
}