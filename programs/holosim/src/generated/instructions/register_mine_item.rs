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
pub struct RegisterMineItem {
    /// The key authorized for this instruction
    pub key: solana_program::pubkey::Pubkey,
    /// The [`Profile`] account
    pub profile: solana_program::pubkey::Pubkey,
    /// The [`Game`] account
    pub game_id: solana_program::pubkey::Pubkey,
    /// The funder for the new mine item
    pub funder: solana_program::pubkey::Pubkey,
    /// The [`MineItem`] account
    pub mine_item: solana_program::pubkey::Pubkey,
    /// The mint address representing the mine item
    pub mint: solana_program::pubkey::Pubkey,
    /// The Solana System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl RegisterMineItem {
    pub fn instruction(
        &self,
        args: RegisterMineItemInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: RegisterMineItemInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
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
            self.mine_item,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = borsh::to_vec(&RegisterMineItemInstructionData::new()).unwrap();
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
pub struct RegisterMineItemInstructionData {
    discriminator: [u8; 8],
}

impl RegisterMineItemInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [54, 173, 58, 74, 128, 116, 109, 20],
        }
    }
}

impl Default for RegisterMineItemInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegisterMineItemInstructionArgs {
    pub name: [u8; 64],
    pub resource_hardness: u16,
    pub key_index: u16,
}

/// Instruction builder for `RegisterMineItem`.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` profile
///   2. `[]` game_id
///   3. `[writable, signer]` funder
///   4. `[writable]` mine_item
///   5. `[writable]` mint
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct RegisterMineItemBuilder {
    key: Option<solana_program::pubkey::Pubkey>,
    profile: Option<solana_program::pubkey::Pubkey>,
    game_id: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    mine_item: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<[u8; 64]>,
    resource_hardness: Option<u16>,
    key_index: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl RegisterMineItemBuilder {
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
    /// The funder for the new mine item
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// The [`MineItem`] account
    #[inline(always)]
    pub fn mine_item(&mut self, mine_item: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mine_item = Some(mine_item);
        self
    }
    /// The mint address representing the mine item
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
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
    pub fn name(&mut self, name: [u8; 64]) -> &mut Self {
        self.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn resource_hardness(&mut self, resource_hardness: u16) -> &mut Self {
        self.resource_hardness = Some(resource_hardness);
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
        let accounts = RegisterMineItem {
            key: self.key.expect("key is not set"),
            profile: self.profile.expect("profile is not set"),
            game_id: self.game_id.expect("game_id is not set"),
            funder: self.funder.expect("funder is not set"),
            mine_item: self.mine_item.expect("mine_item is not set"),
            mint: self.mint.expect("mint is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = RegisterMineItemInstructionArgs {
            name: self.name.clone().expect("name is not set"),
            resource_hardness: self
                .resource_hardness
                .clone()
                .expect("resource_hardness is not set"),
            key_index: self.key_index.clone().expect("key_index is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `register_mine_item` CPI accounts.
pub struct RegisterMineItemCpiAccounts<'a, 'b> {
    /// The key authorized for this instruction
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Profile`] account
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new mine item
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`MineItem`] account
    pub mine_item: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint address representing the mine item
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `register_mine_item` CPI instruction.
pub struct RegisterMineItemCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The key authorized for this instruction
    pub key: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Profile`] account
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new mine item
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`MineItem`] account
    pub mine_item: &'b solana_program::account_info::AccountInfo<'a>,
    /// The mint address representing the mine item
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// The Solana System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: RegisterMineItemInstructionArgs,
}

impl<'a, 'b> RegisterMineItemCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: RegisterMineItemCpiAccounts<'a, 'b>,
        args: RegisterMineItemInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            key: accounts.key,
            profile: accounts.profile,
            game_id: accounts.game_id,
            funder: accounts.funder,
            mine_item: accounts.mine_item,
            mint: accounts.mint,
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
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
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
            *self.mine_item.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.key,
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
        let mut data = borsh::to_vec(&RegisterMineItemInstructionData::new()).unwrap();
        let mut args = borsh::to_vec(&self.__args).unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.key.clone());
        account_infos.push(self.profile.clone());
        account_infos.push(self.game_id.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.mine_item.clone());
        account_infos.push(self.mint.clone());
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

/// Instruction builder for `RegisterMineItem` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` key
///   1. `[]` profile
///   2. `[]` game_id
///   3. `[writable, signer]` funder
///   4. `[writable]` mine_item
///   5. `[writable]` mint
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct RegisterMineItemCpiBuilder<'a, 'b> {
    instruction: Box<RegisterMineItemCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> RegisterMineItemCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(RegisterMineItemCpiBuilderInstruction {
            __program: program,
            key: None,
            profile: None,
            game_id: None,
            funder: None,
            mine_item: None,
            mint: None,
            system_program: None,
            name: None,
            resource_hardness: None,
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
    /// The funder for the new mine item
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
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
    /// The mint address representing the mine item
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
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
    pub fn name(&mut self, name: [u8; 64]) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    #[inline(always)]
    pub fn resource_hardness(&mut self, resource_hardness: u16) -> &mut Self {
        self.instruction.resource_hardness = Some(resource_hardness);
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
        let args = RegisterMineItemInstructionArgs {
            name: self.instruction.name.clone().expect("name is not set"),
            resource_hardness: self
                .instruction
                .resource_hardness
                .clone()
                .expect("resource_hardness is not set"),
            key_index: self
                .instruction
                .key_index
                .clone()
                .expect("key_index is not set"),
        };
        let instruction = RegisterMineItemCpi {
            __program: self.instruction.__program,

            key: self.instruction.key.expect("key is not set"),

            profile: self.instruction.profile.expect("profile is not set"),

            game_id: self.instruction.game_id.expect("game_id is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            mine_item: self.instruction.mine_item.expect("mine_item is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

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
struct RegisterMineItemCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    key: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    profile: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    game_id: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mine_item: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<[u8; 64]>,
    resource_hardness: Option<u16>,
    key_index: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
