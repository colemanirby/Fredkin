use std::collections::HashMap;

use plotlib::{page::Page, repr::Plot, style::{PointMarker, PointStyle}, view::ContinuousView};
use plotters::{chart::{ChartBuilder, LabelAreaPosition}, prelude::{BitMapBackend, Circle, IntoDrawingArea}, series::LineSeries, style::{BLUE, RED, WHITE}};

use crate::file_utils::Run;

pub fn generate_plot(runs_map: &HashMap<usize, Vec<Run>>) {

    println!("generating plot");
    let mut plot_data: Vec<(f64, f64)> = Vec::new();

    for chain_size in runs_map.keys() {
        let mut step_total: u128 = 0;
        let runs = runs_map.get(chain_size).unwrap();
        let total_entries = runs.len() as f64;
        for run in runs {
            step_total += run.step_count;
        }
        let  step_total_conversion= step_total as f64;
        let average_step_count = step_total_conversion/total_entries;
        let chain_size_conversion = *chain_size as f64;
        plot_data.push((chain_size_conversion, average_step_count));
      }
    //   let plot: Plot = Plot::new(plot_data).point_style(PointStyle::new().marker(PointMarker::Square).colour("#DD3355"));

    //   let v = ContinuousView::new().add(plot).x_range(0.0, 32.0).y_range(0.0, 10000.0);
    //   Page::single(&v).save("chains.svg").unwrap();

    let lifetime_drawing = BitMapBackend::new("data/plots/lifetime.png", (1920, 1080))
        .into_drawing_area();
    lifetime_drawing.fill(&WHITE).unwrap();

    let mut lifetime_ctx = ChartBuilder::on(&lifetime_drawing)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
        .build_cartesian_2d(0f64..34f64, 0f64..15000f64)
        .unwrap();

    lifetime_ctx.configure_mesh().draw().unwrap();

    lifetime_ctx.draw_series(plot_data.iter().map(|point| Circle::new(*point, 5, &BLUE))).unwrap();

    let log_lifetime_drawing = BitMapBackend::new("data/plots/lifetime_log.png", (1920, 1080))
    .into_drawing_area();
    log_lifetime_drawing.fill(&WHITE).unwrap();

    let mut lifetime_log_ctx = ChartBuilder::on(&log_lifetime_drawing)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 60)
        .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
        .build_cartesian_2d(0.9f64..1.6f64, 1.5f64..4.2f64)
        .unwrap();

    lifetime_log_ctx.configure_mesh().draw().unwrap();

    lifetime_log_ctx.draw_series(plot_data.iter().map(|point| Circle::new((point.0.ln(), point.1.ln()), 5, &BLUE))).unwrap();

    // for point in &plot_data {
    //     let x = point.0;
    //     let y = point.1;
    //     println!("({x}, {y})");
    // }

    let mut x_2 = plot_data.get(0).unwrap().0.ln();
    let mut y_2 = plot_data.get(0).unwrap().1.ln();
    let mut x_1 = plot_data.get(0).unwrap().0.ln();
    let mut y_1 = plot_data.get(0).unwrap().1.ln();
    for point in &plot_data {
        let x = point.0.ln();
        let y = point.1.ln();
        if x > x_2 {
            x_2 = x;
            y_2 = y;
        }
        else if x < x_1 {
            x_1 = x;
            y_1 = y;

        }
        // println!("({x}, {y})");
    }

    println!("x2: {x_2}, y2: {y_2} and x1: {x_1}, y1: {y_1}");
    let delta_x = x_2 - x_1;
    let delta_y = y_2 - y_1;
    let z = delta_y/delta_x;

    println!("z: {z}");
    // ctx.draw_series(
    //     plot_data.iter().map(|point| Circle::new(*point, 5, &BLUE))
    // ).unwrap();


    // root_drawing_area.fill(&WHITE).unwrap();

    // let mut chart = ChartBuilder::on(&root_drawing_area)
    //     .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
    //     .unwrap();

    // chart.draw_series(LineSeries::new(
    //     (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
    //     &RED
    // )).unwrap();

}