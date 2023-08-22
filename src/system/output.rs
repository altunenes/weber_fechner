
use std::fs::OpenOptions;
use std::io::Write;
use crate::PARTICIPANT_ID;

pub fn print_final_results(final_results: &Vec<(usize, usize, String, f32)>) {
    println!("---Final Results---");
    let mut csv_data = String::from("Trial,Num_Left,Num_Right,Result,Response_Time\n");
    let mut correct_count = 0;
    for (trial, (num_left, num_right, result, response_time)) in final_results.iter().enumerate() {
        if result == "Correct" {
            correct_count += 1;
        }
        println!("Trial {}: Left = {}, Right = {}, Result = {}, Response Time = {}", trial+1, num_left, num_right, result, response_time);
        csv_data += &format!("{},{},{},{},{}\n", trial+1, num_left, num_right, result, response_time);
    }
    let mean_accuracy: f32 = correct_count as f32 / final_results.len() as f32;
    let mean_correct_rt: f32 = final_results
    .iter()
    .filter(|(_, _, is_correct, _)| is_correct == "Correct") 
    .map(|(_, _, _, response_time)| response_time)
    .sum::<f32>() / correct_count as f32;
    println!("Mean Accuracy: {}", mean_accuracy);
    csv_data += &format!("\nMean Accuracy: {}\n", mean_accuracy);
    println!("Mean Correct Response Time: {}", mean_correct_rt);
    csv_data += &format!("Mean Correct Response Time: {}\n", mean_correct_rt);
    let file_name = format!("participant_{}.csv", PARTICIPANT_ID);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsValue;
        use wasm_bindgen::JsCast;
        use web_sys::{Blob, Url, HtmlAnchorElement};
        let csv_array = js_sys::Array::new();
        csv_array.push(&JsValue::from_str(&csv_data));
        let blob = Blob::new_with_str_sequence(&csv_array).unwrap();
        
        let url = Url::create_object_url_with_blob(&blob).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let a: HtmlAnchorElement = document.create_element("a").unwrap().dyn_into().unwrap();
        a.set_href(&url);
        a.set_download(&file_name);
        a.style().set_property("display", "none").unwrap();
        document.body().unwrap().append_child(&a).unwrap();
        a.click();
        document.body().unwrap().remove_child(&a).unwrap();
        Url::revoke_object_url(&url).unwrap();
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_name)
            .unwrap();
        file.write_all(csv_data.as_bytes()).unwrap();
        println!("Data saved to {}", &file_name);
    }

}
