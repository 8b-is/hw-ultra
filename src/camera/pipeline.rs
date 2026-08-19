use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum PipelineStage {
    Idle,
    BlackLevel,
    Demosaic,
    WhiteBalance,
    ColorCorrection,
    GammaCorrection,
    NoiseReduction,
    Sharpening,
    Output,
}

static CURRENT_STAGE: AtomicU8 = AtomicU8::new(0);

fn stage_to_u8(s: PipelineStage) -> u8 {
    match s {
        PipelineStage::Idle => 0,
        PipelineStage::BlackLevel => 1,
        PipelineStage::Demosaic => 2,
        PipelineStage::WhiteBalance => 3,
        PipelineStage::ColorCorrection => 4,
        PipelineStage::GammaCorrection => 5,
        PipelineStage::NoiseReduction => 6,
        PipelineStage::Sharpening => 7,
        PipelineStage::Output => 8,
    }
}

fn u8_to_stage(v: u8) -> PipelineStage {
    match v {
        1 => PipelineStage::BlackLevel,
        2 => PipelineStage::Demosaic,
        3 => PipelineStage::WhiteBalance,
        4 => PipelineStage::ColorCorrection,
        5 => PipelineStage::GammaCorrection,
        6 => PipelineStage::NoiseReduction,
        7 => PipelineStage::Sharpening,
        8 => PipelineStage::Output,
        _ => PipelineStage::Idle,
    }
}

pub fn current_stage() -> PipelineStage {
    u8_to_stage(CURRENT_STAGE.load(Ordering::Acquire))
}

pub fn advance_stage() -> PipelineStage {
    let cur = CURRENT_STAGE.load(Ordering::Acquire);
    let next = if cur >= 8 { 0 } else { cur + 1 };
    CURRENT_STAGE.store(next, Ordering::Release);
    u8_to_stage(next)
}

pub fn reset_pipeline() {
    CURRENT_STAGE.store(0, Ordering::Release);
}

pub fn set_stage(stage: PipelineStage) {
    CURRENT_STAGE.store(stage_to_u8(stage), Ordering::Release);
}

pub fn is_processing() -> bool {
    let s = CURRENT_STAGE.load(Ordering::Acquire);
    s > 0 && s < 8
}
