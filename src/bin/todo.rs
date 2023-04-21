use mytodo::db::{
    create_task, delete_task, establish_connection, query_display_task, query_task, read_input,
    update_task, max_title
};
use std::env;

const PENDING_TASK: &str = "<PendingTask>";

fn help() {
    println!("subcommands:");
    println!("    new<id, title>: create a new task");
    println!("    show<id, title>: show pending task(s)");
    println!("    update<id, title>: update pending task(s)");
    println!("    delete<id, title>: delete task(s)")
}

fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("\nnew: missing <title>.  Add a task name.");
        //help();
        return;
    }

    create_task(&mut establish_connection(), &args[0]);
}

fn max_task(args: &[String]) {
    if args.len() > 1 {
        println!("\nnew: missing <title>.  Add a task name.");
        //help();
        return;
    }

    max_title();
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    match query_task(&mut establish_connection()).len() == 0 {
        true => {
            println!("\nThere are 0 {PENDING_TASK} to show!");
        }
        false => {
            print!("ID    TITLE\n---   -----\n");
            for task in query_task(&mut establish_connection()) {
                print!("{}     {} {}: {PENDING_TASK}\n", task.id, task.title, task.title.len());
            }
        }
    };
}

fn update_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("update: unexpected argument");
        help();
        return;
    }

    match query_task(&mut establish_connection()).len() == 0 {
        true => {
            println!("\nThere are 0 {PENDING_TASK} to update!");
        }
        false => {
            print!("\n\nEnter one or more ID's seperated by a\nspace to update a {PENDING_TASK} below.\n");
            print!("\nID    TITLE\n---   -----\n");
            for task in query_task(&mut establish_connection()) {
                print!("{}     {}: {PENDING_TASK}\n", task.id, task.title);
            }
            print!("\n");

            /*
            Initialize ids with the user's input
            to Vec<i32>.
            */
            let ids = read_input();

            match update_task(ids.clone(), &mut establish_connection()) {
                Ok(n) => {
                    if n > 0 {
                        println!("\nID    TITLE\n---   -----");
                        for task in query_display_task(&mut establish_connection()) {
                            match task.done == false {
                                true => {
                                    print!("{}     {}:  {PENDING_TASK}\n", task.id, task.title);
                                }
                                false => {
                                    print!("{}     {}\n", task.id, task.title);
                                }
                            }
                        }
                    } else {
                        eprintln!("{n} valid Updates for ID(s) {:?}", &ids);
                    }
                }
                Err(e) => {
                    eprintln!("Error updating task(s): {e} with ID(s) {:?}", &ids);
                }
            }
        }
    }
}

fn delete_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("delete: unexpected argument");
        help();
        return;
    }

    let conn = &mut establish_connection();

    match query_display_task(conn).len() == 0 {
        true => {
            println!("\nThere are no tasks to delete!");
        }
        false => {
            print!("\n\nEnter one or more numerical ID's seperated\nby a space to delete one or more tasks below.\n");
            print!("\n");
            for task in query_display_task(conn) {
                match task.done == false {
                    true => {
                        print!("{}     {}:  {PENDING_TASK}\n", task.id, task.title);
                    }
                    false => {
                        print!("{}     {}\n", task.id, task.title);
                    }
                }
            }
            print!("\n");

            /*
            Initialize ids with the user's input
            to Vec<i32>.
            */
            let ids = read_input();

            match delete_task(ids.clone(), conn) {
                Ok(n) => {
                    match n > 0 {
                        true => {
                            println!("{n} valid deletes for IDs {:?}", &ids);
                        }
                        false => {
                            println!("{n} valid deletes for IDs {:?}", &ids);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error deleting task(s): {e} with ID(s) {:?}", &ids);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        "update" => update_tasks(&args[2..]),
        "delete" => delete_tasks(&args[2..]),
        "max" => max_task(&args[2..]),
        _ => help(),
    }
}
