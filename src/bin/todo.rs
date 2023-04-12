use std::env;
use mytodo::db::{create_task, establish_connection, query_task,
                 read_input, update_task, query_display_task};
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
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = &mut establish_connection();
    create_task(conn, &args[0]);
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = &mut establish_connection();
    let post = query_task(conn);
    match post.len() == 0 {
        true => {
            println!("\nThere are 0 {PENDING_TASK} to show!");
        }
        false => {
            print!("ID    TITLE\n---   -----\n");
            for task in query_task(conn) {
                print!("{}     {}: {PENDING_TASK}\n", task.id, task.title );
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

    let conn = &mut establish_connection();
    let post = query_task(conn);
    match post.len() == 0 {
        true => {
            println!("\nThere are 0 {PENDING_TASK} to update!");
        }
        false => {
            print!("\n\nEnter one or more ID's seperated by a\nspace to update a {PENDING_TASK} below.\n");
            print!("\nID    TITLE\n---   -----\n");
            for task in query_task(conn) {
                print!("{}     {}: {PENDING_TASK}\n", task.id, task.title );
            }
            print!("\n"); 
            
            /*
            Initialize ids with the user's input.
            */
            let ids = read_input();

               match update_task(ids.clone(), conn) {
                    Ok(n) => {
                        if n > 0 {
                            println!("\nID    TITLE\n---   -----");
                            for task in query_display_task(conn) {
                                match task.done == false {
                                    true => {
                                        print!("{}     {}:  {PENDING_TASK}\n", task.id, task.title );
                                    }
                                    false => {
                                        print!("{}     {}\n", task.id, task.title );
                                    }
                                }
                            }                     
                        } else {
                            eprintln!("{n} valid Updates for ID(s) {:?}", &ids);
                        }
                    }
                    Err(e) => {eprintln!("Error updating task(s): {e} with ID(s) {:?}", &ids);}
               }
        }
    }
}

/* fn delete_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("delete: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    let post = query_task(&conn);
    match post.len() == 0 {
        true => {
            println!("\nThere are 0 {PENDING_TASK} to update!");
        }
        false => {
            print!("\n\nEnter an ID to update a {PENDING_TASK} below.\n");
            print!("\nID    TITLE\n---   -----\n");
            for task in query_task(&conn) {
                print!("{}     {}: {PENDING_TASK}\n", task.id, task.title );
            }
            print!("\n"); 
            let mut input = String::new();

            io::stdin()
               .read_line(&mut input)
               .expect("\nFailed to read input\n");

                let ids = match input.trim().parse::<i32>() {
                    Ok(n) => n,
                    Err(e) => panic!("Error reading the input {input}: {e}"),
                };

               let rtn = match update_task(ids, &conn) {
                    Ok(n) => {
                        if n > 0 {
                            println!("ID    TITLE\n---   -----");
                            for task in query_display_task(&conn) {
                                match task.done == false {
                                    true => {
                                        print!("{}     {}:  {PENDING_TASK}\n", task.id, task.title );
                                    }
                                    false => {
                                        print!("{}     {}\n", task.id, task.title );
                                    }
                                }
                            }                        
                        } else {
                            println!("Invalid ID: {input}");
                        }
                    }
                    Err(e) => {println!("Error updating task: {e}");}
               };
               rtn
        }
    }
} */

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
        //"delete" => delete_tasks(&args[2..]),
        _ => help(),
    }
}