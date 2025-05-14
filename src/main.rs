use genetic_algorithm::plot;
use genetic_algorithm::population::Population;
use genetic_algorithm::probability::Probability;

fn main() {
    let mut mean_series: Vec<(f32, f32)> = Vec::new();

    for i in 0..30 {
        let mut population =
            Population::new(50, 0.5, Probability::build(0.01).unwrap(), 3.1..=20.0);

        let best_solutions = population.create_generations(30);
        let best = population.best();

        println!("Solution {} = ({:.3}, {:.5})", i + 1, best.0, best.1);

        let plotting_series: Vec<(f32, f32)> = best_solutions
            .iter()
            .enumerate()
            .map(|(i, (_, fitness))| ((i + 1) as f32, *fitness as f32))
            .collect();

        plot::paint(
            &format!("./plots/simulation-{}.png", i + 1),
            &format!("Simulation {}", i + 1),
            "Generation",
            plotting_series,
        )
        .expect("Failed to paint simulation plot");

        mean_series.push((
            (i + 1) as f32,
            best_solutions
                .iter()
                .map(|(_, fitness)| fitness)
                .sum::<f64>() as f32
                / best_solutions.len() as f32,
        ));
    }

    plot::paint("./plots/mean.png", "Mean", "Simulation", mean_series)
        .expect("Failed to paint mean plot");
}
