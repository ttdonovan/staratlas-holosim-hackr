use anyhow::Result;
use ui_holosim::{PlanetUI, ShipUI, SectorUI, StarbaseUI};

use crate::data::GameData;

const PAGE_SIZE: usize = 20;

#[derive(Debug)]
pub enum CurrentScreen {
    Main,
    Ships,
    Planets,
    Sectors,
    Starbases,
}

pub struct App {
    pub game_data: GameData,
    pub current_screen: CurrentScreen,
    pub selected_index: usize,
    pub page_offset: usize,
}

impl App {
    pub fn new(game_data_path: &str) -> Result<Self> {
        let game_data = GameData::load_from_file(game_data_path)?;
        
        Ok(Self {
            game_data,
            current_screen: CurrentScreen::Main,
            selected_index: 0,
            page_offset: 0,
        })
    }

    pub fn next(&mut self) {
        let max_items = match self.current_screen {
            CurrentScreen::Ships => self.game_data.ships.len(),
            CurrentScreen::Planets => self.game_data.planets.len(),
            CurrentScreen::Sectors => self.game_data.sectors.len(),
            CurrentScreen::Starbases => self.game_data.starbases.len(),
            _ => 0,
        };

        if self.selected_index < max_items.saturating_sub(1) {
            self.selected_index += 1;
            
            // Auto-scroll to next page if needed
            let current_page_end = (self.page_offset + 1) * PAGE_SIZE;
            if self.selected_index >= current_page_end {
                self.page_offset += 1;
            }
        }
    }

    pub fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            
            // Auto-scroll to previous page if needed
            let current_page_start = self.page_offset * PAGE_SIZE;
            if self.selected_index < current_page_start {
                self.page_offset = self.page_offset.saturating_sub(1);
            }
        }
    }

    pub fn page_up(&mut self) {
        if self.page_offset > 0 {
            self.page_offset -= 1;
            self.selected_index = self.page_offset * PAGE_SIZE;
        }
    }

    pub fn page_down(&mut self) {
        let max_pages = self.total_pages();
        if self.page_offset < max_pages.saturating_sub(1) {
            self.page_offset += 1;
            self.selected_index = self.page_offset * PAGE_SIZE;
        }
    }

    pub fn current_page(&self) -> usize {
        self.page_offset
    }

    pub fn total_pages(&self) -> usize {
        let total_items = match self.current_screen {
            CurrentScreen::Ships => self.game_data.ships.len(),
            CurrentScreen::Planets => self.game_data.planets.len(),
            CurrentScreen::Sectors => self.game_data.sectors.len(),
            CurrentScreen::Starbases => self.game_data.starbases.len(),
            _ => 0,
        };
        
        (total_items + PAGE_SIZE - 1) / PAGE_SIZE
    }

    pub fn get_current_ships(&self) -> &[ShipUI] {
        let start = self.page_offset * PAGE_SIZE;
        let end = ((self.page_offset + 1) * PAGE_SIZE).min(self.game_data.ships.len());
        &self.game_data.ships[start..end]
    }

    pub fn get_current_planets(&self) -> &[PlanetUI] {
        let start = self.page_offset * PAGE_SIZE;
        let end = ((self.page_offset + 1) * PAGE_SIZE).min(self.game_data.planets.len());
        &self.game_data.planets[start..end]
    }

    pub fn get_current_sectors(&self) -> &[SectorUI] {
        let start = self.page_offset * PAGE_SIZE;
        let end = ((self.page_offset + 1) * PAGE_SIZE).min(self.game_data.sectors.len());
        &self.game_data.sectors[start..end]
    }

    pub fn get_current_starbases(&self) -> &[StarbaseUI] {
        let start = self.page_offset * PAGE_SIZE;
        let end = ((self.page_offset + 1) * PAGE_SIZE).min(self.game_data.starbases.len());
        &self.game_data.starbases[start..end]
    }
}