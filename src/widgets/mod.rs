pub mod clock;

pub trait UsesHorizonData {
    fn update(&self);
}
