use std::fmt;

struct VideoFilterParams {
    key: String,
    value: Option<String>,
}

impl fmt::Display for VideoFilterParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{}={}", self.key, val),
            None => write!(f, "{}", self.key),
        }
    }
}

pub struct VideoFilter {
    inputs: Vec<String>,
    outputs: Vec<String>,
    params: Vec<VideoFilterParams>,
}

impl VideoFilter {
    pub fn new() -> VideoFilter {
        VideoFilter {
            inputs: Vec::new(),
            outputs: Vec::new(),
            params: Vec::new(),
        }
    }
    pub fn input<T>(self, input: T) -> Self
    where
        T: ToString,
    {
        self.insert_input(input.to_string())
    }
    pub fn output<T>(self, output: T) -> Self
    where
        T: ToString,
    {
        self.insert_output(output.to_string())
    }
    pub fn params<K, V>(self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        let val_str = value.to_string();
        self.insert_params(
            key.to_string(),
            if val_str.len() > 0 {
                Some(val_str)
            } else {
                None
            },
        )
    }
    fn insert_input(mut self, input: String) -> Self {
        self.inputs.push(input);
        self
    }
    fn insert_output(mut self, output: String) -> Self {
        self.outputs.push(output);
        self
    }
    fn insert_params(mut self, key: String, value: std::option::Option<String>) -> Self {
        self.params.push(VideoFilterParams { key, value });
        self
    }
    fn vec_conv(&self, vec: &Vec<String>) -> String {
        self.vec_filter(vec)
            .iter()
            .map(|item| format!("[{}]", item))
            .collect::<Vec<_>>()
            .join(";")
    }
    fn vec_filter(&self, vec: &Vec<String>) -> Vec<String> {
        vec.iter()
            .filter(|item| item.len() > 0)
            .cloned()
            .collect::<Vec<_>>()
    }
    fn get_inputs(&self) -> Vec<String> {
        self.vec_filter(&self.inputs)
    }
    fn get_outputs(&self) -> Vec<String> {
        self.vec_filter(&self.outputs)
    }
    fn get_input(&self) -> String {
        self.vec_conv(&self.inputs)
    }
    fn get_output(&self) -> String {
        self.vec_conv(&self.outputs)
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (has_in, has_out, has_params) = (
            self.get_inputs().len() > 0,
            self.get_outputs().len() > 0,
            self.params.len() > 0,
        );
        let filter = if has_params {
            self.params
                .iter()
                .map(|filter| filter.to_string())
                .collect::<Vec<_>>()
                .join(";")
        } else {
            "nullsink".to_string()
        };
        if has_in && has_out && has_params {
            write!(f, "[{}]{}[{}]", self.get_input(), filter, self.get_output())
        } else if has_in && !has_out {
            write!(f, "[{}]{}", self.get_input(), filter)
        } else if has_out && !has_in {
            write!(f, "{}[{}]", filter, self.get_output())
        } else if has_params {
            write!(f, "{}", filter)
        } else {
            write!(f, "")
        }
    }
}
