//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
#[derive(Debug)]
pub struct AddShipToFleet {
    /// The key on the profile.
    pub key: solana_program::pubkey::Pubkey,
    /// The profile that owns the fleet.
    pub owning_profile: solana_program::pubkey::Pubkey,
    /// The faction that the profile belongs to.
    pub owning_profile_faction: solana_program::pubkey::Pubkey,
    /// The fleet.
    pub fleet: solana_program::pubkey::Pubkey,
    /// The [`Game`] account
    pub game_id: solana_program::pubkey::Pubkey,
    /// The [`GameState`] account
    pub game_state: solana_program::pubkey::Pubkey,
    /// The funder for the new `Fleet`
    pub funder: solana_program::pubkey::Pubkey,
    /// The [`FleetShips`] account
    pub fleet_ships: solana_program::pubkey::Pubkey,
    /// The [`Ship`] Account
    pub ship: solana_program::pubkey::Pubkey,
    /// The [`Starbase`] account
    pub starbase: solana_program::pubkey::Pubkey,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: solana_program::pubkey::Pubkey,
    /// The Solana System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl AddShipToFleet {
    pub fn instruction(
        &self,
        args: AddShipToFleetInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: AddShipToFleetInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.key, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owning_profile,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.owning_profile_faction,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fleet, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.game_id,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.game_state,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fleet_ships,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.ship, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.starbase,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.starbase_player,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&AddShipToFleetInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&args).unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddShipToFleetInstructionData {
    discriminator: [u8; 8],
}

impl AddShipToFleetInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [237, 166, 153, 72, 122, 179, 220, 78],
        }
    }
}

impl Default for AddShipToFleetInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddShipToFleetInstructionArgs {
    pub ship_amount: u8,
    pub ship_escrow_index: u32,
    pub fleet_ship_info_index: Option<u32>,
    pub key_index: u16,
}

/// Instruction builder for `AddShipToFleet`.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` owning_profile
///   2. `[]` owning_profile_faction
///   3. `[writable]` fleet
///   4. `[]` game_id
///   5. `[]` game_state
///   6. `[writable, signer]` funder
///   7. `[writable]` fleet_ships
///   8. `[]` ship
///   9. `[]` starbase
///   10. `[writable]` starbase_player
///   11. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct AddShipToFleetBuilder {
    key: Option<solana_program::pubkey::Pubkey>,
    owning_profile: Option<solana_program::pubkey::Pubkey>,
    owning_profile_faction: Option<solana_program::pubkey::Pubkey>,
    fleet: Option<solana_program::pubkey::Pubkey>,
    game_id: Option<solana_program::pubkey::Pubkey>,
    game_state: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    fleet_ships: Option<solana_program::pubkey::Pubkey>,
    ship: Option<solana_program::pubkey::Pubkey>,
    starbase: Option<solana_program::pubkey::Pubkey>,
    starbase_player: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    ship_amount: Option<u8>,
    ship_escrow_index: Option<u32>,
    fleet_ship_info_index: Option<u32>,
    key_index: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl AddShipToFleetBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The key on the profile.
    #[inline(always)]
    pub fn key(&mut self, key: solana_program::pubkey::Pubkey) -> &mut Self {
        self.key = Some(key);
        self
    }
    /// The profile that owns the fleet.
    #[inline(always)]
    pub fn owning_profile(&mut self, owning_profile: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owning_profile = Some(owning_profile);
        self
    }
    /// The faction that the profile belongs to.
    #[inline(always)]
    pub fn owning_profile_faction(
        &mut self,
        owning_profile_faction: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.owning_profile_faction = Some(owning_profile_faction);
        self
    }
    /// The fleet.
    #[inline(always)]
    pub fn fleet(&mut self, fleet: solana_program::pubkey::Pubkey) -> &mut Self {
        self.fleet = Some(fleet);
        self
    }
    /// The [`Game`] account
    #[inline(always)]
    pub fn game_id(&mut self, game_id: solana_program::pubkey::Pubkey) -> &mut Self {
        self.game_id = Some(game_id);
        self
    }
    /// The [`GameState`] account
    #[inline(always)]
    pub fn game_state(&mut self, game_state: solana_program::pubkey::Pubkey) -> &mut Self {
        self.game_state = Some(game_state);
        self
    }
    /// The funder for the new `Fleet`
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// The [`FleetShips`] account
    #[inline(always)]
    pub fn fleet_ships(&mut self, fleet_ships: solana_program::pubkey::Pubkey) -> &mut Self {
        self.fleet_ships = Some(fleet_ships);
        self
    }
    /// The [`Ship`] Account
    #[inline(always)]
    pub fn ship(&mut self, ship: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ship = Some(ship);
        self
    }
    /// The [`Starbase`] account
    #[inline(always)]
    pub fn starbase(&mut self, starbase: solana_program::pubkey::Pubkey) -> &mut Self {
        self.starbase = Some(starbase);
        self
    }
    /// The [`StarbasePlayer`] Account
    #[inline(always)]
    pub fn starbase_player(
        &mut self,
        starbase_player: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.starbase_player = Some(starbase_player);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The Solana System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn ship_amount(&mut self, ship_amount: u8) -> &mut Self {
        self.ship_amount = Some(ship_amount);
        self
    }
    #[inline(always)]
    pub fn ship_escrow_index(&mut self, ship_escrow_index: u32) -> &mut Self {
        self.ship_escrow_index = Some(ship_escrow_index);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn fleet_ship_info_index(&mut self, fleet_ship_info_index: u32) -> &mut Self {
        self.fleet_ship_info_index = Some(fleet_ship_info_index);
        self
    }
    #[inline(always)]
    pub fn key_index(&mut self, key_index: u16) -> &mut Self {
        self.key_index = Some(key_index);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = AddShipToFleet {
            key: self.key.expect("key is not set"),
            owning_profile: self.owning_profile.expect("owning_profile is not set"),
            owning_profile_faction: self
                .owning_profile_faction
                .expect("owning_profile_faction is not set"),
            fleet: self.fleet.expect("fleet is not set"),
            game_id: self.game_id.expect("game_id is not set"),
            game_state: self.game_state.expect("game_state is not set"),
            funder: self.funder.expect("funder is not set"),
            fleet_ships: self.fleet_ships.expect("fleet_ships is not set"),
            ship: self.ship.expect("ship is not set"),
            starbase: self.starbase.expect("starbase is not set"),
            starbase_player: self.starbase_player.expect("starbase_player is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = AddShipToFleetInstructionArgs {
            ship_amount: self.ship_amount.clone().expect("ship_amount is not set"),
            ship_escrow_index: self
                .ship_escrow_index
                .clone()
                .expect("ship_escrow_index is not set"),
            fleet_ship_info_index: self.fleet_ship_info_index.clone(),
            key_index: self.key_index.clone().expect("key_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `add_ship_to_fleet` CPI accounts.
pub struct AddShipToFleetCpiAccounts<'a, 'b> {
    /// The key on the profile.
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The profile that owns the fleet.
    pub owning_profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The faction that the profile belongs to.
    pub owning_profile_faction: &'b solana_program::account_info::AccountInfo<'a>,
    /// The fleet.
    pub fleet: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`GameState`] account
    pub game_state: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new `Fleet`
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`FleetShips`] account
    pub fleet_ships: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Ship`] Account
    pub ship: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Starbase`] account
    pub starbase: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `add_ship_to_fleet` CPI instruction.
pub struct AddShipToFleetCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The key on the profile.
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The profile that owns the fleet.
    pub owning_profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The faction that the profile belongs to.
    pub owning_profile_faction: &'b solana_program::account_info::AccountInfo<'a>,
    /// The fleet.
    pub fleet: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`GameState`] account
    pub game_state: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new `Fleet`
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`FleetShips`] account
    pub fleet_ships: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Ship`] Account
    pub ship: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Starbase`] account
    pub starbase: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: AddShipToFleetInstructionArgs,
}

impl<'a, 'b> AddShipToFleetCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: AddShipToFleetCpiAccounts<'a, 'b>,
        args: AddShipToFleetInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            key: accounts.key,
            owning_profile: accounts.owning_profile,
            owning_profile_faction: accounts.owning_profile_faction,
            fleet: accounts.fleet,
            game_id: accounts.game_id,
            game_state: accounts.game_state,
            funder: accounts.funder,
            fleet_ships: accounts.fleet_ships,
            ship: accounts.ship,
            starbase: accounts.starbase,
            starbase_player: accounts.starbase_player,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(12 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.key.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owning_profile.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.owning_profile_faction.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fleet.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.game_id.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.game_state.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fleet_ships.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ship.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.starbase.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.starbase_player.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = borsh::to_vec(&AddShipToFleetInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(13 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.key.clone());
        account_infos.push(self.owning_profile.clone());
        account_infos.push(self.owning_profile_faction.clone());
        account_infos.push(self.fleet.clone());
        account_infos.push(self.game_id.clone());
        account_infos.push(self.game_state.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.fleet_ships.clone());
        account_infos.push(self.ship.clone());
        account_infos.push(self.starbase.clone());
        account_infos.push(self.starbase_player.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `AddShipToFleet` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` owning_profile
///   2. `[]` owning_profile_faction
///   3. `[writable]` fleet
///   4. `[]` game_id
///   5. `[]` game_state
///   6. `[writable, signer]` funder
///   7. `[writable]` fleet_ships
///   8. `[]` ship
///   9. `[]` starbase
///   10. `[writable]` starbase_player
///   11. `[]` system_program
#[derive(Clone, Debug)]
pub struct AddShipToFleetCpiBuilder<'a, 'b> {
    instruction: Box<AddShipToFleetCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> AddShipToFleetCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(AddShipToFleetCpiBuilderInstruction {
            __program: program,
            key: None,
            owning_profile: None,
            owning_profile_faction: None,
            fleet: None,
            game_id: None,
            game_state: None,
            funder: None,
            fleet_ships: None,
            ship: None,
            starbase: None,
            starbase_player: None,
            system_program: None,
            ship_amount: None,
            ship_escrow_index: None,
            fleet_ship_info_index: None,
            key_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The key on the profile.
    #[inline(always)]
    pub fn key(&mut self, key: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.key = Some(key);
        self
    }
    /// The profile that owns the fleet.
    #[inline(always)]
    pub fn owning_profile(
        &mut self,
        owning_profile: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.owning_profile = Some(owning_profile);
        self
    }
    /// The faction that the profile belongs to.
    #[inline(always)]
    pub fn owning_profile_faction(
        &mut self,
        owning_profile_faction: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.owning_profile_faction = Some(owning_profile_faction);
        self
    }
    /// The fleet.
    #[inline(always)]
    pub fn fleet(&mut self, fleet: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.fleet = Some(fleet);
        self
    }
    /// The [`Game`] account
    #[inline(always)]
    pub fn game_id(
        &mut self,
        game_id: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.game_id = Some(game_id);
        self
    }
    /// The [`GameState`] account
    #[inline(always)]
    pub fn game_state(
        &mut self,
        game_state: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.game_state = Some(game_state);
        self
    }
    /// The funder for the new `Fleet`
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
        self
    }
    /// The [`FleetShips`] account
    #[inline(always)]
    pub fn fleet_ships(
        &mut self,
        fleet_ships: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fleet_ships = Some(fleet_ships);
        self
    }
    /// The [`Ship`] Account
    #[inline(always)]
    pub fn ship(&mut self, ship: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.ship = Some(ship);
        self
    }
    /// The [`Starbase`] account
    #[inline(always)]
    pub fn starbase(
        &mut self,
        starbase: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.starbase = Some(starbase);
        self
    }
    /// The [`StarbasePlayer`] Account
    #[inline(always)]
    pub fn starbase_player(
        &mut self,
        starbase_player: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.starbase_player = Some(starbase_player);
        self
    }
    /// The Solana System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn ship_amount(&mut self, ship_amount: u8) -> &mut Self {
        self.instruction.ship_amount = Some(ship_amount);
        self
    }
    #[inline(always)]
    pub fn ship_escrow_index(&mut self, ship_escrow_index: u32) -> &mut Self {
        self.instruction.ship_escrow_index = Some(ship_escrow_index);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn fleet_ship_info_index(&mut self, fleet_ship_info_index: u32) -> &mut Self {
        self.instruction.fleet_ship_info_index = Some(fleet_ship_info_index);
        self
    }
    #[inline(always)]
    pub fn key_index(&mut self, key_index: u16) -> &mut Self {
        self.instruction.key_index = Some(key_index);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = AddShipToFleetInstructionArgs {
            ship_amount: self
                .instruction
                .ship_amount
                .clone()
                .expect("ship_amount is not set"),
            ship_escrow_index: self
                .instruction
                .ship_escrow_index
                .clone()
                .expect("ship_escrow_index is not set"),
            fleet_ship_info_index: self.instruction.fleet_ship_info_index.clone(),
            key_index: self
                .instruction
                .key_index
                .clone()
                .expect("key_index is not set"),
        };
        let instruction = AddShipToFleetCpi {
            __program: self.instruction.__program,

            key: self.instruction.key.expect("key is not set"),

            owning_profile: self
                .instruction
                .owning_profile
                .expect("owning_profile is not set"),

            owning_profile_faction: self
                .instruction
                .owning_profile_faction
                .expect("owning_profile_faction is not set"),

            fleet: self.instruction.fleet.expect("fleet is not set"),

            game_id: self.instruction.game_id.expect("game_id is not set"),

            game_state: self.instruction.game_state.expect("game_state is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            fleet_ships: self
                .instruction
                .fleet_ships
                .expect("fleet_ships is not set"),

            ship: self.instruction.ship.expect("ship is not set"),

            starbase: self.instruction.starbase.expect("starbase is not set"),

            starbase_player: self
                .instruction
                .starbase_player
                .expect("starbase_player is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct AddShipToFleetCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    key: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owning_profile: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    owning_profile_faction: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fleet: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    game_id: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    game_state: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fleet_ships: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ship: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    starbase: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    starbase_player: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ship_amount: Option<u8>,
    ship_escrow_index: Option<u32>,
    fleet_ship_info_index: Option<u32>,
    key_index: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
