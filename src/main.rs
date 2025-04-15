mod structs;

use csv::Reader;
use rand::seq::SliceRandom;
use rand::thread_rng;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    error::Error,
    fs::File,
    io::{self, Write},
};
use structs::sprint_record::SprintRecord;

fn get_user_input<'a>(prompt: &str) -> Cow<'a, str> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Cow::Owned(input.trim().to_string())
}

fn simulate_sprint(
    target_number_stories: u16,
    historic_story_points_per_sprint: &[u16],
    sprint_allocation: f32,
) -> u16 {
    let mut rng = thread_rng();
    let mut remaining = target_number_stories as f32;
    let mut iterations = 0;

    while remaining > 0.0 {
        if let Some(&random_sample) = historic_story_points_per_sprint.choose(&mut rng) {
            remaining -= random_sample as f32 * sprint_allocation;
        }
        iterations += 1;
    }

    iterations
}

fn generate_confidence_table(sprint_iterations: &[u16]) -> BTreeMap<u16, u32> {
    let mut table = BTreeMap::new();
    for &value in sprint_iterations {
        *table.entry(value).or_insert(0) += 1;
    }
    table
}

fn generate_cumulative_table(confidence_table: &BTreeMap<u16, u32>) -> BTreeMap<u16, u32> {
    let mut cumulative = 0;
    let mut table = BTreeMap::new();

    for (&key, &value) in confidence_table {
        cumulative += value;
        table.insert(key, cumulative);
    }

    table
}

fn print_cumulative_table(cumulative_table: &BTreeMap<u16, u32>, max_rounds: u32) {
    println!("Sprint\tTotal\t| Confidence (%)");
    for (&key, &value) in cumulative_table {
        let percentage = (value as f32 / max_rounds as f32) * 100.0;
        println!("{:<7}\t{:<6}\t| {:.2}%", key, value, percentage);
    }
}

fn run_sprints(
    target_number_stories: u16,
    historic_story_points_per_sprint: SmallVec<[u16; 8]>,
    sprint_allocation: f32,
    max_rounds: u16,
) {
    let mut sprint_iterations: Vec<u16> = Vec::new();

    for _ in 0..max_rounds {
        sprint_iterations.push(simulate_sprint(
            target_number_stories,
            &historic_story_points_per_sprint,
            sprint_allocation,
        ));
    }

    sprint_iterations.sort_unstable();
    let confidence_table = generate_confidence_table(&sprint_iterations);
    let cumulative_table = generate_cumulative_table(&confidence_table);
    print_cumulative_table(&cumulative_table, max_rounds as u32);
}

fn read_sprint_history() -> Result<SmallVec<[u16; 8]>, Box<dyn Error>> {
    let file = File::open("sprints.csv")?;
    let mut rdr = Reader::from_reader(file);
    let mut historic_story_points_per_sprint: SmallVec<[u16; 8]> = SmallVec::new();

    for result in rdr.deserialize() {
        let record: SprintRecord = result?;
        historic_story_points_per_sprint.push(record.completed_story_points);
    }

    Ok(historic_story_points_per_sprint)
}

fn main() {
    let historic_story_points_per_sprint = match read_sprint_history() {
        Ok(history) => history,
        Err(e) => {
            eprintln!("Failed to read sprint history: {}", e);
            SmallVec::new()
        }
    };

    const MAX_ROUNDS: u16 = 10000;
    const QUESTIONS: [&str; 3] = [
        "What is the target number of story points for the project? ",
        "What is the percentage of story points allocated for this project over the entire sprint (from 0.0 to 1.0)? ",
        "For every 10 tasks, how many new tasks are created/added to the sprint? ",
    ];
    let target_story_points: u16 = get_user_input(QUESTIONS[0]).parse().unwrap_or(0);
    let mut sprint_allocation: f32 = get_user_input(QUESTIONS[1]).parse().unwrap_or(1.0);
    let new_task_every_10: u16 = get_user_input(QUESTIONS[2]).parse().unwrap_or(3);
    sprint_allocation = sprint_allocation.clamp(0.0, 1.0);

    let new_target_story_points: u16 = ((target_story_points as f32)
        + (target_story_points as f32 * (new_task_every_10 as f32 / 10.0)))
        .round() as u16;

    run_sprints(
        new_target_story_points,
        historic_story_points_per_sprint,
        sprint_allocation,
        MAX_ROUNDS,
    );
}
