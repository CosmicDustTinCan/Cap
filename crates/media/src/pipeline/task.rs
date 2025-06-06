use flume::{Receiver, Sender};

use crate::pipeline::{MediaError, PipelineControlSignal};

const DEFAULT_QUEUE_SIZE: usize = 2048;

pub type PipelineReadySignal = Sender<Result<(), MediaError>>;

pub trait PipelineSourceTask: Send {
    type Clock;

    fn run(
        &mut self,
        clock: Self::Clock,
        ready_signal: PipelineReadySignal,
        control_signal: PipelineControlSignal,
    );

    fn queue_size(&self) -> usize {
        DEFAULT_QUEUE_SIZE
    }
}

pub trait PipelineSinkTask<Input>: Send {
    fn run(&mut self, ready_signal: PipelineReadySignal, input: &Receiver<Input>);

    fn finish(&mut self);
}
