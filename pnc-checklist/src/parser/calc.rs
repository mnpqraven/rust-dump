use std::fmt::Display;

use crate::parser::datamodel::SLV_COIN;

use super::datamodel::UnitSkill;
use super::datamodel::{SLV_PIVOT, SLV_TOKEN};

/// Tokens and pivots a unit would need to max out its skill
pub struct SkillResourceRequirement {
    token: u32,
    pivot: u32,
    coin: u32
}

impl Display for SkillResourceRequirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unit needs {} tokens, {} pivots and {} DGC Coin", self.token, self.pivot, self.coin)
    }
}

/// calculates total tokens + pivots needed for a unit
///
/// * `current_slv`: unit's current slv
fn calc_slv(current_slv: UnitSkill) -> SkillResourceRequirement {
    fn slice_sum_2end(vector: Vec<u32>, index: u32) -> u32 {
        let (_, right) = vector.split_at(index.try_into().unwrap());
        right.iter().sum()
    }

    let total_token_auto = slice_sum_2end(SLV_TOKEN.to_vec(), current_slv.auto);
    let total_pivot_auto = slice_sum_2end(SLV_PIVOT.to_vec(), current_slv.auto);
    let total_coin_auto = slice_sum_2end(SLV_COIN.to_vec(), current_slv.auto);
    dbg!(total_coin_auto);

    let total_token_passive = slice_sum_2end(SLV_TOKEN.to_vec(), current_slv.passive);
    let total_pivot_passive = slice_sum_2end(SLV_PIVOT.to_vec(), current_slv.passive);
    let total_coin_passive = slice_sum_2end(SLV_COIN.to_vec(), current_slv.passive);
    dbg!(total_coin_passive);

    SkillResourceRequirement {
        token: (total_token_auto + total_token_passive),
        pivot: (total_pivot_auto + total_pivot_passive),
        coin: (total_coin_auto + total_coin_passive)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::datamodel::UnitSkill;

    use super::calc_slv;

    #[test]
    fn test_skill_total() {
        let unit_skill = UnitSkill {
            passive: 5,
            auto: 8,
        };
        let calc = calc_slv(unit_skill);
        assert_eq!(calc.token, 16680);
        assert_eq!(calc.pivot, 44);
    }
    #[test]
    fn test_display() {
        let unit_skill = UnitSkill {
            passive: 5,
            auto: 8,
        };
        let calc = calc_slv(unit_skill);
        println!("{}", calc);
    }
}
