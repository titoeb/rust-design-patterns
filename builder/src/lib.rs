#[derive(Debug, PartialEq)]
pub struct ComplicatedObject {
    complicated_field: String,
}

impl ComplicatedObject {
    pub fn builder() -> ComplicatedObjectBuilder {
        ComplicatedObjectBuilder::default()
    }
}

#[derive(Default)]
pub struct ComplicatedObjectBuilder {
    complicated_field: String,
}

impl ComplicatedObjectBuilder {
    pub fn new() -> ComplicatedObjectBuilder {
        ComplicatedObjectBuilder {
            complicated_field: String::from("X"),
        }
    }
    pub fn set_complicated_field(mut self, complicated_field: String) -> ComplicatedObjectBuilder {
        self.complicated_field = complicated_field;
        self
    }
    pub fn build(self) -> ComplicatedObject {
        ComplicatedObject {
            complicated_field: self.complicated_field,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_builder() {
        let complicated_object = ComplicatedObject {
            complicated_field: String::from("Y"),
        };
        let complicated_object_from_builder = ComplicatedObjectBuilder::new()
            .set_complicated_field(String::from("Y"))
            .build();

        assert_eq!(complicated_object, complicated_object_from_builder)
    }
}
