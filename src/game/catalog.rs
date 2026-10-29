#[derive(Debug, Clone)]
pub struct AreaCandidate {
    pub id: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub static HUNTS_CATALOG: &[AreaCandidate] = &[
        AreaCandidate { id: "pallet-town", min_level: 1, max_level: 15, base_exp: 95.0, base_gold: 40.0 },
        AreaCandidate { id: "viridian-forest", min_level: 15, max_level: 32, base_exp: 260.0, base_gold: 130.0 },
        AreaCandidate { id: "pewter-gym", min_level: 32, max_level: 55, base_exp: 680.0, base_gold: 390.0 },
        AreaCandidate { id: "cerulean-cave", min_level: 55, max_level: 85, base_exp: 1750.0, base_gold: 1100.0 },
        AreaCandidate { id: "vermilion-port", min_level: 85, max_level: 125, base_exp: 4200.0, base_gold: 2600.0 },
        AreaCandidate { id: "lavender-tower", min_level: 125, max_level: 175, base_exp: 9800.0, base_gold: 6200.0 },
        AreaCandidate { id: "safari-zone", min_level: 175, max_level: 240, base_exp: 24000.0, base_gold: 15500.0 },
        AreaCandidate { id: "indigo-plateau", min_level: 240, max_level: 350, base_exp: 58000.0, base_gold: 38000.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static AreaCandidate {
        for area in HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &HUNTS_CATALOG[HUNTS_CATALOG.len() - 1]
    }
}
