use quad_url::{get_program_parameters};


pub fn get_parameter_value(arg: &str) -> String {
    let parameters = get_program_parameters();
    for parameter in parameters.iter() {
        if parameter.starts_with("--") {
            let para = &parameter[2..];
            let para = para.to_string();
            let para2: Vec<&str> = para.split('=').collect();
            if para2.len() == 2 && para2[0] == arg {
                return para2[1].to_string();
            }
        }
    }
    String::new()
}
