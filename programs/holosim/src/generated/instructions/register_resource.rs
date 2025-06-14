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
pub struct RegisterResource {
    /// The key authorized for this instruction
    pub key: solana_program::pubkey::Pubkey,
    /// The [`Profile`] account
    pub profile: solana_program::pubkey::Pubkey,
    /// The [`Game`] account
    pub game_id: solana_program::pubkey::Pubkey,
    /// The funder for the new resource
    pub funder: solana_program::pubkey::Pubkey,
    /// The [`Resource`] account
    pub resource: solana_program::pubkey::Pubkey,
    /// The Location address
    pub location: solana_program::pubkey::Pubkey,
    /// The [`MineItem`] account
    pub mine_item: solana_program::pubkey::Pubkey,
    /// The Solana System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl RegisterResource {
    pub fn instruction(
        &self,
        args: RegisterResourceInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: RegisterResourceInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.key, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.profile,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.game_id,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.resource,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.location,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mine_item,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&RegisterResourceInstructionData::new()).unwrap();
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
pub struct RegisterResourceInstructionData {
    discriminator: [u8; 8],
}

impl RegisterResourceInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [87, 209, 164, 28, 6, 82, 232, 214],
        }
    }
}

impl Default for RegisterResourceInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegisterResourceInstructionArgs {
    pub location_type: u8,
    pub system_richness: u16,
    pub key_index: u16,
}

/// Instruction builder for `RegisterResource`.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` profile
///   2. `[]` game_id
///   3. `[writable, signer]` funder
///   4. `[writable]` resource
///   5. `[writable]` location
///   6. `[writable]` mine_item
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct RegisterResourceBuilder {
    key: Option<solana_program::pubkey::Pubkey>,
    profile: Option<solana_program::pubkey::Pubkey>,
    game_id: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    resource: Option<solana_program::pubkey::Pubkey>,
    location: Option<solana_program::pubkey::Pubkey>,
    mine_item: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    location_type: Option<u8>,
    system_richness: Option<u16>,
    key_index: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RegisterResourceBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The key authorized for this instruction
    #[inline(always)]
    pub fn key(&mut self, key: solana_program::pubkey::Pubkey) -> &mut Self {
        self.key = Some(key);
        self
    }
    /// The [`Profile`] account
    #[inline(always)]
    pub fn profile(&mut self, profile: solana_program::pubkey::Pubkey) -> &mut Self {
        self.profile = Some(profile);
        self
    }
    /// The [`Game`] account
    #[inline(always)]
    pub fn game_id(&mut self, game_id: solana_program::pubkey::Pubkey) -> &mut Self {
        self.game_id = Some(game_id);
        self
    }
    /// The funder for the new resource
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// The [`Resource`] account
    #[inline(always)]
    pub fn resource(&mut self, resource: solana_program::pubkey::Pubkey) -> &mut Self {
        self.resource = Some(resource);
        self
    }
    /// The Location address
    #[inline(always)]
    pub fn location(&mut self, location: solana_program::pubkey::Pubkey) -> &mut Self {
        self.location = Some(location);
        self
    }
    /// The [`MineItem`] account
    #[inline(always)]
    pub fn mine_item(&mut self, mine_item: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mine_item = Some(mine_item);
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
    pub fn location_type(&mut self, location_type: u8) -> &mut Self {
        self.location_type = Some(location_type);
        self
    }
    #[inline(always)]
    pub fn system_richness(&mut self, system_richness: u16) -> &mut Self {
        self.system_richness = Some(system_richness);
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
        let accounts = RegisterResource {
            key: self.key.expect("key is not set"),
            profile: self.profile.expect("profile is not set"),
            game_id: self.game_id.expect("game_id is not set"),
            funder: self.funder.expect("funder is not set"),
            resource: self.resource.expect("resource is not set"),
            location: self.location.expect("location is not set"),
            mine_item: self.mine_item.expect("mine_item is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = RegisterResourceInstructionArgs {
            location_type: self
                .location_type
                .clone()
                .expect("location_type is not set"),
            system_richness: self
                .system_richness
                .clone()
                .expect("system_richness is not set"),
            key_index: self.key_index.clone().expect("key_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `register_resource` CPI accounts.
pub struct RegisterResourceCpiAccounts<'a, 'b> {
    /// The key authorized for this instruction
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Profile`] account
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new resource
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Resource`] account
    pub resource: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Location address
    pub location: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`MineItem`] account
    pub mine_item: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `register_resource` CPI instruction.
pub struct RegisterResourceCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The key authorized for this instruction
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Profile`] account
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new resource
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Resource`] account
    pub resource: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Location address
    pub location: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`MineItem`] account
    pub mine_item: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: RegisterResourceInstructionArgs,
}

impl<'a, 'b> RegisterResourceCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RegisterResourceCpiAccounts<'a, 'b>,
        args: RegisterResourceInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            key: accounts.key,
            profile: accounts.profile,
            game_id: accounts.game_id,
            funder: accounts.funder,
            resource: accounts.resource,
            location: accounts.location,
            mine_item: accounts.mine_item,
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
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.key.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.profile.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.game_id.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.resource.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.location.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mine_item.key,
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
        let mut data = borsh::to_vec(&RegisterResourceInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.key.clone());
        account_infos.push(self.profile.clone());
        account_infos.push(self.game_id.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.resource.clone());
        account_infos.push(self.location.clone());
        account_infos.push(self.mine_item.clone());
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

/// Instruction builder for `RegisterResource` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` profile
///   2. `[]` game_id
///   3. `[writable, signer]` funder
///   4. `[writable]` resource
///   5. `[writable]` location
///   6. `[writable]` mine_item
///   7. `[]` system_program
#[derive(Clone, Debug)]
pub struct RegisterResourceCpiBuilder<'a, 'b> {
    instruction: Box<RegisterResourceCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RegisterResourceCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RegisterResourceCpiBuilderInstruction {
            __program: program,
            key: None,
            profile: None,
            game_id: None,
            funder: None,
            resource: None,
            location: None,
            mine_item: None,
            system_program: None,
            location_type: None,
            system_richness: None,
            key_index: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The key authorized for this instruction
    #[inline(always)]
    pub fn key(&mut self, key: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.key = Some(key);
        self
    }
    /// The [`Profile`] account
    #[inline(always)]
    pub fn profile(
        &mut self,
        profile: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.profile = Some(profile);
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
    /// The funder for the new resource
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
        self
    }
    /// The [`Resource`] account
    #[inline(always)]
    pub fn resource(
        &mut self,
        resource: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.resource = Some(resource);
        self
    }
    /// The Location address
    #[inline(always)]
    pub fn location(
        &mut self,
        location: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.location = Some(location);
        self
    }
    /// The [`MineItem`] account
    #[inline(always)]
    pub fn mine_item(
        &mut self,
        mine_item: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mine_item = Some(mine_item);
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
    pub fn location_type(&mut self, location_type: u8) -> &mut Self {
        self.instruction.location_type = Some(location_type);
        self
    }
    #[inline(always)]
    pub fn system_richness(&mut self, system_richness: u16) -> &mut Self {
        self.instruction.system_richness = Some(system_richness);
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
        let args = RegisterResourceInstructionArgs {
            location_type: self
                .instruction
                .location_type
                .clone()
                .expect("location_type is not set"),
            system_richness: self
                .instruction
                .system_richness
                .clone()
                .expect("system_richness is not set"),
            key_index: self
                .instruction
                .key_index
                .clone()
                .expect("key_index is not set"),
        };
        let instruction = RegisterResourceCpi {
            __program: self.instruction.__program,

            key: self.instruction.key.expect("key is not set"),

            profile: self.instruction.profile.expect("profile is not set"),

            game_id: self.instruction.game_id.expect("game_id is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            resource: self.instruction.resource.expect("resource is not set"),

            location: self.instruction.location.expect("location is not set"),

            mine_item: self.instruction.mine_item.expect("mine_item is not set"),

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
struct RegisterResourceCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    key: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    profile: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    game_id: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    resource: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    location: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mine_item: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    location_type: Option<u8>,
    system_richness: Option<u16>,
    key_index: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
