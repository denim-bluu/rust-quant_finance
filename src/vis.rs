use crate::math;
use plotters::prelude::*;

pub fn draw_line_chart(chart_name: &str, y: &Vec<f64>) {
    let mut line_data = Vec::new();
    for i in 0..y.len() {
        line_data.push((i, y[i]))
    }
    let dir = ".";
    let filepath = format!("{}/{}.png", dir, &chart_name);
    let root = BitMapBackend::new(&filepath, (1280, 960)).into_drawing_area();
    root.fill(&WHITE).expect("Error filling background.");
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .caption(chart_name, ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(0..y.len(), math::array_min(&y)..math::array_max(&y))
        .unwrap();

    chart
        .configure_mesh()
        .light_line_style(&WHITE)
        .draw()
        .unwrap();

    root.present().expect(&format!("Unable to write result to file please make sure directory '{}' exists under the current dir", &dir));
    chart
        .draw_series(LineSeries::new(line_data, BLUE.stroke_width(2)))
        .unwrap()
        .label(chart_name)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x, y)], &BLUE));
    chart
        .configure_series_labels()
        .position(SeriesLabelPosition::UpperMiddle)
        .label_font(("sans-serif", 30.0).into_font())
        .background_style(WHITE.filled())
        .draw()
        .unwrap();
}
