
use std::path::Path;
use std::time::Duration;
use plotters::prelude::*;
use wasmer::{Store, Module, Instance, imports, Value};
use benchmark_shared_data_structures::MultiplyParams;

mod self_ref_test;
mod native_test;
mod pair_test;
mod bincode_test;
mod bytemuck_test;

#[derive(Clone)]
struct Report {
    name : String,
    average : f64,
    standard_dev: f64,
}

fn main() {

    let mut benchmarks: Vec<(String, fn(&Store) -> Duration)> = vec![];

    benchmarks.push(("Native Test".to_string(), native_test::native_test));
    benchmarks.push(("Pair, Preload".to_string(),pair_test::pair_preload));
    benchmarks.push(("Pair, Hotload".to_string(),pair_test::pair_hotload));
    benchmarks.push(("Pair, Preload, Cached".to_string(),pair_test::pair_preload_cached));
    benchmarks.push(("Pair, Hotload, Cached".to_string(),pair_test::pair_hotload_cached));
    benchmarks.push(("Pair, Preload, Self-referential Struct".to_string(), self_ref_test::ouroboros_preload));
    benchmarks.push(("Pair, Hotload, Self-referential Struct".to_string(), self_ref_test::ouroboros_hotload));
    benchmarks.push(("Bincode".to_string(),bincode_test::bincode_test));
    benchmarks.push(("Bincode, Cached".to_string(),bincode_test::bincode_cached));
    benchmarks.push(("Bytemuck".to_string(),bytemuck_test::bytemuck_test));
    benchmarks.push(("Bytemuck, Cached".to_string(),bytemuck_test::bytemuck_cached_test));
    benchmarks.push(("Bytemuck, Static".to_string(),bytemuck_test::bytemuck_fixed_test));
    benchmarks.push(("Bytemuck, Static, Cached".to_string(),bytemuck_test::bytemuck_cached_fixed_test));

    let runs = 5;

    let store = Store::default();

    let results : Vec<Report> = benchmarks.iter().map(|(name, func)|{

        let mut times : Vec<Duration> = vec![];

        for _ in 0..runs {
            let time = func(&store);
            times.push(time)
        }

        let average: f64 = times.iter().map(|i| i.as_secs_f64() ).sum::<f64>()/ runs as f64;

        let standard_dev : f64 = (times.iter()
            .map(|i| i.as_secs_f64()-average)
            .map(|i| i.powf(2.0))
            .sum::<f64>())/ runs as f64;

        Report{
            name: name.clone(),
            average,
            standard_dev
        }
    }).collect();

    for r in results.clone() {
        println!("Benchmark: {:?} Average: {:?} Standard Dev: {:?}", r.name, r.average, r.standard_dev)
    }

    {
        let mode = if cfg!(debug_assertions){
            "Debug"
        } else {
            "Release"
        };

        const D_YELLOW : RGBColor = RGBColor{
            0: 185,
            1: 185,
            2: 0
        };

        const D_GREEN : RGBColor = RGBColor{
            0: 0,
            1: 185,
            2: 0
        };

        const DR_YELLOW : RGBColor = RGBColor{
            0: 100,
            1: 100,
            2: 0
        };

        let stroke = 10;

        let colors: Vec<ShapeStyle> = vec![BLACK.filled(),
                                         YELLOW.mix(0.7).filled(),
                                         YELLOW.mix(0.4).filled(),
                                         D_YELLOW.mix(0.7).filled(),
                                         D_YELLOW.mix(0.4).filled(),
                                         DR_YELLOW.mix(0.7).filled(),
                                         DR_YELLOW.mix(0.4).filled(),
                                         BLUE.mix(0.7).filled(),
                                         BLUE.mix(0.4).filled(),
                                         GREEN.mix(0.7).filled(),
                                         GREEN.mix(0.4).filled(),
                                         D_GREEN.mix(0.7).filled(),
                                         D_GREEN.mix(0.4).filled(), ];
        let path = format!("bench_results/test_{}.png",mode);
        let path = Path::new(path.as_str());
        let root = BitMapBackend::new(path,(800,600)).into_drawing_area();

        root.fill(&WHITE);

        let caption = format!("Micro Benchmarks - {}",mode);

        let y_spec = if cfg!(debug_assertions){
            0f64..1f64
        } else {
            0f64..0.06f64
        };

        let mut chart = ChartBuilder::on(&root)
            .set_all_label_area_size(50)
            .caption(caption.as_str(), ("sans-serif",30.0))
            .build_cartesian_2d(0u32..(results.len() as u32),y_spec)
            .unwrap();

        &chart.configure_mesh()
            .y_desc("Average Runtime (s)")
            .draw().unwrap();

        for (i,(r,c)) in results.into_iter().zip(colors).enumerate() {

            let c = c.clone();
            let name = r.name.clone();

            chart.draw_series(
                vec![r].into_iter().map(|r|{
                    Rectangle::new([(i as u32,0f64),((i+1) as u32,r.average)], c.clone())
                })
            ).unwrap().label(name).legend(move |(x,y)| Rectangle::new([(x, y-5), (x + 20, y+5)], c.clone()));
        }

        chart.configure_series_labels()
            .border_style(&BLACK)
            .background_style(&WHITE)
            .position(SeriesLabelPosition::UpperLeft)
            .draw().unwrap();

        root.present().unwrap()
    }
}




