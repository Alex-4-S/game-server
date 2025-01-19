use plotters::prelude::*;
use std::time::Instant;
use wasm_game_of_life::Universe;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = vec![100, 200, 500, 1000];
    let height = vec![100, 200, 500, 1000];
    let alive_count = 500;

    // Data storage for graph
    let mut data = Vec::new();

    for &w in &width {
        for &h in &height {
            let start = Instant::now();
            let mut universe = Universe::new(w, h, alive_count);
            let init_time = start.elapsed().as_secs_f64();

            let iterations = 100;
            let start = Instant::now();
            universe.run_iterations(iterations);
            let run_time = start.elapsed().as_secs_f64();

            // Store data point: (width, height, init_time, run_time)
            data.push((w, h, init_time, run_time));
        }
    }

    // Plotting
    let root = BitMapBackend::new("performance_graph.png", (1280, 720))
        .into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Performance of Universe Simulation", ("sans-serif", 40))
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(10)
        .build_cartesian_2d(0..1000000, 0.0..35.0)?; // Adjust Y-axis range as needed

    chart.configure_mesh()
        .x_desc("Grid Size (Width x Height)")
        .y_desc("Time (seconds)")
        .draw()?;

    
    // Plot initialization time points (in BLUE)
    chart.draw_series(PointSeries::of_element(
        data.iter().map(|&(w, h, init_time, _)| {
            let grid_size = w * h;
            (grid_size as i32, init_time)
        }),
        5,
        &BLUE,
        &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        },
    ))?.label("Initialization Time")
      .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], BLUE.filled()));

    // Plot run time points (in RED)
    chart.draw_series(PointSeries::of_element(
        data.iter().map(|&(w, h, _, run_time)| {
            let grid_size = w * h;
            (grid_size as i32, run_time)
        }),
        5,
        &RED,
        &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled())
        },
    ))?.label("Runtime")
      .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], RED.filled()));

    // Add legend to the chart
    chart.configure_mesh()
        .draw()?;
        // Configure the legend and draw it
    chart.configure_series_labels()
    .position(SeriesLabelPosition::UpperRight)
    .draw()?;


    println!("Performance graph saved as 'performance_graph.png'");
    Ok(())
}
