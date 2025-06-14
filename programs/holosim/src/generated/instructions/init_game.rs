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
pub struct InitGame {
    /// The entity calling this instruction
    pub signer: solana_program::pubkey::Pubkey,
    /// The sector permissions [`Profile`]
    pub profile: solana_program::pubkey::Pubkey,
    /// The funder for the new game
    pub funder: solana_program::pubkey::Pubkey,
    /// The [`Game`] account
    pub game_id: solana_program::pubkey::Pubkey,
    /// The system program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl InitGame {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.profile,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.funder,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.game_id,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&InitGameInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitGameInstructionData {
    discriminator: [u8; 8],
}

impl InitGameInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [251, 46, 12, 208, 184, 148, 157, 73],
        }
    }
}

impl Default for InitGameInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `InitGame`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` profile
///   2. `[writable, signer]` funder
///   3. `[writable, signer]` game_id
///   4. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct InitGameBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    profile: Option<solana_program::pubkey::Pubkey>,
    funder: Option<solana_program::pubkey::Pubkey>,
    game_id: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitGameBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The entity calling this instruction
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    /// The sector permissions [`Profile`]
    #[inline(always)]
    pub fn profile(&mut self, profile: solana_program::pubkey::Pubkey) -> &mut Self {
        self.profile = Some(profile);
        self
    }
    /// The funder for the new game
    #[inline(always)]
    pub fn funder(&mut self, funder: solana_program::pubkey::Pubkey) -> &mut Self {
        self.funder = Some(funder);
        self
    }
    /// The [`Game`] account
    #[inline(always)]
    pub fn game_id(&mut self, game_id: solana_program::pubkey::Pubkey) -> &mut Self {
        self.game_id = Some(game_id);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// The system program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
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
        let accounts = InitGame {
            signer: self.signer.expect("signer is not set"),
            profile: self.profile.expect("profile is not set"),
            funder: self.funder.expect("funder is not set"),
            game_id: self.game_id.expect("game_id is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `init_game` CPI accounts.
pub struct InitGameCpiAccounts<'a, 'b> {
    /// The entity calling this instruction
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The sector permissions [`Profile`]
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new game
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `init_game` CPI instruction.
pub struct InitGameCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The entity calling this instruction
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,
    /// The sector permissions [`Profile`]
    pub profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The funder for the new game
    pub funder: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Game`] account
    pub game_id: &'b solana_program::account_info::AccountInfo<'a>,
    /// The system program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> InitGameCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitGameCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            profile: accounts.profile,
            funder: accounts.funder,
            game_id: accounts.game_id,
            system_program: accounts.system_program,
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
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.profile.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.funder.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.game_id.key,
            true,
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
        let data = borsh::to_vec(&InitGameInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.profile.clone());
        account_infos.push(self.funder.clone());
        account_infos.push(self.game_id.clone());
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

/// Instruction builder for `InitGame` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[]` profile
///   2. `[writable, signer]` funder
///   3. `[writable, signer]` game_id
///   4. `[]` system_program
#[derive(Clone, Debug)]
pub struct InitGameCpiBuilder<'a, 'b> {
    instruction: Box<InitGameCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitGameCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitGameCpiBuilderInstruction {
            __program: program,
            signer: None,
            profile: None,
            funder: None,
            game_id: None,
            system_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The entity calling this instruction
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    /// The sector permissions [`Profile`]
    #[inline(always)]
    pub fn profile(
        &mut self,
        profile: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.profile = Some(profile);
        self
    }
    /// The funder for the new game
    #[inline(always)]
    pub fn funder(
        &mut self,
        funder: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.funder = Some(funder);
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
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
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
        let instruction = InitGameCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            profile: self.instruction.profile.expect("profile is not set"),

            funder: self.instruction.funder.expect("funder is not set"),

            game_id: self.instruction.game_id.expect("game_id is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitGameCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    profile: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    funder: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    game_id: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
