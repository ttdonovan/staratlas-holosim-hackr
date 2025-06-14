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
pub struct MintCrewToGame {
    /// The [`SagePlayerProfile`] account
    pub sage_player_profile: solana_program::pubkey::Pubkey,
    /// The [`Starbase`] account
    pub starbase: solana_program::pubkey::Pubkey,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: solana_program::pubkey::Pubkey,
    /// The [`SageCrewConfig`] Account
    pub sage_crew_config: solana_program::pubkey::Pubkey,
    /// The crew program `CrewConfig` account
    pub crew_config: solana_program::pubkey::Pubkey,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: solana_program::pubkey::Pubkey,
}

impl MintCrewToGame {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::arithmetic_side_effects)]
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.sage_player_profile,
            false,
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
            self.sage_crew_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.crew_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.instructions_sysvar,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = borsh::to_vec(&MintCrewToGameInstructionData::new()).unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintCrewToGameInstructionData {
    discriminator: [u8; 8],
}

impl MintCrewToGameInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [64, 222, 94, 243, 149, 65, 54, 132],
        }
    }
}

impl Default for MintCrewToGameInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `MintCrewToGame`.
///
/// ### Accounts:
///
///   0. `[writable]` sage_player_profile
///   1. `[]` starbase
///   2. `[writable]` starbase_player
///   3. `[]` sage_crew_config
///   4. `[]` crew_config
///   5. `[optional]` instructions_sysvar (default to `Sysvar1nstructions1111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct MintCrewToGameBuilder {
    sage_player_profile: Option<solana_program::pubkey::Pubkey>,
    starbase: Option<solana_program::pubkey::Pubkey>,
    starbase_player: Option<solana_program::pubkey::Pubkey>,
    sage_crew_config: Option<solana_program::pubkey::Pubkey>,
    crew_config: Option<solana_program::pubkey::Pubkey>,
    instructions_sysvar: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl MintCrewToGameBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// The [`SagePlayerProfile`] account
    #[inline(always)]
    pub fn sage_player_profile(
        &mut self,
        sage_player_profile: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.sage_player_profile = Some(sage_player_profile);
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
    /// The [`SageCrewConfig`] Account
    #[inline(always)]
    pub fn sage_crew_config(
        &mut self,
        sage_crew_config: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.sage_crew_config = Some(sage_crew_config);
        self
    }
    /// The crew program `CrewConfig` account
    #[inline(always)]
    pub fn crew_config(&mut self, crew_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.crew_config = Some(crew_config);
        self
    }
    /// `[optional account, default to 'Sysvar1nstructions1111111111111111111111111']`
    /// Solana Instructions Sysvar
    #[inline(always)]
    pub fn instructions_sysvar(
        &mut self,
        instructions_sysvar: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.instructions_sysvar = Some(instructions_sysvar);
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
        let accounts = MintCrewToGame {
            sage_player_profile: self
                .sage_player_profile
                .expect("sage_player_profile is not set"),
            starbase: self.starbase.expect("starbase is not set"),
            starbase_player: self.starbase_player.expect("starbase_player is not set"),
            sage_crew_config: self.sage_crew_config.expect("sage_crew_config is not set"),
            crew_config: self.crew_config.expect("crew_config is not set"),
            instructions_sysvar: self.instructions_sysvar.unwrap_or(solana_program::pubkey!(
                "Sysvar1nstructions1111111111111111111111111"
            )),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `mint_crew_to_game` CPI accounts.
pub struct MintCrewToGameCpiAccounts<'a, 'b> {
    /// The [`SagePlayerProfile`] account
    pub sage_player_profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Starbase`] account
    pub starbase: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`SageCrewConfig`] Account
    pub sage_crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// The crew program `CrewConfig` account
    pub crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `mint_crew_to_game` CPI instruction.
pub struct MintCrewToGameCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`SagePlayerProfile`] account
    pub sage_player_profile: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`Starbase`] account
    pub starbase: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`StarbasePlayer`] Account
    pub starbase_player: &'b solana_program::account_info::AccountInfo<'a>,
    /// The [`SageCrewConfig`] Account
    pub sage_crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// The crew program `CrewConfig` account
    pub crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Solana Instructions Sysvar
    pub instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> MintCrewToGameCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: MintCrewToGameCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            sage_player_profile: accounts.sage_player_profile,
            starbase: accounts.starbase,
            starbase_player: accounts.starbase_player,
            sage_crew_config: accounts.sage_crew_config,
            crew_config: accounts.crew_config,
            instructions_sysvar: accounts.instructions_sysvar,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.sage_player_profile.key,
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
            *self.sage_crew_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.crew_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.instructions_sysvar.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = borsh::to_vec(&MintCrewToGameInstructionData::new()).unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::SAGE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.sage_player_profile.clone());
        account_infos.push(self.starbase.clone());
        account_infos.push(self.starbase_player.clone());
        account_infos.push(self.sage_crew_config.clone());
        account_infos.push(self.crew_config.clone());
        account_infos.push(self.instructions_sysvar.clone());
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

/// Instruction builder for `MintCrewToGame` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` sage_player_profile
///   1. `[]` starbase
///   2. `[writable]` starbase_player
///   3. `[]` sage_crew_config
///   4. `[]` crew_config
///   5. `[]` instructions_sysvar
#[derive(Clone, Debug)]
pub struct MintCrewToGameCpiBuilder<'a, 'b> {
    instruction: Box<MintCrewToGameCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> MintCrewToGameCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(MintCrewToGameCpiBuilderInstruction {
            __program: program,
            sage_player_profile: None,
            starbase: None,
            starbase_player: None,
            sage_crew_config: None,
            crew_config: None,
            instructions_sysvar: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// The [`SagePlayerProfile`] account
    #[inline(always)]
    pub fn sage_player_profile(
        &mut self,
        sage_player_profile: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sage_player_profile = Some(sage_player_profile);
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
    /// The [`SageCrewConfig`] Account
    #[inline(always)]
    pub fn sage_crew_config(
        &mut self,
        sage_crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sage_crew_config = Some(sage_crew_config);
        self
    }
    /// The crew program `CrewConfig` account
    #[inline(always)]
    pub fn crew_config(
        &mut self,
        crew_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.crew_config = Some(crew_config);
        self
    }
    /// Solana Instructions Sysvar
    #[inline(always)]
    pub fn instructions_sysvar(
        &mut self,
        instructions_sysvar: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.instructions_sysvar = Some(instructions_sysvar);
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
        let instruction = MintCrewToGameCpi {
            __program: self.instruction.__program,

            sage_player_profile: self
                .instruction
                .sage_player_profile
                .expect("sage_player_profile is not set"),

            starbase: self.instruction.starbase.expect("starbase is not set"),

            starbase_player: self
                .instruction
                .starbase_player
                .expect("starbase_player is not set"),

            sage_crew_config: self
                .instruction
                .sage_crew_config
                .expect("sage_crew_config is not set"),

            crew_config: self
                .instruction
                .crew_config
                .expect("crew_config is not set"),

            instructions_sysvar: self
                .instruction
                .instructions_sysvar
                .expect("instructions_sysvar is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct MintCrewToGameCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    sage_player_profile: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    starbase: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    starbase_player: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sage_crew_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    crew_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    instructions_sysvar: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
