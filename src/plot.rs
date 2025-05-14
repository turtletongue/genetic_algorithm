use plotters::prelude::*;

pub fn paint(
    filename: &str,
    chart_label: &str,
    x_label: &str,
    series: Vec<(f32, f32)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(chart_label, ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(1.0_f32..30.0_f32, 0.013_f32..0.017_f32)?;

    chart
        .configure_mesh()
        .set_all_tick_mark_size(0.0001)
        .disable_x_mesh()
        .x_desc(x_label)
        .disable_y_mesh()
        .y_desc("Fitness")
        .y_label_formatter(&|v| format!("{:.3}", v))
        .draw()?;

    chart.draw_series(LineSeries::new(series, &BLUE))?;

    // To avoid the IO failure being ignored silently, we manually call the present
    // function
    root.present().expect("Unable to write result to file");

    Ok(())
}
