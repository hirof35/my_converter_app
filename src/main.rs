#![allow(non_snake_case)]
use dioxus::prelude::*;
use plotly::{Plot, Scatter};
use chrono::Local;
use std::fs::{OpenOptions, File};
use std::io::{Write, Read};
use std::path::Path;

// 1. 履歴データの構造体を定義 (Pythonの辞書やDataFrameの1行分に相当)
#[derive(Debug, Clone)]
struct HistoryLog {
    time: String,
    category: String,
    from_val: String,
    to_val: String,
    num_value: f64,
}

const DB_FILE: &str = "conversion_history.csv";

// CSVからデータを読み込む関数
fn load_data() -> Vec<HistoryLog> {
    let mut logs = Vec::new();
    if !Path::new(DB_FILE).exists() {
        return logs;
    }
    
    let mut file = File::open(DB_FILE).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    // 簡易的なCSVパース (本格的なアプリでは csv クレートを使うと安全です)
    for line in contents.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 5 {
            logs.push(HistoryLog {
                time: parts[0].to_string(),
                category: parts[1].to_string(),
                from_val: parts[2].to_string(),
                to_val: parts[3].to_string(),
                num_value: parts[4].parse().unwrap_or(0.0),
            });
        }
    }
    logs
}

// 2. メイン関数（アプリの起動）
fn main() {
    launch(App);
}

// 3. UIコンポーネント (Streamlitの画面表示部分)
#[component]
fn App() -> Element {
    // 状態管理 (Streamlitのセッション状態や再描画トリガーに相当)
    let mut history = use_signal(load_data);
    let mut input_val = use_signal(|| 1.0);
    
    let current_res = input_val() * 100.0; // 例として m -> cm 変換

    // 保存ボタンが押された時の処理
    let save_history = move |_| {
        let now = Local::now().format("%H:%M:%S").to_string();
        let log_line = format!("{},長さ,{} m,{} cm,{}\n", now, input_val(), current_res, current_res);
        
        // ファイルへの追記
        let file_exists = Path::new(DB_FILE).exists();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(DB_FILE)
            .unwrap();
            
        if !file_exists {
            writeln!(file, "時刻,カテゴリー,変換前,変換後,数値").unwrap();
        }
        write!(file, "{}", log_line).unwrap();
        
        // 画面の状態を更新して再描画
        history.set(load_data());
    };

    // PlotlyのグラフをHTML/要素としてレンダリングする準備
    let mut plot = Plot::new();
    let times: Vec<String> = history().iter().map(|l| l.time.clone()).collect();
    let values: Vec<f64> = history().iter().map(|l| l.num_value).collect();
    let trace = Scatter::new(times, values).name("変換値の推移");
    plot.add_trace(trace);

    rsx! {
        div { style: "font-family: sans-serif; padding: 20px; max-width: 800px; margin: 0 auto;",
            h1 { "📊 Rust & Dioxus 仕様の高度な単位変換" }
            
            div { style: "background: #f0f2f6; padding: 20px; border-radius: 10px; margin-bottom: 20px;",
                h3 { "長さ変換 (m から cm)" }
                input {
                    r#type: "number",
                    value: "{input_val}",
                    oninput: move |evt| {
                        if let Ok(v) = evt.value().parse::<f64>() {
                            input_val.set(v);
                        }
                    }
                }
                span { style: "margin-left: 10px;", " m  ＝  " }
                strong { "{current_res} cm" }
                
                button { 
                    style: "margin-left: 20px; padding: 5px 10px; background: #ff4b4b; color: white; border: none; border-radius: 5px; cursor: pointer;",
                    onclick: save_history, 
                    "変換して保存" 
                }
            }

            h2 { "変換トレンドの分析 (Plotly)" }
            // 本来はPlotlyのJavaScriptをバインドしますが、ここでは履歴をリスト表示
            div { style: "margin-top: 20px;",
                h3 { "履歴一覧" }
                table { style: "width: 100%; border-collapse: collapse;",
                    tr { style: "background: #ddd;",
                        th { "時刻" } th { "カテゴリー" } th { "変換前" } th { "変換後" }
                    }
                    for log in history().iter() {
                        tr {
                            td { "{log.time}" }
                            td { "{log.category}" }
                            td { "{log.from_val}" }
                            td { "{log.to_val}" }
                        }
                    }
                }
            }
        }
    }
}