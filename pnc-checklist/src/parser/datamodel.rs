// need to init loading specifying algo quantity for each class ?
pub enum Class {
    Guard,
    Warrior,
    Sniper,
    Specialist,
    Medic,
}
#[allow(clippy::upper_case_acronyms)]
/// List of algorithms
pub enum Algorithm {
    //offense
    TowerLimit,
    Feedforward,
    Deduction,
    Progression,
    DataRepair,
    MLRMatrix,
    //stability
    Encapsulate,
    Iteration,
    Perception,
    Overflow,
    Rationality,
    Convolution,
    //special
    Inspiration,
    LoopGain,
    SVM,
    Paradigm,
    DeltaV,
    Cluster,
    Stratagem,
    // blank slot
    BLANK
}

pub enum AlgoMainStat {
    Hashrate,
    HashratePercent,
    Atk,
    AtkPer,
    Health,
    HealthPercent,
    Haste,
    HealInc,
}
pub enum AlgoSubStat {
    CritRate,
    CritDmg,
    Hashrate,
    HashratePercent,
}
pub enum SkillCurrency {
    SkillToken,
    SkillPivot,
}
pub struct UnitSkill {
    pub passive: u32,
    pub auto: u32,
}

pub struct AlgoPiece {
    piece: Algorithm,
    main_stat: AlgoMainStat,
    sub_stat: Option<Vec<AlgoSubStat>>,
    module: u32,
}
pub struct Unit {
    name: String,
    skill_level: UnitSkill,
    algo: Vec<AlgoPiece>,
}
/// cost of skill level token
/// follow user-displayed slv so the 1st index will be 0 (slv always starts at 1)
pub const SLV_TOKEN: [u32; 10] = [0, 100, 200, 360, 560, 880, 1320, 1920, 2680, 3600];
pub const SLV_PIVOT: [u32; 10] = [0, 0, 0, 0, 0, 0, 0, 4, 8, 12];
pub const SLV_COIN: [u32; 10] = [0, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 10000];
