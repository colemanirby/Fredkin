// use std::collections::{BTreeMap, HashMap};

// use plotlib::{page::Page, repr::Plot, style::{PointMarker, PointStyle}, view::ContinuousView};
// use plotters::{chart::{ChartBuilder, LabelAreaPosition}, prelude::{BitMapBackend, Circle, IntoDrawingArea}, series::LineSeries, style::{BLUE, RED, WHITE}};

// use crate::file_utils::{self, Run, RunData, ZData};

// pub fn generate_lifetime_plot_single_ss(runs_map: &BTreeMap<usize, Vec<Run>>, spin_sector: &usize) {

//     let mut data: ZData = file_utils::load_data("./data/z_data.txt".to_string());

//     println!("generating plot");
//     let mut plot_data: Vec<(f64, f64)> = Vec::new();

//     for chain_size in runs_map.keys() {
//         let mut step_total: u128 = 0;
//         let runs = runs_map.get(chain_size).unwrap();
//         let total_entries = runs.len() as f64;
//         for run in runs {
//             step_total += run.step_count;
//         }
//         let  step_total_conversion= step_total as f64;
//         let average_step_count = step_total_conversion/total_entries;
//         let chain_size_conversion = *chain_size as f64;
//         plot_data.push((chain_size_conversion, average_step_count));
//       }
//     //   let plot: Plot = Plot::new(plot_data).point_style(PointStyle::new().marker(PointMarker::Square).colour("#DD3355"));

//     //   let v = ContinuousView::new().add(plot).x_range(0.0, 32.0).y_range(0.0, 10000.0);
//     //   Page::single(&v).save("chains.svg").unwrap();

//     let lifetime_file_path = format!("data/plots/lifetime_{}.png", spin_sector);

//     let lifetime_drawing = BitMapBackend::new(lifetime_file_path.as_str(), (1920, 1080))
//         .into_drawing_area();
//     lifetime_drawing.fill(&WHITE).unwrap();

//     let final_entry = plot_data.last().unwrap();
//     let max_x = final_entry.0 + 2.0;
//     let max_y = final_entry.1 + 2.0;


//     let mut lifetime_ctx = ChartBuilder::on(&lifetime_drawing)
//         .set_label_area_size(LabelAreaPosition::Left, 60)
//         .set_label_area_size(LabelAreaPosition::Bottom, 60)
//         .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
//         .build_cartesian_2d(0.0..max_x, 0f64..max_y)
//         .unwrap();

//     lifetime_ctx.configure_mesh().draw().unwrap();

//     lifetime_ctx.draw_series(plot_data.iter().map(|point| Circle::new(*point, 5, &BLUE))).unwrap();

//     let lifetime_log_file_path = format!("data/plots/lifetime_log_{}.png", spin_sector);

//     let log_lifetime_drawing = BitMapBackend::new(lifetime_log_file_path.as_str(), (1920, 1080))
//     .into_drawing_area();
//     log_lifetime_drawing.fill(&WHITE).unwrap();

//     let max_x_log = max_x.ln() + 1.0;
//     let max_y_log = max_y.ln() + 1.0;

//     let mut lifetime_log_ctx = ChartBuilder::on(&log_lifetime_drawing)
//         .set_label_area_size(LabelAreaPosition::Left, 60)
//         .set_label_area_size(LabelAreaPosition::Bottom, 60)
//         .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
//         .build_cartesian_2d(0.9f64..max_x_log, 1.5f64..max_y_log)
//         .unwrap();

//     lifetime_log_ctx.configure_mesh().draw().unwrap();

//     lifetime_log_ctx.draw_series(plot_data.iter().map(|point| Circle::new((point.0.ln(), point.1.ln()), 5, &BLUE))).unwrap();

//     // for point in &plot_data {
//     //     let x = point.0;
//     //     let y = point.1;
//     //     println!("({x}, {y})");
//     // }

//     let mut x_2 = plot_data.get(0).unwrap().0.ln();
//     let mut y_2 = plot_data.get(0).unwrap().1.ln();
//     let mut x_1 = plot_data.get(0).unwrap().0.ln();
//     let mut y_1 = plot_data.get(0).unwrap().1.ln();
//     for point in &plot_data {
//         let x = point.0.ln();
//         let y = point.1.ln();
//         if x > x_2 {
//             x_2 = x;
//             y_2 = y;
//         }
//         else if x < x_1 {
//             x_1 = x;
//             y_1 = y;

//         }
//         // println!("({x}, {y})");
//     }

//     println!("x2: {x_2}, y2: {y_2} and x1: {x_1}, y1: {y_1}");
//     let delta_x = x_2 - x_1;
//     let delta_y = y_2 - y_1;
//     let z = delta_y/delta_x - 1.0;

//     println!("z: {z}");

//     println!("Average");

//     let mut sum_slope = 0.0;

//     for i in 0..plot_data.len() - 1 {
//         let x_2 = plot_data.get_mut(i + 1).unwrap().0;
//         let x_1 = plot_data.get_mut(i).unwrap().0;

//         let y_2 = plot_data.get_mut(i + 1).unwrap().1;
//         let y_1 = plot_data.get_mut(i).unwrap().1;

//         let delta_x = x_2.ln() - x_1.ln();
//         let delta_y = y_2.ln() - y_1.ln();

//         sum_slope += delta_y/delta_x;
//     } 

//     let average_z = sum_slope/(plot_data.len() as f64 - 1.0) - 1.0;

//     println!("average z: {average_z}");

//     // let mut z_data = HashMap::new();
//     let mut zs = Vec::new();
//     zs.push(z);
//     zs.push(average_z);
//     // z_data.insert(*spin_sector, zs);

//     data.z_data.insert(*spin_sector, zs);
//     // let data = ZData{z_data};

//     file_utils::save_data("./data/z_data.txt".to_string(), &data);

//     // ctx.draw_series(
//     //     plot_data.iter().map(|point| Circle::new(*point, 5, &BLUE))
//     // ).unwrap();


//     // root_drawing_area.fill(&WHITE).unwrap();

//     // let mut chart = ChartBuilder::on(&root_drawing_area)
//     //     .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
//     //     .unwrap();

//     // chart.draw_series(LineSeries::new(
//     //     (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
//     //     &RED
//     // )).unwrap();

// }

// pub fn generate_lifetime_plot_mutliple_ss_single_chainsize_log(min_spin_sector: usize, max_spin_sector: usize, chain_size:usize) {

//     let mut average_vec: Vec<(f64,f64)> = Vec::new();

//     let mut max_y: f64 = 0.0;

//     for i in min_spin_sector..=max_spin_sector {
//         let file_name = format!("./data/runs/run_ss_{}.json", i);
//         let run_data:RunData = file_utils::load_data(file_name);

//         let run = run_data.runs.get(&chain_size).unwrap();

//         let mut sum = 0;
//         let mut total_number_of_runs:u128 = 0;
//         for step_count in run {

//             sum+=step_count;
//             total_number_of_runs+=1;

//         }
//         let sum_conversion = sum as f64;
//         let total_number_of_runs_conversion = total_number_of_runs as f64;
//         let average = sum_conversion/total_number_of_runs_conversion;
//         if average > max_y {
//             max_y = average;
//         }

//         println!("average for ss {}: {}", i, average);
//         average_vec.push((i as f64,average));
//     }

//     let spin_sector_lifetimes = format!("data/plots/ss_{}_cs_{}_log.png", max_spin_sector, chain_size);
    
//     let spin_sector_lifetimes_drawing = BitMapBackend::new(spin_sector_lifetimes.as_str(), (1920, 1080))
//     .into_drawing_area();
//     spin_sector_lifetimes_drawing.fill(&WHITE).unwrap();

//     // let max_x_log = max_x.ln() + 1.0;
//     // max_y  = max_y + 1000.0;
//     let max_y_log = max_y.ln() + 1.0;
//     let max_x_log = (max_spin_sector as f64).ln() + 1.0;

//     let mut spin_sector_lifetime_ctx = ChartBuilder::on(&spin_sector_lifetimes_drawing)
//         .set_label_area_size(LabelAreaPosition::Left, 60)
//         .set_label_area_size(LabelAreaPosition::Bottom, 60)
//         .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
//         .build_cartesian_2d(0.0f64..max_x_log as f64 + 1.0, 0.0f64..max_y_log)
//         .unwrap();

//         spin_sector_lifetime_ctx.configure_mesh().draw().unwrap();

//         spin_sector_lifetime_ctx.draw_series(average_vec.iter().map(|point| Circle::new((point.0.ln(), point.1.ln()), 5, &BLUE))).unwrap();


// }

// pub fn generate_lifetime_plot_mutliple_ss_single_chainsize(min_spin_sector: usize, max_spin_sector: usize, chain_size:usize) {

//     let mut average_vec: Vec<(f64,f64)> = Vec::new();

//     let mut max_y: f64 = 0.0;
//     let mut min_y: f64 = 0.0;

//     for i in min_spin_sector..=max_spin_sector {
//         let file_name = format!("./data/runs/run_ss_{}.json", i);
//         let run_data:RunData = file_utils::load_data(file_name);

//         let run = run_data.runs.get(&chain_size).unwrap();

//         let mut sum = 0;
//         let mut total_number_of_runs:u128 = 0;
//         for step_count in run {

//             sum+=step_count;
//             total_number_of_runs+=1;

//         }
//         let sum_conversion = sum as f64;
//         let total_number_of_runs_conversion = total_number_of_runs as f64;
//         let average = sum_conversion/total_number_of_runs_conversion;
//         if i == min_spin_sector {
//             min_y = average;
//         } else if average < min_y {
//             min_y = average;
//         } 
        
//         if average > max_y {
//             max_y = average;
//         }

//         println!("average for ss {}: {}", i, average);
//         average_vec.push((i as f64,average));
//     }

//     let spin_sector_lifetimes = format!("data/plots/ss_{}_cs_{}.png", max_spin_sector, chain_size);
    
//     let spin_sector_lifetimes_drawing = BitMapBackend::new(spin_sector_lifetimes.as_str(), (1920, 1080))
//     .into_drawing_area();
//     spin_sector_lifetimes_drawing.fill(&WHITE).unwrap();

//     let max_x = max_spin_sector as f64 + 1.0;
//     let min_x = min_spin_sector as f64;

//     let mut spin_sector_lifetime_ctx = ChartBuilder::on(&spin_sector_lifetimes_drawing)
//         .set_label_area_size(LabelAreaPosition::Left, 60)
//         .set_label_area_size(LabelAreaPosition::Bottom, 60)
//         .caption("Fredkin Chain Lifetimes", ("sans-serif", 40))
//         .build_cartesian_2d(min_x..max_x, 950f64..max_y)
//         .unwrap();

//         spin_sector_lifetime_ctx.configure_mesh().draw().unwrap();

//         spin_sector_lifetime_ctx.draw_series(average_vec.iter().map(|point| Circle::new((point.0, point.1), 5, &BLUE))).unwrap();


// }