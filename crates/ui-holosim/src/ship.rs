use crate::UIConversionError;
use serde::{Deserialize, Serialize};
use staratlas_holosim::generated::accounts::Ship;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipUI {
    pub account_type: String,
    pub discriminator: String,
    pub version: u8,
    pub name: String,
    pub size_class: u8,
    pub stats: ShipStatsUI,
    pub mint: String,
    pub update_id: u64,
    pub max_update_id: u64,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipStatsUI {
    pub movement_stats: MovementStatsUI,
    pub cargo_stats: CargoStatsUI,
    pub misc_stats: MiscStatsUI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementStatsUI {
    pub subwarp_speed: u32,
    pub warp_speed: u32,
    pub max_warp_distance: u16,
    pub warp_cool_down: u16,
    pub subwarp_fuel_consumption_rate: u32,
    pub warp_fuel_consumption_rate: u32,
    pub planet_exit_fuel_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoStatsUI {
    pub cargo_capacity: u32,
    pub fuel_capacity: u32,
    pub ammo_capacity: u32,
    pub ammo_consumption_rate: u32,
    pub food_consumption_rate: u32,
    pub mining_rate: u32,
    pub upgrade_rate: u32,
    pub cargo_transfer_rate: u32,
    pub tractor_beam_gather_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiscStatsUI {
    pub required_crew: u16,
    pub passenger_capacity: u16,
    pub crew_count: u16,
    pub rented_crew: u16,
    pub respawn_time: u16,
    pub scan_cool_down: u16,
    pub sdu_per_scan: u32,
    pub scan_cost: u32,
}

impl TryFrom<&Ship> for ShipUI {
    type Error = UIConversionError;

    fn try_from(ship: &Ship) -> Result<Self, Self::Error> {
        Ok(ShipUI {
            account_type: "Ship".to_string(),
            discriminator: hex::encode(&ship.discriminator),
            version: ship.version,
            name: std::str::from_utf8(&ship.name)?
                .trim_matches('\0')
                .to_string(),
            size_class: ship.size_class,
            stats: ShipStatsUI {
                movement_stats: MovementStatsUI {
                    subwarp_speed: ship.stats.movement_stats.subwarp_speed,
                    warp_speed: ship.stats.movement_stats.warp_speed,
                    max_warp_distance: ship.stats.movement_stats.max_warp_distance,
                    warp_cool_down: ship.stats.movement_stats.warp_cool_down,
                    subwarp_fuel_consumption_rate: ship
                        .stats
                        .movement_stats
                        .subwarp_fuel_consumption_rate,
                    warp_fuel_consumption_rate: ship
                        .stats
                        .movement_stats
                        .warp_fuel_consumption_rate,
                    planet_exit_fuel_amount: ship.stats.movement_stats.planet_exit_fuel_amount,
                },
                cargo_stats: CargoStatsUI {
                    cargo_capacity: ship.stats.cargo_stats.cargo_capacity,
                    fuel_capacity: ship.stats.cargo_stats.fuel_capacity,
                    ammo_capacity: ship.stats.cargo_stats.ammo_capacity,
                    ammo_consumption_rate: ship.stats.cargo_stats.ammo_consumption_rate,
                    food_consumption_rate: ship.stats.cargo_stats.food_consumption_rate,
                    mining_rate: ship.stats.cargo_stats.mining_rate,
                    upgrade_rate: ship.stats.cargo_stats.upgrade_rate,
                    cargo_transfer_rate: ship.stats.cargo_stats.cargo_transfer_rate,
                    tractor_beam_gather_rate: ship.stats.cargo_stats.tractor_beam_gather_rate,
                },
                misc_stats: MiscStatsUI {
                    required_crew: ship.stats.misc_stats.required_crew,
                    passenger_capacity: ship.stats.misc_stats.passenger_capacity,
                    crew_count: ship.stats.misc_stats.crew_count,
                    rented_crew: ship.stats.misc_stats.rented_crew,
                    respawn_time: ship.stats.misc_stats.respawn_time,
                    scan_cool_down: ship.stats.misc_stats.scan_cool_down,
                    sdu_per_scan: ship.stats.misc_stats.sdu_per_scan,
                    scan_cost: ship.stats.misc_stats.scan_cost,
                },
            },
            mint: ship.mint.to_string(),
            update_id: ship.update_id,
            max_update_id: ship.max_update_id,
            next: if ship.next.key == solana_sdk::system_program::ID {
                None
            } else {
                Some(ship.next.key.to_string())
            },
        })
    }
}

impl ShipUI {
    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let ship = Ship::from_bytes(data)?;
        Ok(Self::try_from(&ship)?)
    }
}
