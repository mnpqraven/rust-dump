pub enum Class {
    Guard,
    Warrior,
    Sniper,
    Specialist,
    Medic
}

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
    Stratagem
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
    SkillPivot
}
pub struct UnitSkill {
    passive: u32,
    auto: u32
}

pub struct AlgoPiece {
    piece: Algorithm,
    main_stat: AlgoMainStat,
    sub_stat: Option<Vec<AlgoSubStat>>,
    module: u32
}
pub struct Unit {
    name: String,
    skill_level: UnitSkill,
    algo: Vec<AlgoPiece>
}
