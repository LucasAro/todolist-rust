use reqwest;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
enum Frequency {
    Diária,
    Semanal,
    Mensal,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
    frequency: Frequency,
}

impl Task {
    fn new(description: String, frequency: Frequency) -> Self {
        Task {
            description,
            completed: false,
            frequency,
        }
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    loop {
        clear_terminal();
        println!("\x1b[1;34m--- To-Do List ---\x1b[0m");
        println!("\x1b[1;33m1. Adicionar tarefa\x1b[0m");
        println!("\x1b[1;33m2. Listar tarefas\x1b[0m");
        println!("\x1b[1;33m3. Marcar tarefa como concluída\x1b[0m");
        println!("\x1b[1;33m4. Apagar tarefa\x1b[0m");
        println!("\x1b[1;33m5. Sair\x1b[0m");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Falha ao ler entrada");

        match choice.trim() {
            "1" => add_task().await,
            "2" => list_tasks().await,
            "3" => complete_task().await,
            "4" => delete_task().await,
            "5" => {
                println!("\x1b[1;32mSaindo...\x1b[0m");
                break;
            }
            _ => println!("\x1b[1;31mOpção inválida\x1b[0m"),
        }
    }
}

async fn add_task() {
    println!("\x1b[1;36mDigite a descrição da tarefa:\x1b[0m");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("Falha ao ler entrada");

    println!("\x1b[1;36mEscolha a frequência da tarefa:\x1b[0m");
    println!("1. Diária");
    println!("2. Semanal");
    println!("3. Mensal");

    let mut frequency_choice = String::new();
    io::stdin()
        .read_line(&mut frequency_choice)
        .expect("Falha ao ler entrada");

    let frequency = match frequency_choice.trim() {
        "1" => Frequency::Diária,
        "2" => Frequency::Semanal,
        "3" => Frequency::Mensal,
        _ => {
            println!("\x1b[1;31mOpção inválida, definindo como Diária.\x1b[0m");
            Frequency::Diária
        }
    };

    let task = Task::new(description.trim().to_string(), frequency);

    println!("\x1b[1;33mCarregando...\x1b[0m");
    let mut tasks = load_tasks().await;
    tasks.push(task);
    save_tasks(&tasks).await;
    println!("\x1b[1;32mTarefa adicionada com sucesso!\x1b[0m");
}

async fn list_tasks() {
    clear_terminal();

    println!("\x1b[1;33mCarregando...\x1b[0m");
    let tasks = load_tasks().await;

    println!("\x1b[1;34m--- Lista de Tarefas ---\x1b[0m");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "\x1b[1;32mConcluída\x1b[0m" } else { "\x1b[1;31mPendente\x1b[0m" };
        let frequency = match task.frequency {
            Frequency::Diária => "Diária",
            Frequency::Semanal => "Semanal",
            Frequency::Mensal => "Mensal",
        };
        println!("{}: {} [{} - {}]", index + 1, task.description, status, frequency);
    }

    println!("\n\x1b[1;36mPressione Enter para voltar ao menu principal.\x1b[0m");
    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Falha ao ler entrada");
    clear_terminal();
}

async fn complete_task() {
    clear_terminal();
    println!("\x1b[1;33mCarregando...\x1b[0m");

    let tasks = load_tasks().await;
    println!("\x1b[1;34m--- Escolha uma tarefa para marcar como concluída ---\x1b[0m");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "\x1b[1;32mConcluída\x1b[0m" } else { "\x1b[1;31mPendente\x1b[0m" };
        println!("{}: {} [{}]", index + 1, task.description, status);
    }

    println!("\x1b[1;36mDigite o número da tarefa:\x1b[0m");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler entrada");

    let mut tasks = tasks;
    if let Ok(i) = index.trim().parse::<usize>() {
        if i > 0 && i <= tasks.len() {
            tasks[i - 1].completed = true;
            save_tasks(&tasks).await;
            println!("\x1b[1;32mTarefa marcada como concluída!\x1b[0m");
        } else {
            println!("\x1b[1;31mNúmero inválido!\x1b[0m");
        }
    } else {
        println!("\x1b[1;31mEntrada inválida!\x1b[0m");
    }
}

async fn delete_task() {
    clear_terminal();
    println!("\x1b[1;33mCarregando...\x1b[0m");

    let tasks = load_tasks().await;
    println!("\x1b[1;34m--- Escolha uma tarefa para apagar ---\x1b[0m");
    for (index, task) in tasks.iter().enumerate() {
        let status = if task.completed { "\x1b[1;32mConcluída\x1b[0m" } else { "\x1b[1;31mPendente\x1b[0m" };
        println!("{}: {} [{}]", index + 1, task.description, status);
    }

    println!("\x1b[1;36mDigite o número da tarefa:\x1b[0m");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler entrada");

    let mut tasks = tasks;
    if let Ok(i) = index.trim().parse::<usize>() {
        if i > 0 && i <= tasks.len() {
            tasks.remove(i - 1);
            save_tasks(&tasks).await;
            println!("\x1b[1;32mTarefa apagada com sucesso!\x1b[0m");
        } else {
            println!("\x1b[1;31mNúmero inválido!\x1b[0m");
        }
    } else {
        println!("\x1b[1;31mEntrada inválida!\x1b[0m");
    }
}

async fn load_tasks() -> Vec<Task> {
    let url = "https://api.jsonbin.io/v3/b/<BIN_ID_AQUI>/latest";
    let api_key = "SUA_APIKEY_AQUI";

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-Master-Key", api_key)
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json_data) => {
                let tasks: Vec<Task> = serde_json::from_value(json_data["record"].clone()).unwrap_or_else(|_| Vec::new());
                tasks
            }
            Err(_) => {
                println!("\x1b[1;31mErro ao parsear JSON.\x1b[0m");
                Vec::new()
            }
        },
        Err(_) => {
            println!("\x1b[1;31mErro ao fazer requisição.\x1b[0m");
            Vec::new()
        }
    }
}

async fn save_tasks(tasks: &[Task]) {
    let url = "https://api.jsonbin.io/v3/b/<BIN_ID_AQUI>";
    let api_key = "SUA_APIKEY_AQUI";

    let client = reqwest::Client::new();
    let response = client
        .put(url)
        .header("X-Master-Key", api_key)
        .header("Content-Type", "application/json")
        .json(tasks)
        .send()
        .await;

    match response {
        Ok(_) => println!("\x1b[1;32mTarefas salvas com sucesso!\x1b[0m"),
        Err(_) => println!("\x1b[1;31mFalha ao salvar as tarefas no servidor\x1b[0m"),
    }
}
