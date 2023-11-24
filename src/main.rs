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
        println!("Masukkan nama atau 'exit' untuk kluar");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");

        let name = input.trim();
        if name == "exit" {
            break;
        }

        let task = Task::new(current_id, String::from(name));
        current_id += 1;

        println!("mau mengerjakan soal atau skip ? (mengerjakan/skip)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");

        let action = input.trim();
        if action == "mengerjakan" {
            let secret_number = 42;
            loop {
                println!("Tebang angka masukkan berapa saja :");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("err");

                let guessed_number = match input.trim().parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Masukkan angka yang valid!");
                        continue;
                    }
                };

                if guessed_number == secret_number {
                    println!("selamat anda berhasil!");
                    _tasks.push(Task {
                        id: current_id,
                        name: String::from(name),
                        status: String::from("done"),
                    });
                    break;
                } else if guessed_number > secret_number {
                    println!("angka terlalu tinggi. ingin skip atau tebak ulang? (skip/ulang)")
                } else if guessed_number < secret_number {
                    println!("angka terlalu rendah. ingin skip atau tebak ulang? (skip/ulang)")
                }

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Err");
                let decision = input.trim();

                if decision == "skip" {
                    _tasks.push(Task {
                        id: current_id,
                        name: String::from(name),
                        status: String::from("belum"),
                    });
                } else if decision == "ulang" {
                    continue;
                } else {
                    println!("perintah tidak valid, status tetap");
                    _tasks.push(Task {
                        id: current_id,
                        name: String::from(name),
                        status: String::from("belum"),
                    })
                }
            }
        } else if action == "skip" {
            _tasks.push(task)
        } else {
            println!("perintah tidak valid. Status tetap 'belum'");
            _tasks.push(task)
        }

        println!("Daftar Todo");
        for task in &_tasks {
            task.display()
        }
    }
}
