#![allow(dead_code)]

extern crate rand;

use std::collections::HashMap;
use std::fmt;
use self::rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Sentence(Vec<String>);
pub type Prefix = (String, String);
#[derive(Debug)]
pub struct Database(HashMap<Prefix, Vec<String>>);

pub enum MarkovError {
  InsufficientLength,
  NoCompletion(Prefix)
}

impl fmt::Debug for MarkovError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      MarkovError::InsufficientLength => {
        write!(f, "Insufficient sentence length!")
      },

      MarkovError::NoCompletion(ref p) => {
        write!(f, "No completion available for (\"{}\", \"{}\")!", p.0, p.1)
      }
    }
  }
}

impl From<String> for Sentence {
  /// Transform a String first into a sequence of words, and then
  /// the Sentence type, by way of splitting on whitespace and
  /// and collecting as a Vec.
  fn from(sentence: String) -> Sentence {
    let words = sentence
      .split_whitespace()
      .map(|e| String::from(e))
      .collect::<Vec<String>>();

    Sentence(words)
  }
}

impl<'a> From<&'a str> for Sentence {
  /// Transform a &str into a sequence of words,
  /// and then the Sentence type.
  fn from(sentence: &'a str) -> Sentence {
    let words = sentence
      .split_whitespace()
      .map(|e| String::from(e))
      .collect::<Vec<String>>();

    Sentence(words)
  }
}

impl Clone for Sentence {
  fn clone(&self) -> Sentence {
    // Yes I'm meming in source code now.
    let ref insine = self.0;
    Sentence(insine.clone())
  }
}

impl Database {
  pub fn new() -> Database {
    Database(HashMap::<Prefix, Vec<String>>::new())
  }

  /// Take a Sentence and add it to the database for text generation.
  pub fn parse(&mut self, sen: Sentence) -> Result<(), MarkovError> {
    let Sentence(ref raw) = sen;

    // Be sure there's enough elements.
    if raw.len() < 3 {
      return Err(MarkovError::InsufficientLength)
    }
    
    // This sucks but I'm not sure there's a nicer way to do it.
    for ind in 1 .. (raw.len() - 1) {
      let prefix = (raw[ind - 1].clone(), raw[ind].clone());
      let next = raw[ind + 1].clone();

      if !self.0.contains_key(&prefix) {
        self.0.insert(prefix, vec![next]);
      }

      else {
        let mut existing = self.0.get_mut(&prefix).unwrap();
        existing.push(next);
      }
    }

    Ok(())
  }

  /// Get the completions for a Prefix.
  pub fn complete(&self, prefix: &Prefix) -> Option<&Vec<String>> {
    if let Some(value) = self.0.get(&prefix) {
      Some(value)
    }

    else {
      None
    }
  }

  /// Get a random completion for a Prefix.
  pub fn rand_complete(&self, prefix: &Prefix) -> Option<&String> {
    if let Some(completions) = self.0.get(&prefix) {
      thread_rng().choose(&completions)
    }

    else {
      None
    }
  }

  /// Generate some markov text starting from a Prefix.
  pub fn generate(&self, prefix: &Prefix, n: usize) -> Result<String, MarkovError> {
    let mut output = String::new();
    let mut generator: Prefix = prefix.clone();

    output.push_str(&*prefix.0);
    output.push(' ');
    output.push_str(&*prefix.1);
    
    for _ in 0..n {
      if let Some(word) = self.rand_complete(&generator) {
        output.push(' ');
        output.push_str(&*word);
        generator = (generator.1, word.clone())
      }

      else {
        if output.len() == 0 {
          return Err(MarkovError::NoCompletion(generator.clone()))
        }
      }
    }
    
    Ok(output)
  }
}
