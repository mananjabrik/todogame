use std::io;

struct Task {
    id: usize,
    name: String,
    status: String,
}

impl Task {
    fn new(id: usize, name: String) -> Self {
        Task {
            id,
            name,
            status: String::from("belum"),
        }
    }

    fn display(&self) {
        println!("ID:{}, name:{}, Status:{}", self.id, self.name, self.status,)
    }
}

fn main() {
    let mut _tasks: Vec<Task> = Vec::new();
    let mut current_id = 1;

    loop {
        println!(
            "
███    ███  █████  ███████ ██    ██ ██   ██ ██   ██  █████  ███    ██     
████  ████ ██   ██ ██      ██    ██ ██  ██  ██  ██  ██   ██ ████   ██     
██ ████ ██ ███████ ███████ ██    ██ █████   █████   ███████ ██ ██  ██     
██  ██  ██ ██   ██      ██ ██    ██ ██  ██  ██  ██  ██   ██ ██  ██ ██     
██      ██ ██   ██ ███████  ██████  ██   ██ ██   ██ ██   ██ ██   ████     
                                                                          
                                                                          
███    ██  █████  ███    ███  █████      ██████                           
████   ██ ██   ██ ████  ████ ██   ██          ██                          
██ ██  ██ ███████ ██ ████ ██ ███████       ▄███                           
██  ██ ██ ██   ██ ██  ██  ██ ██   ██       ▀▀                             
██   ████ ██   ██ ██      ██ ██   ██       ██                             
        "
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");

        let name = input.trim();
        if name == "exit" {
            break;
        }

        let task = Task::new(current_id, String::from(name));
        current_id += 1;
        println!("       
██   ██ ███████ ██████       ██  █████  ██   ██  █████  ███    ██     ███████  ██████   █████  ██      
██  ██  ██      ██   ██      ██ ██   ██ ██  ██  ██   ██ ████   ██     ██      ██    ██ ██   ██ ██      
█████   █████   ██████       ██ ███████ █████   ███████ ██ ██  ██     ███████ ██    ██ ███████ ██      
██  ██  ██      ██   ██ ██   ██ ██   ██ ██  ██  ██   ██ ██  ██ ██          ██ ██    ██ ██   ██ ██      
██   ██ ███████ ██   ██  █████  ██   ██ ██   ██ ██   ██ ██   ████     ███████  ██████  ██   ██ ███████ 
        ");
        println!("ketik m untuk mengerjakan soal");
        println!("ketik s untuk skip soal");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");

        let action = input.trim();
        if action == "m" {
            let secret_number = 42;
            loop {
                println!(
                    "
████████ ███████ ██████   █████  ██   ██      █████  ███    ██  ██████  ██   ██  █████  
    ██    ██      ██   ██ ██   ██ ██  ██      ██   ██ ████   ██ ██       ██  ██  ██   ██ 
    ██    █████   ██████  ███████ █████       ███████ ██ ██  ██ ██   ███ █████   ███████ 
    ██    ██      ██   ██ ██   ██ ██  ██      ██   ██ ██  ██ ██ ██    ██ ██  ██  ██   ██ 
    ██    ███████ ██████  ██   ██ ██   ██     ██   ██ ██   ████  ██████  ██   ██ ██   ██                                                                        
                    "
                );
                println!("masukkan angka berapa saja ");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("err");

                let guessed_number = match input.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("masukkan angka koh");
                        continue;
                    }
                };

                if guessed_number == secret_number {
                    println!("Jozz !!");
                    _tasks.push(Task {
                        id: current_id,
                        name: String::from(name),
                        status: String::from("done"),
                    });
                    break;
                } else if guessed_number > secret_number {
                    println!("angka terlalu tinggi. ingin skip atau tebak ulang? ketik (s atau u)")
                } else if guessed_number < secret_number {
                    println!("angka terlalu rendah. ingin skip atau tebak ulang? ketik (s atau u)")
                }

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Err");
                let decision = input.trim();

                if decision == "s" {
                    _tasks.push(Task {
                        id: current_id,
                        name: String::from(name),
                        status: String::from("belum"),
                    });
                    break;
                } else if decision == "u" {
                    continue;
                } else {
                    println!("becah rah");
                    continue;
                }
            }
        } else if action == "s" {
            _tasks.push(task)
        } else {
            println!("m atau s koh mak gendeng");
            break;
        }
        println!("Daftar Todo");
        for task in &_tasks {
            task.display()
        }
    }
}
