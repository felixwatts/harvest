use crate::plan::Plan;
use crate::constant::SEASON_LENGTH;
use core::fmt::Display;
use crate::params::Params;

pub struct Formatter<'a> {
    params: &'a Params,
    plan: &'a Plan
}

impl<'a> Formatter<'a> {
    pub fn new(params: &'a Params, plan: &'a Plan) -> Formatter<'a> {
        Formatter{
            params: params,
            plan: plan
        }
    }
}

impl<'a> Display for Formatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let evaluator = crate::evaluator::Evaluator::new(self.params, self.plan);

        let harvest_plan = evaluator.get_harvest_plan();

        write!(f, "{:>11}", "Week")?;
        for v in 1..self.params.varieties.len() {
            let variety = &self.params.varieties[v];
            let mut name = variety.name.clone();
            name.truncate(9);
            write!(f, "{:>11}", name)?;
        }
        writeln!(f)?;

        for week in 0..SEASON_LENGTH {
            write!(f, "{:>11}", week)?;
            for v in 1..self.params.varieties.len() {
                match self.params.varieties[v].is_harvestable_in_week(week) {
                    true => {
                        let harvestable_units = harvest_plan[v][week];
                        let saturation = harvestable_units as f32  / self.params.num_baskets as f32;
                        write!(f, "{:>10.0}%", saturation * 100.0)?;
                    },
                    false => {
                        write!(f, "           ")?;
                    }

                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "Utilization: {:.0}%", evaluator.get_bed_utilization() * 100.0)?;
        writeln!(f, "Saturation: {:.0}%", evaluator.get_market_saturation() * 100.0)?;
        writeln!(f, "Profit: {:.2}", evaluator.get_profit() as f32 / 100.0)
    }
}