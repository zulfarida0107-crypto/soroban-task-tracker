#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub category: Symbol,
    pub is_completed: bool,
}

const TASK_LIST: Symbol = symbol_short!("TASK_LST");

#[contract]
pub struct TaskTrackerContract;

#[contractimpl]
impl TaskTrackerContract {
    // Ambil semua task
    pub fn get_tasks(env: Env) -> Vec<Task> {
        env.storage()
            .instance()
            .get(&TASK_LIST)
            .unwrap_or(Vec::new(&env))
    }

    // Tambah task
    pub fn add_task(env: Env, title: String, category: Symbol) -> u64 {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_LIST)
            .unwrap_or(Vec::new(&env));

        let id = env.prng().gen::<u64>();

        let task = Task {
            id,
            title,
            category,
            is_completed: false,
        };

        tasks.push_back(task);
        env.storage().instance().set(&TASK_LIST, &tasks);

        id
    }

    // Tandai selesai
    pub fn complete_task(env: Env, id: u64) -> bool {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_LIST)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.is_completed = true;
                tasks.set(i, task);
                env.storage().instance().set(&TASK_LIST, &tasks);
                return true;
            }
        }

        false
    }

    // Hapus task
    pub fn remove_task(env: Env, id: u64) -> bool {
        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&TASK_LIST)
            .unwrap_or(Vec::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);
                env.storage().instance().set(&TASK_LIST, &tasks);
                return true;
            }
        }

        false
    }
}