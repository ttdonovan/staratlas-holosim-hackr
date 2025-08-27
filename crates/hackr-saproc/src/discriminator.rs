use std::collections::HashMap;

/// Common 8-byte discriminators for Star Atlas account types
/// These are typically the first 8 bytes of the account data
pub struct Discriminators {
    discriminators: HashMap<[u8; 8], &'static str>,
}

impl Discriminators {
    pub fn new() -> Self {
        let mut discriminators = HashMap::new();

        // Holosim discriminators (from codamaIDL.json)
        discriminators.insert(
            [0xf5, 0xd3, 0x48, 0x3f, 0x2c, 0x82, 0x76, 0xc1],
            "CombatConfig",
        );
        discriminators.insert(
            [0x5a, 0xba, 0x9b, 0xd0, 0x5d, 0xba, 0x70, 0xbf],
            "CraftingInstance",
        );
        discriminators.insert(
            [0x35, 0x06, 0x7f, 0x17, 0xf7, 0x0c, 0xe1, 0xf9],
            "DisbandedFleet",
        );
        discriminators.insert([0x6d, 0xcf, 0xfb, 0x30, 0x6a, 0x02, 0x88, 0xa3], "Fleet");
        discriminators.insert(
            [0xfc, 0x51, 0x93, 0xf6, 0xde, 0x8d, 0xb9, 0x6e],
            "FleetShips",
        );
        discriminators.insert([0x1b, 0x5a, 0xa6, 0x7d, 0x4a, 0x64, 0x79, 0x12], "Game");
        discriminators.insert(
            [0x90, 0x5e, 0xd0, 0xac, 0xf8, 0x63, 0x86, 0x78],
            "GameState",
        );
        discriminators.insert([0x97, 0xe1, 0xcf, 0xe4, 0x73, 0xd2, 0x40, 0x9f], "Loot");
        discriminators.insert([0x40, 0x37, 0xd4, 0x13, 0xd7, 0x9c, 0x16, 0x42], "MineItem");
        discriminators.insert([0xf2, 0x1b, 0xec, 0x2a, 0xdc, 0xd9, 0x84, 0x80], "Planet");
        discriminators.insert(
            [0xdd, 0xb9, 0x30, 0x07, 0x4b, 0xc4, 0x26, 0xdb],
            "PlayerCrewRecord",
        );
        discriminators.insert(
            [0xe0, 0x9c, 0x81, 0x5f, 0x0f, 0x1d, 0x84, 0xd0],
            "ProgressionConfig",
        );
        discriminators.insert([0x0a, 0xa0, 0x02, 0x01, 0x2a, 0xcf, 0x33, 0xd4], "Resource");
        discriminators.insert(
            [0xf7, 0x2a, 0x18, 0x6d, 0xa1, 0x03, 0xf8, 0xa6],
            "SageCrewConfig",
        );
        discriminators.insert(
            [0x0a, 0x37, 0x4b, 0xea, 0x7e, 0x0e, 0x2f, 0x92],
            "SagePlayerProfile",
        );
        discriminators.insert([0x41, 0x75, 0x17, 0x52, 0x50, 0x85, 0xf7, 0xe9], "Sector");
        discriminators.insert([0x72, 0x29, 0xf5, 0xe8, 0x18, 0x3a, 0xea, 0x9e], "Ship");
        discriminators.insert([0xd6, 0x83, 0xcf, 0xd0, 0xca, 0x94, 0xa2, 0x30], "Star");
        discriminators.insert([0xcc, 0xb6, 0x1d, 0xe7, 0xdc, 0x1d, 0x34, 0x02], "Starbase");
        discriminators.insert(
            [0xc0, 0xea, 0x90, 0x56, 0x48, 0x13, 0x05, 0x63],
            "StarbasePlayer",
        );
        discriminators.insert(
            [0xea, 0x7f, 0xe3, 0x5a, 0x90, 0x41, 0x55, 0x6f],
            "SurveyDataUnitTracker",
        );

        // Player Profile discriminators (from codamaIDL.json)
        discriminators.insert(
            [0xc5, 0xd8, 0x63, 0xec, 0xc9, 0xf5, 0xd3, 0x81],
            "PlayerName",
        );
        discriminators.insert([0x05, 0x76, 0xa8, 0x9d, 0xcd, 0xef, 0x24, 0xf0], "Profile");
        discriminators.insert(
            [0x4e, 0xf9, 0x48, 0xb8, 0xc6, 0xb2, 0x04, 0x87],
            "ProfileRoleMembership",
        );
        discriminators.insert([0x7f, 0x4c, 0x02, 0xba, 0x82, 0x38, 0xbe, 0xb8], "Role");

        // Profile Faction discriminators (from codamaIDL.json)
        discriminators.insert(
            [0x53, 0xac, 0x49, 0xaa, 0xbf, 0xbf, 0xcc, 0x51],
            "ProfileFactionAccount",
        );

        Self { discriminators }
    }

    pub fn identify_account_type(&self, data: &[u8]) -> Option<&'static str> {
        if data.len() < 8 {
            return None;
        }

        let mut discriminator = [0u8; 8];
        discriminator.copy_from_slice(&data[0..8]);

        self.discriminators.get(&discriminator).copied()
    }

    pub fn get_discriminator_bytes(&self, account_type: &str) -> Option<[u8; 8]> {
        for (bytes, name) in &self.discriminators {
            if *name == account_type {
                return Some(*bytes);
            }
        }
        None
    }

    pub fn list_known_types(&self) -> Vec<&'static str> {
        self.discriminators.values().copied().collect()
    }
}

impl Default for Discriminators {
    fn default() -> Self {
        Self::new()
    }
}
