use std::fmt;
#[derive(Clone)]
pub struct VideoFilterParams {
    key: Option<String>,
    value: Option<String>,
    params: Option<Vec<Self>>,
    is_sub_params: bool,
}

impl VideoFilterParams {
    pub fn new() -> Self {
        Self {
            key: None,
            value: None,
            params: None,
            is_sub_params: false,
        }
    }

    pub fn is_sub_params(mut self) -> Self {
        self.is_sub_params = true;
        self
    }

    pub fn kv<K, V>(key: K, val: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        VideoFilterParams::new().key(key).value(val)
    }

    pub fn key<T>(mut self, key: T) -> Self
    where
        T: ToString,
    {
        self.key = Some(key.to_string());
        self
    }

    pub fn value<T>(mut self, value: T) -> Self
    where
        T: ToString,
    {
        let value = value.to_string();
        self.params = None;
        self.value = if value.len() > 0 { Some(value) } else { None };
        self
    }

    pub fn params<T>(self, param: T) -> Self
    where
        T: ToString,
    {
        self.params_raw(Self::new().key(param))
    }

    pub fn params_raw(mut self, param: Self) -> Self {
        if let Some(mut params) = self.params {
            params.push(param);
            self.value = None;
            self.params = Some(params);
            self
        } else {
            self.params = Some(Vec::new());
            self.params_raw(param)
        }
    }
}

impl Default for VideoFilterParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ToString for VideoFilterParams {
    fn to_string(&self) -> String {
        if let Some(key) = &self.key {
            if let Some(value) = &self.value {
                if self.is_sub_params {
                    format!("{}='{}'", key, value)
                } else {
                    format!("{}={}", key, value)
                }
            } else if let (Some(params), false) = (&self.params, self.is_sub_params) {
                format!(
                    "{}={}",
                    key,
                    params
                        .iter()
                        .cloned()
                        .map(|params| params.is_sub_params().to_string())
                        .collect::<Vec<_>>()
                        .join(":")
                )
            } else {
                format!("{}", key)
            }
        } else {
            "".to_owned()
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
    pub fn input<T>(mut self, input: T) -> Self
    where
        T: ToString,
    {
        self.inputs.push(input.to_string());
        self
    }
    pub fn output<T>(mut self, output: T) -> Self
    where
        T: ToString,
    {
        self.outputs.push(output.to_string());
        self
    }
    pub fn params_raw(mut self, params: VideoFilterParams) -> Self {
        self.params.push(params);
        self
    }
    pub fn params_key<K>(mut self, key: K) -> Self
    where
        K: ToString,
    {
        self.params.push(VideoFilterParams::new().key(key));
        self
    }
    pub fn params<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: ToString,
    {
        self.params.push(VideoFilterParams::kv(key, value));
        self
    }
    fn vec_conv(&self, vec: &Vec<String>) -> String {
        self.vec_filter(vec)
            .iter()
            .map(|item| format!("[{}]", item))
            .collect::<Vec<_>>()
            .join("")
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
                .join(",")
        } else {
            "nullsink".to_string()
        };
        if has_in && has_out && has_params {
            write!(f, "{}{}{}", self.get_input(), filter, self.get_output())
        } else if has_in && !has_out {
            write!(f, "{}{}", self.get_input(), filter)
        } else if has_out && !has_in {
            write!(f, "{}{}", filter, self.get_output())
        } else if has_params {
            write!(f, "{}", filter)
        } else {
            write!(f, "")
        }
    }
}
