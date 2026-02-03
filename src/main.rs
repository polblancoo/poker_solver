use eframe::egui;
use poker::{cards, Card, Evaluator, Rank, Suit};
use std::collections::HashSet;
use std::str::FromStr;

// --- ESTRUCTURAS DE DATOS ---

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct MyCard {
    rank: Rank,
    suit: Suit,
}

impl MyCard {
    fn to_poker_card(&self) -> Card {
        // La crate 'poker' permite crear cartas directamente con Rank y Suit
        Card::new(self.rank, self.suit)
    }

    fn rank_char(&self) -> char {
        match self.rank {
            Rank::Two => '2', Rank::Three => '3', Rank::Four => '4', Rank::Five => '5',
            Rank::Six => '6', Rank::Seven => '7', Rank::Eight => '8', Rank::Nine => '9',
            Rank::Ten => 'T', Rank::Jack => 'J', Rank::Queen => 'Q', Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }

    fn suit_char(&self) -> char {
        match self.suit {
            Suit::Clubs => 'c',
            Suit::Diamonds => 'd',
            Suit::Hearts => 'h',
            Suit::Spades => 's',
        }
    }

    fn display_text(&self) -> String {
        let r = match self.rank {
            Rank::Ten => "10".to_string(),
            _ => self.rank_char().to_string(),
        };
        let s = match self.suit {
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        format!("{}{}", r, s)
    }

    fn color(&self) -> egui::Color32 {
        match self.suit {
            Suit::Hearts => egui::Color32::from_rgb(235, 60, 60),   // Rojo
            Suit::Diamonds => egui::Color32::from_rgb(60, 100, 235), // Azul
            Suit::Clubs => egui::Color32::from_rgb(60, 200, 60),    // Verde
            Suit::Spades => egui::Color32::from_rgb(180, 180, 180), // Gris Claro
        }
    }
}

// --- APP STATE ---

struct PokerApp {
    // Cartas en juego
    hero_hand: [Option<MyCard>; 2],
    villain_hand: [Option<MyCard>; 2],   // Villano (Opcional)
    friends_hands: [[Option<MyCard>; 2]; 3], // 3 amigos
    board: Vec<Option<MyCard>>,              // Hasta 5 cartas

    // Estado de selección
    selected_card_idx: Option<CardSlot>, // Qué slot estamos editando

    // Matriz
    excluded_cells: HashSet<(usize, usize)>, // Celdas desactivadas por el usuario (click)
    
    // Evaluador
    evaluator: Evaluator,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum CardSlot {
    Hero(usize),
    Villain(usize), // Nuevo
    Friend(usize, usize), // friend_idx, card_idx
    Board(usize),
}

impl Default for PokerApp {
    fn default() -> Self {
        Self {
            hero_hand: [None; 2],
            villain_hand: [None; 2],
            friends_hands: [[None; 2]; 3],
            board: vec![None; 5],
            selected_card_idx: None,
            excluded_cells: HashSet::new(),
            evaluator: Evaluator::new(),
        }
    }
}

// --- UTILIDADES ---

fn rank_to_str(r: Rank) -> &'static str {
    match r {
        Rank::Ace => "A", Rank::King => "K", Rank::Queen => "Q", Rank::Jack => "J",
        Rank::Ten => "T", Rank::Nine => "9", Rank::Eight => "8", Rank::Seven => "7",
        Rank::Six => "6", Rank::Five => "5", Rank::Four => "4", Rank::Three => "3",
        Rank::Two => "2",
    }
}

impl PokerApp {
    fn get_all_known_cards(&self) -> HashSet<MyCard> {
        let mut known = HashSet::new();
        // Hero
        for c in self.hero_hand.iter().flatten() { known.insert(*c); }
        // Villain
        for c in self.villain_hand.iter().flatten() { known.insert(*c); }
        // Friends
        for f in self.friends_hands.iter() {
            for c in f.iter().flatten() { known.insert(*c); }
        }
        // Board
        for c in self.board.iter().flatten() { known.insert(*c); }
        known
    }

    fn render_card_selector(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("SELECCIONAR CARTA").strong().size(16.0));
        ui.separator();
        
        let ranks = [
            Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, 
            Rank::Nine, Rank::Eight, Rank::Seven, Rank::Six, 
            Rank::Five, Rank::Four, Rank::Three, Rank::Two
        ];
        
        let suits = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

        egui::Grid::new("selector_grid").spacing([5.0, 5.0]).show(ui, |ui| {
            for suit in suits {
                for rank in ranks {
                    let card = MyCard { rank, suit };
                    let known = self.get_all_known_cards().contains(&card);
                    
                    // Si ya está usada, deshabilitamos el botón visualmente
                    let btn_text = egui::RichText::new(card.display_text())
                        .color(if known { egui::Color32::DARK_GRAY } else { card.color() })
                        .size(18.0); // Cartas más grandes en selector
                    
                    if ui.add_enabled(!known, egui::Button::new(btn_text).min_size(egui::vec2(30.0, 40.0))).clicked() {
                        if let Some(slot) = self.selected_card_idx {
                            match slot {
                                CardSlot::Hero(i) => self.hero_hand[i] = Some(card),
                                CardSlot::Villain(i) => self.villain_hand[i] = Some(card),
                                CardSlot::Friend(f, i) => self.friends_hands[f][i] = Some(card),
                                CardSlot::Board(i) => self.board[i] = Some(card),
                            }
                            // Auto-avance simple (opcional, por ahora no para evitar confusión)
                            self.selected_card_idx = None; 
                        }
                    }
                }
                ui.end_row();
            }
        });
        
        ui.add_space(10.0);
        if ui.button("🗑 Borrar Carta").clicked() {
             if let Some(slot) = self.selected_card_idx {
                match slot {
                    CardSlot::Hero(i) => self.hero_hand[i] = None,
                    CardSlot::Villain(i) => self.villain_hand[i] = None,
                    CardSlot::Friend(f, i) => self.friends_hands[f][i] = None,
                    CardSlot::Board(i) => self.board[i] = None,
                }
                self.selected_card_idx = None;
            }
        }
    }

    fn render_slot(&mut self, ui: &mut egui::Ui, card: Option<MyCard>, slot: CardSlot, label: &str) {
        let is_selected = self.selected_card_idx == Some(slot);
        
        let (text, color, bg) = if let Some(c) = card {
            (c.display_text(), c.color(), egui::Color32::from_gray(40))
        } else {
            ("?".to_string(), egui::Color32::GRAY, egui::Color32::from_gray(20))
        };

        let btn = egui::Button::new(
            egui::RichText::new(text)
                .size(20.0)
                .color(color)
                .strong()
        )
        .fill(if is_selected { egui::Color32::from_rgb(60, 60, 100) } else { bg })
        .min_size(egui::vec2(40.0, 60.0)); // Tamaño carta UI

        ui.vertical(|ui| {
            ui.label(egui::RichText::new(label).size(10.0));
            if ui.add(btn).clicked() {
                self.selected_card_idx = Some(slot);
            }
        });
    }
}

impl eframe::App for PokerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").min_width(300.0).show(ctx, |ui| {
            ui.heading("🎰 CONTROLES");
            ui.separator();

            // HERO
            ui.horizontal(|ui| {
                ui.label("HERO:");
                self.render_slot(ui, self.hero_hand[0], CardSlot::Hero(0), "C1");
                self.render_slot(ui, self.hero_hand[1], CardSlot::Hero(1), "C2");
            });
            ui.separator();

            // BOARD
            ui.label("MESA (Flop/Turn/River):");
            ui.horizontal(|ui| {
                for i in 0..5 {
                    let label = match i { 0..=2 => "Flop", 3 => "Turn", _ => "Riv" };
                    self.render_slot(ui, self.board[i], CardSlot::Board(i), label);
                }
            });
            ui.separator();

            // VILLANO (OPCIONAL)
            ui.collapsing("🦹 Villano (Opcional)", |ui| {
                ui.label("Si pones cartas aquí, el cálculo será 1vs1.");
                ui.horizontal(|ui| {
                    ui.label("Villano:");
                    self.render_slot(ui, self.villain_hand[0], CardSlot::Villain(0), "C1");
                    self.render_slot(ui, self.villain_hand[1], CardSlot::Villain(1), "C2");
                });
            });
            ui.separator();

            // ALIADOS
            ui.collapsing("👥 Aliados (Blockers)", |ui| {
                for i in 0..3 {
                    ui.horizontal(|ui| {
                        ui.label(format!("J{}:", i+2));
                        self.render_slot(ui, self.friends_hands[i][0], CardSlot::Friend(i, 0), "C1");
                        self.render_slot(ui, self.friends_hands[i][1], CardSlot::Friend(i, 1), "C2");
                    });
                }
            });

            ui.add_space(20.0);
            
            // SELECTOR (Si hay slot activo)
            if self.selected_card_idx.is_some() {
                self.render_card_selector(ui);
            } else {
                ui.label("Click en una carta para editarla.");
                if ui.button("❌ Limpiar Todo").clicked() {
                    *self = PokerApp::default();
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // --- 1. CÁLCULO DE ESTADÍSTICAS GLOBALES ---
            let known_cards = self.get_all_known_cards();
            let ranks = [
                Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Ten, 
                Rank::Nine, Rank::Eight, Rank::Seven, Rank::Six, 
                Rank::Five, Rank::Four, Rank::Three, Rank::Two
            ];
            
            let board_cards: Vec<Card> = self.board.iter().flatten().map(|c| c.to_poker_card()).collect();
            let hero_cards: Vec<Card> = self.hero_hand.iter().flatten().map(|c| c.to_poker_card()).collect();

            // Evaluar Hero
            let hero_score = if hero_cards.len() == 2 && board_cards.len() >= 3 {
                let mut all_hero = hero_cards.clone();
                all_hero.extend(board_cards.clone());
                if let Ok(rank) = self.evaluator.evaluate(&all_hero) {
                    Some(rank)
                } else { None }
            } else { None };

            // Chequear si hay un villano específico
            let villain_cards: Vec<Card> = self.villain_hand.iter().flatten().map(|c| c.to_poker_card()).collect();
            let is_1v1 = villain_cards.len() == 2;

            // Variables para contadores totales
            let mut total_possible_hands = 0;
            let mut total_winning = 0; // Manos que YO gano
            let mut total_losing = 0;  // Manos que ME ganan
            let mut total_ties = 0;

            // Bucle de cálculo
            let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
            
            if hero_score.is_some() {
                if is_1v1 {
                    // CÁLCULO 1vs1
                    let mut all_villain = villain_cards.clone();
                    all_villain.extend(board_cards.clone());
                    
                    if let Ok(v_score) = self.evaluator.evaluate(&all_villain) {
                         if let Some(h_s) = hero_score {
                            total_possible_hands = 1;
                            if v_score > h_s { total_losing = 1; }
                            else if v_score < h_s { total_winning = 1; }
                            else { total_ties = 1; }
                        }
                    }
                } else {
                    // CÁLCULO VS RANGO (DECK RESTANTE)
                    for r1 in ranks.iter() {
                        for r2 in ranks.iter() {
                            // Iterar palos para generar cartas específicas
                            for s1 in suits {
                                for s2 in suits {
                                    // Evitar duplicados y cartas imposibles (mismo slot)
                                    let c1 = MyCard { rank: *r1, suit: s1 };
                                    let c2 = MyCard { rank: *r2, suit: s2 };
                                    
                                    // Regla: C1 debe ser "mayor" o igual a C2 para no contar doble (AhKd es lo mismo que KdAh)
                                    // Usamos una comparación simple de bytes o orden
                                    if c1.to_poker_card() <= c2.to_poker_card() { continue; }

                                    // Si alguna carta ya es conocida (mia, amigos, mesa), esta mano es IMPOSIBLE
                                    if known_cards.contains(&c1) || known_cards.contains(&c2) { continue; }

                                    // Si llegamos acá, es una mano POSIBLE que puede tener un oponente
                                    total_possible_hands += 1;

                                    let mut op_hand = vec![c1.to_poker_card(), c2.to_poker_card()];
                                    op_hand.extend(board_cards.clone());

                                    if let Ok(op_score) = self.evaluator.evaluate(&op_hand) {
                                        if let Some(h_s) = hero_score {
                                            if op_score > h_s { total_losing += 1; } // Villano Mayor = Villano Gana
                                            else if op_score < h_s { total_winning += 1; } // Villano Menor = Villano Pierde
                                            else { total_ties += 1; }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // --- 2. DASHBOARD DE PROBABILIDADES ---
            if is_1v1 {
                 ui.heading("⚔️ DUELO 1 vs 1 (Hero vs Villano)");
            } else {
                 ui.heading("📊 TUS POSIBILIDADES (Cálculo Real)");
            }
            ui.add_space(5.0);
            
            if hero_score.is_some() && total_possible_hands > 0 {
                if is_1v1 {
                    if total_winning > 0 {
                        ui.label(egui::RichText::new("¡GANAS TU! 🏆").size(30.0).strong().color(egui::Color32::GREEN));
                    } else if total_losing > 0 {
                         ui.label(egui::RichText::new("¡PIERDES! 💀").size(30.0).strong().color(egui::Color32::RED));
                    } else {
                         ui.label(egui::RichText::new("¡EMPATE! 🤝").size(30.0).strong().color(egui::Color32::from_rgb(100, 100, 255)));
                    }
                } else {
                    let win_pct = (total_winning as f32 / total_possible_hands as f32) * 100.0;
                    let lose_pct = (total_losing as f32 / total_possible_hands as f32) * 100.0;
                    let tie_pct = (total_ties as f32 / total_possible_hands as f32) * 100.0;

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(format!("TIENES UN {:.1}% DE VICTORIA", win_pct)).size(20.0).strong().color(egui::Color32::GREEN));
                    });
                    
                    // Barra de progreso visual
                    let width = ui.available_width();
                    let height = 20.0;
                    let (rect, _resp) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
                    
                    if ui.is_rect_visible(rect) {
                        let painter = ui.painter();
                        // Fondo rojo (derrota)
                        painter.rect_filled(rect, 5.0, egui::Color32::from_rgb(200, 50, 50));
                        
                        // Parte Verde (victoria) + Azul (empate)
                        let safe_pct = win_pct + tie_pct;
                        let safe_width = (safe_pct / 100.0) * width;
                        let safe_rect = egui::Rect::from_min_size(rect.min, egui::vec2(safe_width, height));
                        painter.rect_filled(safe_rect, 5.0, egui::Color32::from_rgb(50, 180, 50));
                    }

                    ui.horizontal(|ui| {
                        ui.label(format!("Manos que te ganan: {}", total_losing));
                        ui.label("|");
                        ui.label(format!("Manos que ganas: {}", total_winning));
                        ui.label("|");
                        ui.label(format!("Empates: {}", total_ties));
                    });
                    ui.label("Esta barra muestra tu fuerza contra CUALQUIER mano aleatoria que pueda tener un rival.");
                }
            } else if hero_score.is_none() {
                 ui.label(egui::RichText::new("⚠️ FALTAN CARTAS PARA CALCULAR").color(egui::Color32::YELLOW));
            }

            ui.separator();

            // MATRIZ HEADER
            ui.horizontal(|ui| {
                ui.heading("DETALLE DE MANOS RIVALES");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("⬛ Imposible").color(egui::Color32::DARK_GRAY));
                    ui.label(egui::RichText::new("🟩 Ganas").color(egui::Color32::GREEN));
                    ui.label(egui::RichText::new("🟥 Pierdes").color(egui::Color32::RED));
                });
            });
            ui.separator();

            egui::Grid::new("poker_matrix").spacing([2.0, 2.0]).show(ui, |ui| {
                // Header (Ranks)
                ui.label(""); // Esquina vacía
                for r in ranks {
                    ui.label(egui::RichText::new(rank_to_str(r)).strong().size(16.0));
                }
                ui.end_row();

                for (r1_idx, r1) in ranks.iter().enumerate() {
                    // Row Header
                    ui.label(egui::RichText::new(rank_to_str(*r1)).strong().size(16.0));

                    for (r2_idx, r2) in ranks.iter().enumerate() {
                        let is_pair = r1_idx == r2_idx;
                        let is_suited = r1_idx < r2_idx; // Upper triangle
                        
                        // Nombre de la celda (ej: AKs, 77, QJo)
                        let (rank_a, rank_b) = if r1_idx <= r2_idx { (r1, r2) } else { (r2, r1) };
                        let suffix = if is_pair { "" } else if is_suited { "s" } else { "o" };
                        let cell_name = format!("{}{}{}", 
                            rank_to_str(*rank_a), 
                            rank_to_str(*rank_b), 
                            suffix
                        );

                        // --- LÓGICA DE COLOR Y ESTADO ---
                        let mut blocked_count = 0;
                        let mut winning_count = 0;
                        let mut losing_count = 0;
                        let mut tie_count = 0;
                        let mut total_combos = 0;

                        // Generar los combos reales para esta celda
                        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
                        for s1 in suits {
                            for s2 in suits {
                                // Filtrar combos inválidos para la celda
                                if is_pair && s1 == s2 { continue; } // AA no puede ser AhAh
                                if is_suited && s1 != s2 { continue; }
                                if !is_suited && !is_pair && s1 == s2 { continue; }
                                
                                // Para offsuit, evitar duplicados inversos. La matriz itera r1, r2.
                                // La convención visual es r1 filas, r2 columnas.
                                // Si r1 < r2 (suited area), la mano es r1-r2 suited.
                                // Si r1 > r2 (offsuit area), la mano es r1-r2 offsuit.
                                
                                // Validar que las cartas coincidan con la definición de la celda
                                let c1 = MyCard { rank: *r1, suit: s1 };
                                let c2 = MyCard { rank: *r2, suit: s2 };

                                // Chequear bloqueos (Hero, Aliados, Board)
                                if known_cards.contains(&c1) || known_cards.contains(&c2) {
                                    blocked_count += 1;
                                    total_combos += 1;
                                    continue;
                                }

                                total_combos += 1;

                                // Si llegamos aquí, el combo es posible para el villano
                                if let Some(h_score) = hero_score {
                                    let mut villain_hand = vec![c1.to_poker_card(), c2.to_poker_card()];
                                    villain_hand.extend(board_cards.clone());
                                    
                                    // Evaluar Villano
                                    if let Ok(v_score) = self.evaluator.evaluate(&villain_hand) {
                                        if v_score > h_score {
                                            losing_count += 1; // Villano Mayor -> Yo Pierdo
                                        } else if v_score < h_score {
                                            winning_count += 1; // Villano Menor -> Yo Gano
                                        } else {
                                            tie_count += 1;
                                        }
                                    }
                                }
                            }
                        }

                        // Determinar color de la celda
                        let is_fully_blocked = blocked_count == total_combos && total_combos > 0;
                        let user_excluded = self.excluded_cells.contains(&(r1_idx, r2_idx));
                        
                        let bg_color = if user_excluded {
                            egui::Color32::from_rgb(20, 20, 20) // Casi negro (desactivado manual)
                        } else if is_fully_blocked {
                            egui::Color32::from_rgb(40, 40, 40) // Gris oscuro (bloqueado por cartas)
                        } else if losing_count > 0 {
                             egui::Color32::from_rgb(200, 50, 50) // ROJO FUERTE: Peligro
                        } else if winning_count > 0 {
                             egui::Color32::from_rgb(50, 180, 50) // VERDE FUERTE: Ganas
                        } else if tie_count > 0 {
                             egui::Color32::from_rgb(50, 100, 200) // AZUL: Empate
                        } else {
                            // Estado neutro
                            if hero_score.is_some() {
                                // Si estamos evaluando y no hay ganador ni perdedor (y no está bloqueado totalmente),
                                // significa que no hay combos válidos para esa celda en específico (ej: 22 está en board).
                                egui::Color32::from_rgb(30, 30, 30)
                            } else {
                                // Pre-flop / Inactivo
                                if is_pair { egui::Color32::from_rgb(100, 80, 0) } // Marrón
                                else if is_suited { egui::Color32::from_rgb(0, 60, 60) } // Cyan
                                else { egui::Color32::from_rgb(50, 50, 50) } // Gris
                            }
                        };

                        let text_color = if is_fully_blocked || user_excluded {
                            egui::Color32::GRAY
                        } else {
                            egui::Color32::WHITE
                        };

                        let btn = egui::Button::new(egui::RichText::new(cell_name).size(12.0).color(text_color))
                            .fill(bg_color)
                            .min_size(egui::vec2(35.0, 35.0));

                        if ui.add(btn).clicked() {
                            if user_excluded {
                                self.excluded_cells.remove(&(r1_idx, r2_idx));
                            } else {
                                self.excluded_cells.insert((r1_idx, r2_idx));
                            }
                        }
                    }
                    ui.end_row();
                }
            });
            
            ui.add_space(15.0);
            ui.separator();
            ui.label(egui::RichText::new("📖 AYUDA MEMORIA DE COLORES").strong().size(14.0));
            
            egui::Grid::new("legend_grid").spacing([20.0, 5.0]).show(ui, |ui| {
                ui.label(egui::RichText::new("🟥 ROJO").color(egui::Color32::from_rgb(200, 50, 50)).strong());
                ui.label("Peligro: El villano tiene al menos una combinación en esta celda que te gana.");
                ui.end_row();

                ui.label(egui::RichText::new("🟩 VERDE").color(egui::Color32::from_rgb(50, 180, 50)).strong());
                ui.label("Seguro: Tu mano actual vence a todas las combinaciones posibles de esta celda.");
                ui.end_row();

                ui.label(egui::RichText::new("🟦 AZUL").color(egui::Color32::from_rgb(50, 100, 200)).strong());
                ui.label("Empate: Tu mano y la del villano tienen el mismo valor (Split Pot).");
                ui.end_row();

                ui.label(egui::RichText::new("⬛ GRIS OSCURO").color(egui::Color32::GRAY).strong());
                ui.label("Bloqueado: Esta mano es imposible porque las cartas están en tu mano, aliados o mesa.");
                ui.end_row();

                ui.label(egui::RichText::new("🔘 NEGRO").color(egui::Color32::BLACK).strong());
                ui.label("Excluido: Hiciste click para quitar esta mano del rango del villano manualmente.");
                ui.end_row();

                ui.label(egui::RichText::new("🟫 MARRÓN/CYAN").color(egui::Color32::from_rgb(100, 80, 0)).strong());
                ui.label("Pre-flop: Indica Pares (Marrón) y Suited (Cyan). Se activan los colores reales al poner el Flop.");
                ui.end_row();
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Poker Solver - Double or Nothing",
        options,
        Box::new(|_cc| Box::new(PokerApp::default())),
    )
}