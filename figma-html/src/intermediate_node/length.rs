use std::{borrow::Cow, fmt, ops::Add};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Hash, Eq)]
pub enum LengthUnit {
    Px,
    Percentage,
}

impl fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthUnit::Px => write!(f, "px"),
            LengthUnit::Percentage => write!(f, "%"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Length<'a> {
    Zero,
    Value {
        unit: LengthUnit,
        value: f64,
    },
    Var {
        name: Cow<'a, str>,
        containing_block_relative: Option<bool>,
        multiplier: f64,
    },
    Addition {
        terms: Vec<Length<'a>>,
    },
}

impl<'a> Length<'a> {
    pub fn new_from_option_pixels(pixels: Option<f64>) -> Self {
        match pixels {
            None => Self::Zero,
            Some(p) if p == 0.0 => Self::Zero,
            Some(p) => Self::Value {
                unit: LengthUnit::Px,
                value: p,
            },
        }
    }

    /**
     * Whether the length is relative to the
     * [containing-block's](https://developer.mozilla.org/en-US/docs/Web/CSS/Containing_block)
     * size. In practice this is true if the length involves a percentage.
     * Strictly speaking CSS variables might be given a value that
     * is containing-block-relative. When using a length variable you
     * can declare it as `None` to declare you don't know.
     */
    pub fn containing_block_relative(&self) -> Option<bool> {
        match self {
            Self::Zero => Some(false),
            Self::Value {
                unit: LengthUnit::Px,
                ..
            } => Some(false),
            Self::Value {
                unit: LengthUnit::Percentage,
                ..
            } => Some(true),
            Self::Var {
                containing_block_relative,
                ..
            } => *containing_block_relative,
            Self::Addition { terms } => {
                let mut r = Some(false);
                for term in terms.iter() {
                    match term.containing_block_relative() {
                        Some(true) => return Some(true),
                        None => r = None,
                        Some(false) => {}
                    }
                }
                r
            }
        }
    }

    fn requires_calc(&self) -> bool {
        match self {
            Self::Zero | Self::Value { .. } => false,
            Self::Addition { .. } => true,
            Self::Var { multiplier, .. } => *multiplier != 1.0,
        }
    }

    pub fn normalise(self) -> Self {
        match self {
            Self::Zero => self,
            Self::Value { value, .. } => {
                if value == 0.0 {
                    Self::Zero
                } else {
                    self
                }
            }
            Self::Var { .. } => self,
            Self::Addition { terms } => {
                struct Scope<'a> {
                    values: IndexMap<LengthUnit, f64>,
                    variables: IndexMap<Cow<'a, str>, (f64, Option<bool>)>,
                }
                let mut new = Scope {
                    values: IndexMap::new(),
                    variables: IndexMap::new(),
                };
                fn add_value<'a>(new: &mut Scope<'a>, unit: LengthUnit, value: f64) {
                    *new.values.entry(unit).or_insert(0.0) += value;
                }
                fn add_var<'a>(
                    new: &mut Scope<'a>,
                    name: Cow<'a, str>,
                    multiplier: f64,
                    containing_block_relative: Option<bool>,
                ) {
                    let entry = new.variables.entry(name).or_insert((0.0, Some(false)));
                    entry.0 += multiplier;
                    entry.1 = match (entry.1, containing_block_relative) {
                        (Some(true), _) | (_, Some(true)) => Some(true),
                        (None, _) | (_, None) => None,
                        _ => Some(false),
                    }
                }
                fn add_terms<'a>(new: &mut Scope<'a>, terms: Vec<Length<'a>>) {
                    for term in terms.into_iter().map(Length::normalise) {
                        match term {
                            Length::Zero => {}
                            Length::Value { unit, value } => add_value(new, unit, value),
                            Length::Var {
                                name,
                                containing_block_relative,
                                multiplier,
                            } => add_var(new, name, multiplier, containing_block_relative),
                            Length::Addition { terms } => add_terms(new, terms),
                        }
                    }
                }
                add_terms(&mut new, terms);
                if new.values.is_empty() && new.variables.is_empty() {
                    Self::Zero
                } else if new.values.is_empty() && new.variables.len() == 1 {
                    let (name, (multiplier, containing_block_relative)) =
                        new.variables.pop().unwrap();
                    Self::Var {
                        name,
                        containing_block_relative,
                        multiplier,
                    }
                } else if new.values.len() == 1 && new.variables.is_empty() {
                    let (&unit, &value) = new.values.first().unwrap();
                    Self::Value { unit, value }
                } else {
                    let mut terms = vec![];
                    for (name, (multiplier, containing_block_relative)) in new.variables.into_iter()
                    {
                        terms.push(Self::Var {
                            name,
                            containing_block_relative,
                            multiplier,
                        });
                    }
                    for (unit, value) in new.values.into_iter() {
                        terms.push(Self::Value { unit, value })
                    }
                    Self::Addition { terms }
                }
            }
        }
    }
}

impl<'a> fmt::Display for Length<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let requires_calc = self.requires_calc();
        if requires_calc {
            write!(f, "calc(")?;
        }
        match self {
            Self::Zero => {
                write!(f, "0")?;
            }
            Self::Value { unit, value } => {
                write!(f, "{value}{unit}")?;
            }
            Self::Var {
                name, multiplier, ..
            } => {
                if multiplier != &1.0 {
                    write!(f, "{multiplier}*")?;
                }
                write!(f, "var(--{name}")?;
            }
            Self::Addition { terms } => {
                for (i, term) in terms.iter().enumerate() {
                    if i != 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{term}")?;
                }
            }
        };
        if requires_calc {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl<'a> Add for Length<'a> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Length::Addition {
            terms: vec![self, other],
        }
        .normalise()
    }
}
