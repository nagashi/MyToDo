use mytodo::db::*;
use std::env;

const PENDING_TASK: &str = "<PendingTask>";

fn help() {
    println!("subcommands:");
    println!("    new: String argument needed to create a new task");
    println!("    show<id, title>: show pending task(s)");
    println!("    update<id, title>: update pending task(s)");
    println!("    delete: No arguments needed to call delete_tasks()");
    println!("    max: No arguments needed to call max_title()");
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
        println!("\nmax: No argument needed.");
        //help();
        return;
    }

    println!("{}", max_title());
}

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    match query_task(&mut establish_connection()).len() > 0 {
        false => {
            println!("\nThere are 0 {PENDING_TASK} to show!");
        }
        true => {
            let nbr = display_header();

            query_task(&mut establish_connection())
                .into_iter()
                .for_each(|task| {
                    print!(
                        "{}{}{PENDING_TASK}\n",
                        task.id
                            .to_string()
                            .pad_to_width_with_alignment(6, Alignment::Left),
                        task.title
                            .to_string()
                            .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                    );
                });
        }
    };
}

fn update_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("update: unexpected argument");
        help();
        return;
    }

    match query_task(&mut establish_connection()).len() > 0 {
        false => {
            println!("\nThere are 0 {PENDING_TASK} to update!");
        }
        true => {
            print!("\n\nEnter one or more ID's seperated by a\nspace to update a {PENDING_TASK} below.\n");
            let nbr = display_header();

            query_task(&mut establish_connection())
                .into_iter()
                .for_each(|task| {
                    print!(
                        "{}{}{PENDING_TASK}\n",
                        task.id
                            .to_string()
                            .pad_to_width_with_alignment(6, Alignment::Left),
                        task.title
                            .to_string()
                            .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                    );
                });
            print!("\n");

            /*
            Initialize ids with the user's input
            to Vec<i32>.
            */
            let ids = read_input();

            match update_task(ids.clone(), &mut establish_connection()) {
                Ok(n) => {
                    if n > 0 {
                        let nbr = display_header();

                        query_display_task(&mut establish_connection())
                            .into_iter()
                            .for_each(|task| match task.done == false {
                                true => {
                                    print!(
                                        "{}{}{PENDING_TASK}\n",
                                        task.id
                                            .to_string()
                                            .pad_to_width_with_alignment(6, Alignment::Left),
                                        task.title
                                            .to_string()
                                            .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                                    );
                                }
                                false => {
                                    print!(
                                        "{}{}\n",
                                        task.id
                                            .to_string()
                                            .pad_to_width_with_alignment(6, Alignment::Left),
                                        task.title
                                            .to_string()
                                            .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                                    );
                                }
                            });
                    } else {
                        println!("{n} valid Updates for ID(s) {:?}", &ids);
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

    match query_display_task(conn).len() > 0 {
        false => {
            println!("\nThere are no tasks to delete!");
        }
        true => {
            println!("\n\nEnter one or more numerical ID's seperated\nby a space to delete one or more tasks below.");
            let nbr = display_header();

            query_display_task(conn)
                .into_iter()
                .for_each(|task| match task.done == false {
                    true => {
                        print!(
                            "{}{}{PENDING_TASK}\n",
                            task.id
                                .to_string()
                                .pad_to_width_with_alignment(6, Alignment::Left),
                            task.title
                                .to_string()
                                .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                        );
                    }
                    false => {
                        print!(
                            "{}{}\n",
                            task.id
                                .to_string()
                                .pad_to_width_with_alignment(6, Alignment::Left),
                            task.title
                                .to_string()
                                .pad_to_width_with_alignment(nbr + 3, Alignment::Left)
                        );
                    }
                });
            print!("\n");

            /*
            Initialize ids with the user's input
            to Vec<i32>.
            */
            let ids = read_input();

            match delete_task(ids.clone(), conn) {
                Ok(n) => match n > 0 {
                    true => {
                        println!("{n} valid deletes for IDs {:?}", &ids);
                    }
                    false => {
                        println!("{n} valid deletes for IDs {:?}", &ids);
                    }
                },
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
