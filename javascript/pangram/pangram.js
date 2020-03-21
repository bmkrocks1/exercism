//
// This is only a SKELETON file for the 'Pangram' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

// The total letters of the alphabet
const ALPHABET_COUNT = 26;

export const isPangram = str =>
  new Set(str.toLowerCase().replace(/[^a-z]/g, '')).size === ALPHABET_COUNT
