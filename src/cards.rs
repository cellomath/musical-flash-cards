use js_sys::Generator;
use rand::prelude::SliceRandom;
use std::cell::RefCell;
use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, SvgsvgElement, Element};

use crate::SVG_NAMESPACE;

#[derive(Debug)]
pub struct Card {
    clef: Clef,
    notes: [Note; 3],
}

impl FromStr for Card {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut elements = src.split(':');
        macro_rules! parse_next {
            () => {
                elements.next().and_then(|s| s.parse().ok()).ok_or(())?
            };
        }
        Ok(Card {
            clef: parse_next!(),
            notes: [parse_next!(), parse_next!(), parse_next!()],
        })
    }
}

pub fn status_card(source: &str) -> Element{
    let document = web_sys::window().unwrap().document().unwrap();
    let card: Element = document
        .create_element("div").unwrap();
    card.set_attribute("width", "100%").unwrap();
    card.set_attribute("height", "100%").unwrap();
    card.set_attribute("style", "font-size: 24px; text-align: center;").unwrap();
    card.set_inner_html(source);
    card
}

impl From<&Card> for Element {
    fn from(card: &Card) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let card_svg: SvgsvgElement = document
            .create_element_ns(SVG_NAMESPACE, "svg")
            .unwrap()
            .dyn_into()
            .unwrap();
        card_svg.set_attribute("viewBox", "0 0 300 200").unwrap();
        card_svg.set_attribute("width", "100%").unwrap();
        card_svg.set_attribute("height", "100%").unwrap();

        let mut current_height = 80;
        for _ in 0..5 {
            let y = &current_height.to_string();
            let line = document.create_element_ns(SVG_NAMESPACE, "line").unwrap();
            line.set_attribute("x1", "0").unwrap();
            line.set_attribute("x2", "300").unwrap();
            line.set_attribute("y1", y).unwrap();
            line.set_attribute("y2", y).unwrap();
            line.set_attribute("style", "stroke: black; stroke-width: 1;")
                .unwrap();
            card_svg.append_child(&line).unwrap();
            current_height += 10;
        }

        let clef = document.create_element_ns(SVG_NAMESPACE, "image").unwrap();
        match card.clef {
            Clef::Treble => {
                clef.set_attribute("x", "10").unwrap();
                clef.set_attribute("y", "70").unwrap();
                clef.set_attribute("href", "/img/treble_clef.svg").unwrap();
            }
            Clef::Alto => {
                clef.set_attribute("x", "10").unwrap();
                clef.set_attribute("y", "80").unwrap();
                clef.set_attribute("href", "/img/tenor_clef.svg").unwrap();
            }
            Clef::Tenor => {
                clef.set_attribute("x", "10").unwrap();
                clef.set_attribute("y", "70").unwrap();
                clef.set_attribute("href", "/img/tenor_clef.svg").unwrap();
            }
            Clef::Bass => {
                clef.set_attribute("x", "10").unwrap();
                clef.set_attribute("y", "80").unwrap();
                clef.set_attribute("href", "/img/bass_clef.svg").unwrap();
            }
        }
        card_svg.append_child(&clef).unwrap();

        let center_note = card.clef.center_note();

        let mut center_x = 100;
        for note in card.notes {
            let staff_position = center_note.staff_distance(note);
            let center_y = 100 - (staff_position as i32 * 5);
            let notehead = document.create_element_ns(SVG_NAMESPACE, "path").unwrap();
            notehead
                .set_attribute(
                    "d",
                    &format!(
                        "M {},{} c 0,2 2,4 4,4 4,0 8,-3 8,-6 0,-3 -2,-4 -4,-4 -4,0 -8,3 -8,6",
                        center_x - 6,
                        center_y + 1
                    ),
                )
                .unwrap();
            notehead
                .set_attribute("style", "fill: #000000; fill-opacity: 1;")
                .unwrap();
            card_svg.append_child(&notehead).unwrap();
            if let Some(accidental) = note.accidental {
                let element = document.create_element_ns(SVG_NAMESPACE, "path").unwrap();
                use Accidental as A;
                element.set_attribute("d", &match accidental{
                    A::Sharp => format!("m {},{} -9,3 v -3 l 9,-3  m -3,-7 h 1 v 29 h -1  m -4,-28 h 1 v 30 h -1  m 7,-11 -9,3 v -3 l 9,-3", center_x-10, center_y-5),
                    A::Flat => format!("m {},{} c 0,-1 2,0 2,0 v 13 c 0,0 1,-1 3,-1 2,0 3,2 3,4 0,2 -4,8 -7,8 -1,0 -1,-24 -1,-24 z m 6,16 c 0,-2 -1,-2 -2,-2 -1,0 -2,2 -2,2 v 6 c 0,1 4,-3 4,-6 z",center_x-17, center_y-18),
                    A::DoubleSharp => format!("m {},{} c 0,-2 -0,-4 1,-4 2,0 3,-1 4,-2 -1,-1 -2,-2 -4,-2 -1,0 -1,-2 -1,-4 v 0 c 2,0 4,-0 4,1 0,2 1,3 2,4 1,-1 2,-2 2,-4 0,-1 2,-1 4,-1 0,2 0,4 -1,4 -2,0 -3,1 -4,2 1,1 2,2 4,2 1,0 1,2 1,4 -2,0 -4,0 -4,-1 0,-2 -1,-3 -2,-4 -1,1 -2,2 -2,4 0,1 -2,1 -4,1 z", center_x-21, center_y+6),
                    A::DoubleFlat => format!("m {},{} c 0,-2 -1,-2 -2,-2 -1,0 -2,2 -2,2 v 6 c 0,1 4,-3 4,-6 z m 0,-16 c 0,-1 2,0 2,0 v 13 c 0,0 1,-1 3,-1 2,0 3,2 3,4 0,2 -4,8 -7,8 0,0 -1,-0 -1,-4 0,0 -3,4 -5,4 -1,0 -1,-24 -1,-24 0,-1 2,0 2,0 v 13 c 0,0 1,-1 3,-1 l 1,-0 m 6,4 c 0,-2 -1,-2 -2,-2 -1,0 -2,2 -2,2 v 6 c 0,1 4,-3 4,-6 z", center_x-17, center_y-2),
                    A::Natural => {todo!()}
                }).unwrap();
                element
                    .set_attribute("style", "fill: #000000; fill-opacity: 1;")
                    .unwrap();
                card_svg.append_child(&element).unwrap();
            }
            let stem = document.create_element_ns(SVG_NAMESPACE, "line").unwrap();
            if staff_position > 0 {
                let l_str = &format!("{}.5", center_x - 6);
                stem.set_attribute("x1", l_str).unwrap();
                stem.set_attribute("x2", l_str).unwrap();
                stem.set_attribute("y1", &(center_y + 1).to_string())
                    .unwrap();
                stem.set_attribute("y2", &(center_y + 35).max(100).to_string())
                    .unwrap();
            } else {
                let r_str = &format!("{}.5", center_x + 5);
                stem.set_attribute("x1", r_str).unwrap();
                stem.set_attribute("x2", r_str).unwrap();
                stem.set_attribute("y1", &(center_y - 1).to_string())
                    .unwrap();
                stem.set_attribute("y2", &(center_y - 35).min(100).to_string())
                    .unwrap();
            }
            stem.set_attribute("style", "stroke: black; stroke-width: 1;")
                .unwrap();
            card_svg.append_child(&stem).unwrap();

            let mut prev_ledger_line = 80; // top line position
            let mut staff_position = staff_position;
            while staff_position >= 6 {
                staff_position -= 2;
                prev_ledger_line -= 10;
                let y_str = &prev_ledger_line.to_string();
                let ledger_line = document.create_element_ns(SVG_NAMESPACE, "line").unwrap();
                ledger_line
                    .set_attribute("x1", &(center_x - 10).to_string())
                    .unwrap();
                ledger_line
                    .set_attribute("x2", &(center_x + 10).to_string())
                    .unwrap();
                ledger_line.set_attribute("y1", y_str).unwrap();
                ledger_line.set_attribute("y2", y_str).unwrap();
                ledger_line
                    .set_attribute("style", "stroke: black; stroke-width: 1;")
                    .unwrap();
                card_svg.append_child(&ledger_line).unwrap();
            }

            let mut prev_ledger_line = 120; // bottom line position
            let mut staff_position = staff_position;
            while staff_position <= -6 {
                staff_position += 2;
                prev_ledger_line += 10;
                let y_str = &prev_ledger_line.to_string();
                let ledger_line = document.create_element_ns(SVG_NAMESPACE, "line").unwrap();
                ledger_line
                    .set_attribute("x1", &(center_x - 10).to_string())
                    .unwrap();
                ledger_line
                    .set_attribute("x2", &(center_x + 10).to_string())
                    .unwrap();
                ledger_line.set_attribute("y1", y_str).unwrap();
                ledger_line.set_attribute("y2", y_str).unwrap();
                ledger_line
                    .set_attribute("style", "stroke: black; stroke-width: 1;")
                    .unwrap();
                card_svg.append_child(&ledger_line).unwrap();
            }
            center_x += 70;
        }
        card_svg.dyn_into().unwrap()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CelloCardGenerator {
    finger_pattern_1_allowed: bool,
    finger_pattern_2_allowed: bool,
    finger_pattern_34_allowed: bool,
    finger_pattern_5_allowed: bool,
    bass_clef: Option<RangeInclusive<u8>>,
    tenor_clef: Option<RangeInclusive<u8>>,
    treble_clef: Option<RangeInclusive<u8>>,
    alto_clef: Option<RangeInclusive<u8>>,
    half_position_allowed: bool,
    position_1_allowed: bool,
    position_2_allowed: bool,
    position_3_allowed: bool,
    position_4_allowed: bool,
    position_5_allowed: bool,
    position_6_allowed: bool,
    position_7_allowed: bool,
    a_string: bool,
    d_string: bool,
    g_string: bool,
    c_string: bool,
    max_double_accidentals: u8,
    max_sharps: u8,
    max_flats: u8,
    shuffled_order: bool,
    string_count: u8,
}

impl CelloCardGenerator {
    pub const fn no_sharps_flats() -> Self {
        CelloCardGenerator {
            finger_pattern_1_allowed: true,
            finger_pattern_2_allowed: true,
            finger_pattern_34_allowed: true,
            finger_pattern_5_allowed: false,
            bass_clef: Some(
                Note {
                    letter: Letter::C,
                    octave: 2,
                    accidental: None,
                }
                .midi()..=Note {
                    letter: Letter::C,
                    octave: 5,
                    accidental: None,
                }
                .midi(),
            ),
            tenor_clef: None,
            treble_clef: None,
            alto_clef: None,
            half_position_allowed: true,
            position_1_allowed: true,
            position_2_allowed: true,
            position_3_allowed: true,
            position_4_allowed: true,
            position_5_allowed: true,
            position_6_allowed: true,
            position_7_allowed: true,
            a_string: true,
            d_string: true,
            g_string: true,
            c_string: true,
            max_double_accidentals: 0,
            max_sharps: 0,
            max_flats: 0,
            shuffled_order: false,
            string_count: 1,
        }
    }

    pub const fn one_flat() -> Self {
        let mut gen = CelloCardGenerator::no_sharps_flats();
        gen.max_flats = 1;
        gen
    }

    pub const fn one_sharp() -> Self {
        let mut gen = CelloCardGenerator::one_flat();
        gen.max_sharps = 1;
        gen
    }

    pub const fn two_flats() -> Self {
        let mut gen = CelloCardGenerator::one_sharp();
        gen.max_flats = 2;
        gen
    }

    pub const fn two_sharps() -> Self {
        let mut gen = CelloCardGenerator::two_flats();
        gen.max_sharps = 2;
        gen
    }

    pub const fn three_flats() -> Self {
        let mut gen = CelloCardGenerator::two_sharps();
        gen.max_flats = 3;
        gen
    }

    pub const fn three_sharps() -> Self {
        let mut gen = CelloCardGenerator::three_flats();
        gen.max_sharps = 3;
        gen
    }

    pub const fn tenor_clef_initial() -> Self {
        let mut gen = CelloCardGenerator::no_sharps_flats();
        gen.bass_clef = None;
        gen.tenor_clef = Some(
            Note{letter: Letter::C, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 5, accidental: None}.midi());
        gen
    }

    pub const fn tenor_clef_advanced() -> Self {
        let mut gen = CelloCardGenerator::three_sharps();
        gen.bass_clef = None;
        gen.tenor_clef = Some(
            Note{letter: Letter::C, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 5, accidental: None}.midi());
        gen
    }

    pub const fn treble_clef_initial() -> Self {
        let mut gen = CelloCardGenerator::no_sharps_flats();
        gen.bass_clef = None;
        gen.treble_clef = Some(
            Note{letter: Letter::G, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 5, accidental: None}.midi());
        gen
    }

    pub const fn treble_clef_advanced() -> Self {
        let mut gen = CelloCardGenerator::three_sharps();
        gen.bass_clef = None;
        gen.treble_clef = Some(
            Note{letter: Letter::G, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 5, accidental: None}.midi());
        gen
    }

    pub const fn advanced() -> Self {
        let mut gen = CelloCardGenerator::three_sharps();
        gen.tenor_clef = Some(
            Note{letter: Letter::G, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 3, accidental: None}.midi());
        gen.treble_clef = Some(
            Note{letter: Letter::G, octave: 3, accidental: None}.midi()..=
            Note{letter: Letter::C, octave: 5, accidental: None}.midi());
        gen.max_double_accidentals = 1;
        gen.shuffled_order = true;
        gen.string_count = 2;
        gen.finger_pattern_5_allowed = true;
        gen
    }

    pub const fn impossible() -> Self {
        CelloCardGenerator {
            finger_pattern_1_allowed: true,
            finger_pattern_2_allowed: true,
            finger_pattern_34_allowed: true,
            finger_pattern_5_allowed: true,
            bass_clef: Some(
                Note {
                    letter: Letter::C,
                    octave: 2,
                    accidental: None,
                }
                .midi()..=Note {
                    letter: Letter::C,
                    octave: 5,
                    accidental: None,
                }
                .midi(),
            ),
            tenor_clef: Some(
                Note {
                    letter: Letter::G,
                    octave: 3,
                    accidental: None,
                }
                .midi()..=Note {
                    letter: Letter::C,
                    octave: 5,
                    accidental: None,
                }
                .midi(),
            ),
            treble_clef: Some(
                Note {
                    letter: Letter::G,
                    octave: 3,
                    accidental: None,
                }
                .midi()..=Note {
                    letter: Letter::C,
                    octave: 6,
                    accidental: None,
                }
                .midi(),
            ),
            alto_clef: Some(
                Note {
                    letter: Letter::C,
                    octave: 3,
                    accidental: None,
                }
                .midi()..=Note {
                    letter: Letter::G,
                    octave: 4,
                    accidental: None,
                }
                .midi(),
            ),
            half_position_allowed: true,
            position_1_allowed: true,
            position_2_allowed: true,
            position_3_allowed: true,
            position_4_allowed: true,
            position_5_allowed: true,
            position_6_allowed: true,
            position_7_allowed: true,
            a_string: true,
            d_string: true,
            g_string: true,
            c_string: true,
            max_double_accidentals: 3,
            max_sharps: 3,
            max_flats: 3,
            shuffled_order: true,
            string_count: 4,
        }
    }

    pub fn read_settings() -> CelloCardGenerator {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = |element_id| {
            document
                .get_element_by_id(element_id)
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
        };
        let finger_pattern_1_allowed = element("finger_pattern_1_allowed").checked();
        let finger_pattern_2_allowed = element("finger_pattern_2_allowed").checked();
        let finger_pattern_34_allowed = element("finger_pattern_34_allowed").checked();
        let finger_pattern_5_allowed = element("finger_pattern_5_allowed").checked();

        macro_rules! range{
            ($id:literal) => {
                {
                    let min = element(concat!($id,"_clef_min")).value();
                    let max = element(concat!($id,"_clef_max")).value();
                    min.parse::<u8>().ok()
                    .or(min.parse::<Note>().ok().map(|n| n.midi()))
                    .map(|min|
                        max.parse::<u8>().ok()
                        .or(max.parse::<Note>().ok().map(|n| n.midi()))
                        .map(|max| min..=max)
                    ).flatten()
                }
            }
        }
        let bass_clef = range!("bass");
        let tenor_clef = range!("tenor");
        let treble_clef = range!("treble");
        let alto_clef = range!("alto");

        let half_position_allowed = element("half_position_allowed").checked();
        let position_1_allowed = element("position_1_allowed").checked();
        let position_2_allowed = element("position_2_allowed").checked();
        let position_3_allowed = element("position_3_allowed").checked();
        let position_4_allowed = element("position_4_allowed").checked();
        let position_5_allowed = element("position_5_allowed").checked();
        let position_6_allowed = element("position_6_allowed").checked();
        let position_7_allowed = element("position_7_allowed").checked();

        let a_string = element("a_string").checked();
        let d_string = element("d_string").checked();
        let g_string = element("g_string").checked();
        let c_string = element("c_string").checked();
        let max_double_accidentals = element("max_double_accidentals")
            .value()
            .parse()
            .unwrap_or(0);
        let max_sharps = element("max_sharps").value().parse().unwrap_or(0);
        let max_flats = element("max_flats").value().parse().unwrap_or(0);

        let shuffled_order = element("shuffled_order").checked();
        let string_count = element("string_count").value().parse().unwrap_or(0);

        CelloCardGenerator {
            finger_pattern_1_allowed,
            finger_pattern_2_allowed,
            finger_pattern_34_allowed,
            finger_pattern_5_allowed,
            bass_clef,
            tenor_clef,
            treble_clef,
            alto_clef,
            half_position_allowed,
            position_1_allowed,
            position_2_allowed,
            position_3_allowed,
            position_4_allowed,
            position_5_allowed,
            position_6_allowed,
            position_7_allowed,
            a_string,
            d_string,
            g_string,
            c_string,
            max_double_accidentals,
            max_sharps,
            max_flats,
            shuffled_order,
            string_count,
        }
    }

    pub fn write_settings(&self){
        let CelloCardGenerator {
            finger_pattern_1_allowed,
            finger_pattern_2_allowed,
            finger_pattern_34_allowed,
            finger_pattern_5_allowed,
            bass_clef,
            tenor_clef,
            treble_clef,
            alto_clef,
            half_position_allowed,
            position_1_allowed,
            position_2_allowed,
            position_3_allowed,
            position_4_allowed,
            position_5_allowed,
            position_6_allowed,
            position_7_allowed,
            a_string,
            d_string,
            g_string,
            c_string,
            max_double_accidentals,
            max_sharps,
            max_flats,
            shuffled_order,
            string_count } = self.clone();
        let document = web_sys::window().unwrap().document().unwrap();
        let element = |element_id| {
            document
                .get_element_by_id(element_id)
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap()
        };
        element("finger_pattern_1_allowed").set_checked(finger_pattern_1_allowed);
        element("finger_pattern_2_allowed").set_checked(finger_pattern_2_allowed);
        element("finger_pattern_34_allowed").set_checked(finger_pattern_34_allowed);
        element("finger_pattern_5_allowed").set_checked(finger_pattern_5_allowed);

        if let Some(range) = bass_clef {
            element("bass_clef_min").set_value(&Note::from_midi(*range.start()).to_string());
            element("bass_clef_max").set_value(&Note::from_midi(*range.end()).to_string());
            crate::update_bass_notes();
        } else {
            element("bass_clef_min").set_value("");
            element("bass_clef_max").set_value("");
        }
        if let Some(range) = tenor_clef {
            element("tenor_clef_min").set_value(&Note::from_midi(*range.start()).to_string());
            element("tenor_clef_max").set_value(&Note::from_midi(*range.end()).to_string());
            crate::update_tenor_notes();
        } else {
            element("tenor_clef_min").set_value("");
            element("tenor_clef_max").set_value("");
        }
        if let Some(range) = treble_clef {
            element("treble_clef_min").set_value(&Note::from_midi(*range.start()).to_string());
            element("treble_clef_max").set_value(&Note::from_midi(*range.end()).to_string());
            crate::update_treble_notes();
        } else {
            element("treble_clef_min").set_value("");
            element("treble_clef_max").set_value("");
        }
        if let Some(range) = alto_clef {
            element("alto_clef_min").set_value(&Note::from_midi(*range.start()).to_string());
            element("alto_clef_max").set_value(&Note::from_midi(*range.end()).to_string());
            crate::update_alto_notes();
        } else {
            element("alto_clef_min").set_value("");
            element("alto_clef_max").set_value("");
        }

        element("half_position_allowed").set_checked(half_position_allowed);
        element("position_1_allowed").set_checked(position_1_allowed);
        element("position_2_allowed").set_checked(position_2_allowed);
        element("position_3_allowed").set_checked(position_3_allowed);
        element("position_4_allowed").set_checked(position_4_allowed);
        element("position_5_allowed").set_checked(position_5_allowed);
        element("position_6_allowed").set_checked(position_6_allowed);
        element("position_7_allowed").set_checked(position_7_allowed);

        element("a_string").set_checked(a_string);
        element("d_string").set_checked(d_string);
        element("g_string").set_checked(g_string);
        element("c_string").set_checked(c_string);
        element("max_double_accidentals").set_value(&max_double_accidentals.to_string());
        element("max_sharps").set_value(&max_sharps.to_string());
        element("max_flats").set_value(&max_flats.to_string());

        element("shuffled_order").set_checked(shuffled_order);
        element("string_count").set_value(&string_count.to_string());
    }

    pub fn card_generator(&self) -> Box<[Card]> {
        let mut finger_patterns = Vec::with_capacity(4);
        if self.finger_pattern_1_allowed {
            finger_patterns.push([
                Interval::new(IntervalQuality::Minor, 2),
                Interval::new(IntervalQuality::Minor, 3),
            ]);
        }
        if self.finger_pattern_2_allowed {
            finger_patterns.push([
                Interval::new(IntervalQuality::Major, 2),
                Interval::new(IntervalQuality::Minor, 3),
            ]);
        }
        if self.finger_pattern_34_allowed {
            finger_patterns.push([
                Interval::new(IntervalQuality::Major, 2),
                Interval::new(IntervalQuality::Major, 3),
            ]);
        }
        if self.finger_pattern_5_allowed {
            finger_patterns.push([
                Interval::new(IntervalQuality::Augmented, 2),
                Interval::new(IntervalQuality::Major, 3),
            ]);
        }

        let mut clefs = Vec::with_capacity(4);
        if let Some(range) = &self.bass_clef {
            clefs.push((Clef::Bass, range.clone()));
        }
        if let Some(range) = &self.treble_clef {
            clefs.push((Clef::Treble, range.clone()));
        }
        if let Some(range) = &self.tenor_clef {
            clefs.push((Clef::Tenor, range.clone()));
        }
        if let Some(range) = &self.alto_clef {
            clefs.push((Clef::Alto, range.clone()));
        }

        let CelloCardGenerator {
            half_position_allowed,
            position_1_allowed,
            position_2_allowed,
            position_3_allowed,
            position_4_allowed,
            position_5_allowed,
            position_6_allowed,
            position_7_allowed,
            a_string,
            d_string,
            g_string,
            c_string,
            max_double_accidentals,
            max_sharps,
            max_flats,
            finger_pattern_1_allowed: _,
            finger_pattern_2_allowed: _,
            finger_pattern_34_allowed: _,
            finger_pattern_5_allowed: _,
            bass_clef: _,
            tenor_clef: _,
            treble_clef: _,
            alto_clef: _,
            shuffled_order,
            string_count
        } = self.clone();
        let rng = RefCell::new(thread_rng());
        let offsets = [Interval{interval: 1, quality: IntervalQuality::Perfect}, Interval{interval: 5, quality: IntervalQuality::Perfect}, Interval{interval: 9, quality: IntervalQuality::Major}, Interval{interval: 13, quality: IntervalQuality::Major}];
        (0..=255/*C#2 to A4*/)
            .flat_map(|start_note_midi| {
                let finger_patterns_ref = &finger_patterns;
                Note::from_midi(start_note_midi)
                    .all_enharmonic_equivalents()
                    .filter(|start_note| {
                        if a_string
                        /*A#3-A4: 58..=69*/
                        {
                            if match start_note {
                                Note {
                                    letter: Letter::A,
                                    octave: 3,
                                    accidental: Some(Accidental::Sharp),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::B,
                                    octave: 3,
                                    accidental: Some(Accidental::Flat),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::B,
                                    octave: 3,
                                    accidental: _,
                                } => position_1_allowed,
                                Note {
                                    letter: Letter::C,
                                    octave: 4,
                                    accidental: _,
                                } => position_2_allowed,
                                Note {
                                    letter: Letter::D,
                                    octave: 4,
                                    accidental: _,
                                } => position_3_allowed,
                                Note {
                                    letter: Letter::E,
                                    octave: 4,
                                    accidental: _,
                                } => position_4_allowed,
                                Note {
                                    letter: Letter::F,
                                    octave: 4,
                                    accidental: _,
                                } => position_5_allowed,
                                Note {
                                    letter: Letter::G,
                                    octave: 4,
                                    accidental: _,
                                } => position_6_allowed,
                                Note {
                                    letter: Letter::A,
                                    octave: 4,
                                    accidental: _,
                                } => position_7_allowed,
                                _ => false,
                            } {
                                return true;
                            }
                        }
                        if d_string {
                            if match start_note {
                                Note {
                                    letter: Letter::D,
                                    octave: 3,
                                    accidental: Some(Accidental::Sharp),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::E,
                                    octave: 3,
                                    accidental: Some(Accidental::Flat),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::E,
                                    octave: 3,
                                    accidental: _,
                                } => position_1_allowed,
                                Note {
                                    letter: Letter::F,
                                    octave: 3,
                                    accidental: _,
                                } => position_2_allowed,
                                Note {
                                    letter: Letter::G,
                                    octave: 3,
                                    accidental: _,
                                } => position_3_allowed,
                                Note {
                                    letter: Letter::A,
                                    octave: 3,
                                    accidental: _,
                                } => position_4_allowed,
                                Note {
                                    letter: Letter::B,
                                    octave: 3,
                                    accidental: _,
                                } => position_5_allowed,
                                Note {
                                    letter: Letter::C,
                                    octave: 4,
                                    accidental: _,
                                } => position_6_allowed,
                                Note {
                                    letter: Letter::D,
                                    octave: 4,
                                    accidental: _,
                                } => position_7_allowed,
                                _ => false,
                            } {
                                return true;
                            }
                        }
                        if g_string {
                            if match start_note {
                                Note {
                                    letter: Letter::G,
                                    octave: 2,
                                    accidental: Some(Accidental::Sharp),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::A,
                                    octave: 2,
                                    accidental: Some(Accidental::Flat),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::A,
                                    octave: 2,
                                    accidental: _,
                                } => position_1_allowed,
                                Note {
                                    letter: Letter::B,
                                    octave: 2,
                                    accidental: _,
                                } => position_2_allowed,
                                Note {
                                    letter: Letter::C,
                                    octave: 3,
                                    accidental: _,
                                } => position_3_allowed,
                                Note {
                                    letter: Letter::D,
                                    octave: 3,
                                    accidental: _,
                                } => position_4_allowed,
                                Note {
                                    letter: Letter::E,
                                    octave: 3,
                                    accidental: _,
                                } => position_5_allowed,
                                Note {
                                    letter: Letter::F,
                                    octave: 3,
                                    accidental: _,
                                } => position_6_allowed,
                                Note {
                                    letter: Letter::G,
                                    octave: 3,
                                    accidental: _,
                                } => position_7_allowed,
                                _ => false,
                            } {
                                return true;
                            }
                        }
                        if c_string {
                            if match start_note {
                                Note {
                                    letter: Letter::C,
                                    octave: 2,
                                    accidental: Some(Accidental::Sharp),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::D,
                                    octave: 2,
                                    accidental: Some(Accidental::Flat),
                                } => half_position_allowed,
                                Note {
                                    letter: Letter::D,
                                    octave: 2,
                                    accidental: _,
                                } => position_1_allowed,
                                Note {
                                    letter: Letter::E,
                                    octave: 2,
                                    accidental: _,
                                } => position_2_allowed,
                                Note {
                                    letter: Letter::F,
                                    octave: 2,
                                    accidental: _,
                                } => position_3_allowed,
                                Note {
                                    letter: Letter::G,
                                    octave: 2,
                                    accidental: _,
                                } => position_4_allowed,
                                Note {
                                    letter: Letter::A,
                                    octave: 2,
                                    accidental: _,
                                } => position_5_allowed,
                                Note {
                                    letter: Letter::B,
                                    octave: 2,
                                    accidental: _,
                                } => position_6_allowed,
                                Note {
                                    letter: Letter::C,
                                    octave: 3,
                                    accidental: _,
                                } => position_7_allowed,
                                _ => false,
                            } {
                                return true;
                            }
                        }
                        false
                    })
                    .flat_map(|start_note| {
                        let start_note: Note = start_note.clone();
                        finger_patterns_ref
                            .iter()
                            .filter_map(move |intervals| {
                                if let (Some(note2), Some(note3)) =
                                    (start_note + intervals[0], start_note + intervals[1])
                                {
                                    Some([start_note, note2, note3])
                                } else {
                                    None
                                }
                            })
                            .filter(|notes| {
                                let mut sharps = 0;
                                let mut flats = 0;
                                let mut double = 0;
                                for note in notes {
                                    match note.accidental {
                                        Some(Accidental::Flat) => {
                                            flats += 1;
                                        }
                                        Some(Accidental::DoubleFlat) => {
                                            double += 1;
                                            flats += 1;
                                        }
                                        Some(Accidental::DoubleSharp) => {
                                            double += 1;
                                            sharps += 1;
                                        }
                                        Some(Accidental::Sharp) => {
                                            sharps += 1;
                                        }
                                        None | Some(Accidental::Natural) => {}
                                    }
                                }
                                if sharps > max_sharps {
                                    return false;
                                }
                                if flats > max_flats {
                                    return false;
                                }
                                if double > max_double_accidentals {
                                    return false;
                                }
                                true
                            })
                    })
            })
            .flat_map(|notes| {
                let rng_ref = &rng;
                clefs.iter().filter_map(move |(clef, range)| {
                    let mut rng = rng_ref.borrow_mut();
                    let notes = match (shuffled_order, string_count) {
                        (false,0) => notes,
                        (true, _) => {
                            let mut notes = notes.map(|n: Note| n.add(offsets[rng.gen_range(0..string_count) as usize]).unwrap_or(n));
                            notes.shuffle(rng.deref_mut());
                            notes
                        },
                        (false,_) => {
                            let mut notes = notes.map(|n: Note| n.add(offsets[rng.gen_range(0..string_count) as usize]).unwrap_or(n));
                            notes.sort_by_key(|n| n.midi());
                            notes
                        }
                    };
                    if notes.iter().all(|note| range.contains(&note.midi())) {
                        return Some(Card {
                            clef: clef.clone(),
                            notes,
                        });
                    }
                    None
                })
            })
            .collect()
    }
}
#[cfg(test)]
#[test]
fn cello_card_generator() {
    CelloCardGenerator::no_sharps_flats()
        .card_generator()
        .into_iter()
        .for_each(|c| {
            println!("{c:?}");
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    C = 0,
    D = 1,
    E = 2,
    F = 3,
    G = 4,
    A = 5,
    B = 6,
}

impl From<Letter> for char {
    #[inline]
    fn from(letter: Letter) -> Self{
        use Letter as L;
        match letter {
            L::A => 'A',
            L::B => 'B',
            L::C => 'C',
            L::D => 'D',
            L::E => 'E',
            L::F => 'F',
            L::G => 'G'
        }
    }
}

impl TryFrom<char> for Letter {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Letter as L;
        Ok(match value {
            'A' => L::A,
            'B' => L::B,
            'C' => L::C,
            'D' => L::D,
            'E' => L::E,
            'F' => L::F,
            'G' => L::G,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Clef {
    Treble,
    Alto,
    Tenor,
    Bass,
}

impl FromStr for Clef {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Clef as C;
        Ok(match s {
            "B4" => C::Treble,
            "A3" => C::Tenor,
            "C4" => C::Alto,
            "D3" => C::Bass,
            _ => {
                return Err(());
            }
        })
    }
}

impl Clef {
    const fn center_note(self) -> Note {
        use Clef as C;
        use Letter as L;
        let accidental = None;
        match self {
            C::Treble => Note {
                letter: L::B,
                octave: 4,
                accidental,
            },
            C::Alto => Note {
                letter: L::C,
                octave: 4,
                accidental,
            },
            C::Tenor => Note {
                letter: L::A,
                octave: 3,
                accidental,
            },
            C::Bass => Note {
                letter: L::D,
                octave: 3,
                accidental,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Accidental {
    DoubleFlat = -2,
    Flat = -1,
    Natural = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl TryFrom<i8> for Accidental {
    type Error = ();
    #[inline]
    fn try_from(val: i8) -> Result<Accidental, Self::Error> {
        use Accidental as A;
        Ok(match val {
            -2 => A::DoubleFlat,
            -1 => A::Flat,
            0 => A::Natural,
            1 => A::Sharp,
            2 => A::DoubleSharp,
            _ => {
                return Err(());
            }
        })
    }
}

impl Accidental{
    fn to_str(self) -> &'static str{
        use Accidental as A;
        match self{
            A::DoubleFlat => "bb",
            A::Flat => "b",
            A::Natural => "",
            A::Sharp => "#",
            A::DoubleSharp => "##",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    letter: Letter,
    octave: u8,
    accidental: Option<Accidental>,
}

use std::str::FromStr;

#[allow(clippy::iter_nth_zero)]
impl FromStr for Note {
    type Err = ();
    fn from_str(src: &str) -> Result<Note, Self::Err> {
        let mut src = src.chars();
        let letter = src.next().ok_or(())?.try_into()?;
        let mut tmp = src.clone();
        let accidental = match tmp.next() {
            Some('#') => match tmp.next() {
                Some('#') => {
                    src.nth(1);
                    Some(Accidental::DoubleSharp)
                }
                _ => {
                    src.nth(0);
                    Some(Accidental::Sharp)
                }
            },
            Some('b') => match tmp.clone().next() {
                Some('b') => {
                    src.nth(1);
                    Some(Accidental::DoubleFlat)
                }
                _ => {
                    src.nth(0);
                    Some(Accidental::Flat)
                }
            },
            Some('n') => {
                src.nth(0);
                Some(Accidental::Natural)
            }
            _ => None,
        };
        tmp = src.clone();
        let mut octave = 0;
        while let Some(digit) = tmp.next().and_then(|d| d.to_digit(10)) {
            src.next();
            octave *= 10;
            octave += digit as u8;
        }
        Ok(Note {
            letter,
            octave,
            accidental,
        })
    }
}

impl ToString for Note {
    fn to_string(&self) -> String{
        let l: char = self.letter.into();
        if let Some(a) = self.accidental{
            format!("{}{}{}",l,a.to_str(),self.octave)
        }
        else {
            format!("{}{}",l,self.octave)
        }
    }
}


impl Note {
    /// How far up the staff do you have to go to get to the other note?
    const fn staff_distance(self, other: Note) -> i8 {
        let octave_distance = (other.octave as i8 - self.octave as i8) * 7;
        let letter_distance = other.letter as i8 - self.letter as i8;
        octave_distance + letter_distance
    }

    const fn midi(self) -> u8 {
        // note: c4 is 60
        use Letter as L;
        return (match self.letter {
            L::C => 0i8,
            L::D => 2,
            L::E => 4,
            L::F => 5,
            L::G => 7,
            L::A => 9,
            L::B => 11,
        }.wrapping_add(if let Some(accidental) = self.accidental {
            accidental as i8
        } else {
            0
        }) as u8).wrapping_add(12 * (self.octave.wrapping_add(1)));
    }

    pub fn from_midi(midi: u8) -> Note {
        let octave = midi / 12;
        let after_octave = midi % 12;
        let note_map = [0, 2, 4, 5, 7, 9, 11];
        let reverse_note_map = [L::C, L::D, L::E, L::F, L::G, L::A, L::B];
        let note = note_map.binary_search(&after_octave);
        use Letter as L;
        let (letter, accidental) = match note {
            Ok(val) => (reverse_note_map[val], None),
            Err(next_note) => (reverse_note_map[next_note - 1], {
                assert!(
                    after_octave - note_map[next_note - 1] == 1,
                    "should have been found in the note map"
                );
                Some(Accidental::Sharp)
            }),
        };
        Note {
            octave: octave.wrapping_sub(1),
            letter,
            accidental,
        }
    }

    fn enharmonic_equivalent(self, letter: Letter) -> Option<Note> {
        let letter_distance = letter as i8 - self.letter as i8;

        let octave = match letter_distance {
            5..=7 => self.octave.wrapping_sub(1),
            -7..=-5 => self.octave.wrapping_add(1),
            _ => self.octave,
        };
        let mut new_note = Note {
            octave,
            letter,
            accidental: None,
        };
        let adjustment = (new_note.midi()).wrapping_sub(self.midi()) as i8;
        match adjustment {
            -2 => {
                new_note.accidental = Some(Accidental::DoubleSharp);
                Some(new_note)
            }
            -1 => {
                new_note.accidental = Some(Accidental::Sharp);
                Some(new_note)
            }
            0 => Some(new_note),
            1 => {
                new_note.accidental = Some(Accidental::Flat);
                Some(new_note)
            }
            2 => {
                new_note.accidental = Some(Accidental::DoubleFlat);
                Some(new_note)
            }
            //there is no enharmonic equivalent for this letter.
            _ => {
                println!("{self:?} has no enharmonic_equivalent {letter:?} (adjustmnet {adjustment} needed)", );
                None
            },
        }
    }

    fn all_enharmonic_equivalents(self) -> impl Iterator<Item = Note> {
        use Letter::*;
        [C, D, E, F, G, A, B]
            .into_iter()
            .filter_map(move |letter| self.enharmonic_equivalent(letter))
    }
}

#[cfg(test)]
#[test]
fn enharmonic_tests() {
    assert_eq!(
        "Cbb4"
            .parse::<Note>()
            .unwrap()
            .enharmonic_equivalent(Letter::B)
            .unwrap(),
        "Bb3".parse().unwrap()
    );
}

use std::ops::{Add, RangeInclusive, DerefMut};
impl Add<Interval> for Note {
    type Output = Option<Note>;
    fn add(self, interval: Interval) -> Option<Note> {
        let other_midi = self.midi().wrapping_add(interval.midi_offset() as u8);
        let delta_letter = if interval.interval > 0 {
            interval.interval - 1
        } else {
            interval.interval + 1
        };
        use Letter as L;
        let new_letter = match (self.letter as i8 + delta_letter) % 7 {
            0 => L::C,
            1 => L::D,
            2 => L::E,
            3 => L::F,
            4 => L::G,
            5 => L::A,
            6 => L::B,
            _ => unreachable!("_%7 is in the range 0..=6"),
        };
        Note::from_midi(other_midi).enharmonic_equivalent(new_letter)
    }
}

#[cfg(test)]
#[test]
fn midi_notes() {
    assert_eq!("C4".parse::<Note>().unwrap().midi(), 60);
    assert_eq!("B##3".parse::<Note>().unwrap().midi(), 61);
    assert_eq!("Abb4".parse::<Note>().unwrap().midi(), 67);
}

#[derive(Debug, Clone, Copy)]
pub enum IntervalQuality {
    Major,
    Perfect,
    Minor,
    Diminished,
    Augmented,
}

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    quality: IntervalQuality,
    interval: i8,
}

impl Interval {
    pub fn new(quality: IntervalQuality, interval: i8) -> Self {
        match (interval % 7, quality) {
            (1 | 4 | 5, IntervalQuality::Major | IntervalQuality::Minor) => {
                panic!("{}'s cannot be major or minor", interval)
            }
            (2 | 3 | 6 | 0, IntervalQuality::Perfect) => {
                panic!("{}'s cannot be perfect", interval)
            }
            _ => Interval { quality, interval },
        }
    }

    pub fn midi_offset(self) -> i8 {
        use IntervalQuality as Q;
        let interval: u8 = self.interval.abs() as u8 - 1;
        let octaves = (interval / 7) as i8;
        let interval_index = (interval % 7) as usize;
        let major_perfect_offsets = [0i8, 2, 4, 5, 7, 9, 11];
        println!("{self:?}",);
        println!("octaves: {octaves:?}",);
        println!("interval_index: {interval_index:?}",);
        let offset = octaves * 12
            + match self.quality {
                Q::Major | Q::Perfect => major_perfect_offsets[interval_index],
                Q::Diminished => match interval_index {
                    0 | 3 | 4 => major_perfect_offsets[interval_index] - 1,
                    _ => major_perfect_offsets[interval_index] - 2,
                },
                Q::Minor => major_perfect_offsets[interval_index] - 1,
                Q::Augmented => major_perfect_offsets[interval_index] + 1,
            };
        if self.interval > 0 {
            offset as i8
        } else {
            -(offset as i8)
        }
    }
}

#[cfg(test)]
#[test]
fn interval_tests() {
    use Letter::*;
    use Accidental::*;
    let test_interval = Interval{interval: 1, quality: IntervalQuality::Perfect};
    for letter in [A, B, C, D, E, F, G] {
        for octave in 0..=8{
            let note = Note{ letter, octave, accidental: None};
            println!("{note:?}");
            assert_eq!(note.midi(), (note+ test_interval).unwrap().midi());
            for accidental in [DoubleFlat, DoubleSharp, Flat, Sharp, Natural]{
                let note = Note{ letter, octave, accidental: Some(accidental) };
                println!("{note:?}");
                assert_eq!(note.midi(), (note+ test_interval).unwrap().midi());
            }
        }
    }
    assert_eq!(Interval::new(IntervalQuality::Perfect, 8).midi_offset(), 12);
    assert_eq!(Interval::new(IntervalQuality::Major, 10).midi_offset(), 16);
    assert_eq!(
        Interval::new(IntervalQuality::Diminished, 7).midi_offset(),
        9
    );
    assert_eq!(Interval::new(IntervalQuality::Perfect, 1).midi_offset(), 0);
    assert_eq!(Interval::new(IntervalQuality::Major, -3).midi_offset(), -4);
}
